use std::error::Error;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::str::FromStr;
use cidr::IpCidr;
use unqlite::{Cursor, KV, Transaction, UnQLite};
use crate::model::{data_operations, factory, locking_operations, Selection, SelectionOperation};
use crate::error::{DBSaveError, ScopeInitError};
use crate::interpolate::{factory as faktory, ProtoScope};
use crate::scope::{Scope, ScopeDescription};

#[derive(serde::Serialize, serde::Deserialize, Copy, Clone)]
pub struct SchemaDescription {
    pub prefix_length: u8,
    pub allocation_prefix_length: u8,
    pub locked: bool
}

#[derive(serde::Serialize, serde::Deserialize, Copy, Clone)]
pub struct Schema {
    pub pool: u128,
    pub descriptions: [Option<SchemaDescription>; 2],
    pub parent: Option<u128>,

}

impl data_operations<Schema, SchemaDescription> for Schema {
    fn begin_tx(db: &mut UnQLite) -> Result<(), unqlite::Error> {
        match db.begin() {
            Ok(_) => {
                Ok(())
            }
            Err(e) => Err(e)
        }
    }

    fn roll_back_tx(db: &mut UnQLite) -> Result<(), unqlite::Error> {
        match db.rollback() {
            Ok(_) => {
                Ok(())
            }
            Err(e) => Err(e)
        }
    }

    fn commit(db: &mut UnQLite) -> Result<(), unqlite::Error> {
        match db.commit() {
            Ok(_) => {                
                Ok(())
            }
            Err(e) => Err(e)
        }
    }

    fn initialize_db(db: &mut UnQLite) -> Result<(), Box<dyn Error>> {
        match Schema::is_db_initialized(db)? {
            true => {
                Ok(())
            }
            false => {
                let mut s = Schema::new_from_string(
                    "100.64.0.0".to_string(),
                    17,
                    None)?;

                match s.actual.to_proto_scope()?
                    .children(20)
                    .map(|f| -> Result<Selection<Schema>, Box<dyn Error>> {
                        let mut child = Schema::new_from_proto_scope(
                            f,
                            Some(&mut s))
                            .unwrap();

                         Ok(match child.operation {
                            SelectionOperation::UPDATE_PARENT_DESCRIPTIONS => {
                                s.actual.descriptions[1] = child.actual.descriptions[0];
                                Schema::save(&mut s, db)?;
                                child
                            }
                            SelectionOperation::DEFAULT => {
                                Schema::save(&mut child, db)?;
                                child
                            }
                        })
                    })
                    .any(|selection| -> bool {
                        selection.is_err()
                    }) {
                    true => {
                        Err(DBSaveError.into())
                    }
                    false => {
                        Ok(())
                    }
                }
            }
        }
    }

    fn dao() -> Result<UnQLite, Box<dyn Error>> {
        match std::env::var("SCHEMA_DB_FILE") {
            Ok(_) => {
                if std::env::var("SCHEMA_DB_FILE")? == "" {                    
                    let file = tempfile::NamedTempFile::new().unwrap().into_temp_path();
                    Ok(UnQLite::create(file.to_str().unwrap()))
                }
                else {
                    Ok(UnQLite::create(std::env::var("SCHEMA_DB_FILE")?))
                }
            }
            Err(e) => Err(Box::new(e))
        }        
    }

    fn save(s: &mut Selection<Schema>, db: &mut UnQLite) -> Result<(), Box<dyn Error>> {
        s.saved = true;

        match db.kv_store(
            s.actual.pool.to_be_bytes(),
            serde_json::to_string(&mut s.actual)?.as_bytes()) {
            Ok(_) => {
                Ok(())
            }
            Err(_) => {
                todo!()
            }
        }
    }

    fn exists_in_database(_id: u128) -> Result<bool, Box<dyn Error>> {
        todo!()
    }

    fn retrieve_all() -> Result<Vec<Selection<Schema>>, Box<dyn Error>> {
        let db = Schema::dao().unwrap();
        let mut entry = db.first();
        let mut ret: Vec<Selection<Schema>> = Vec::new();

        loop {
            if entry.is_none() {
               break;
            }
            else {
                let record = entry.expect("valid entry");
                let (_key, mut value) = record.key_value();

                let v = String::from_utf8_lossy(value.as_mut_slice()).to_string();
                let selection = Schema::new_from_json(v)?;

                ret.push(selection);

                entry = record.next();
            }
        }

        Ok(ret)
    }

    fn allocate_pool(_tags: Vec<String>) -> Result<Selection<Schema>, Box<dyn Error>> {
        todo!("not implemented for schema")
    }

    fn allocate_address(_network: String) -> Result<Selection<Schema>, Box<dyn Error>> {
        todo!("not implemented for schema")
    }

    fn release_pool(_network: String) -> Result<(), Box<dyn Error>> {
        todo!("not implemented for schema")
    }

    fn release_address(_network: String) -> Result<(), Box<dyn Error>> {
        todo!("not implemented for schema")
    }

    fn is_db_initialized(db: &mut UnQLite) -> Result<bool, Box<dyn Error>> {
        match db.first() {
            None => {
                Ok(false)
            }
            Some(_) => {
                Ok(true)
            }
        }
    }
}

impl factory<Schema, SchemaDescription, Scope, ScopeDescription> for Schema {
    fn new_from_string(network: String, prefix_length: u8, parent: Option<&mut Selection<Schema>>) -> Result<Selection<Schema>, Box<dyn Error>> {
        let net = string_to_ip_cidr(network, prefix_length)?;

        let parent_id = match parent {
            None => {
                None
            }
            Some(p) => {
                Some(p.actual.pool)
            }
        };

        let update_parent = parent_id.is_some() && string_to_u128_id(net.first_address().to_string())? == parent_id.unwrap();

        Ok(Selection {
            actual: Schema {
                pool: string_to_u128_id(net.first_address().to_string())?,
                descriptions: [Some(SchemaDescription {
                    prefix_length,
                    allocation_prefix_length: net.network_length(),
                    locked: false
                }), None],
                parent: parent_id
            },
            selected_prefix_length: Some(prefix_length),
            saved: false,
            operation: if update_parent {
                SelectionOperation::UPDATE_PARENT_DESCRIPTIONS
            } else {
                SelectionOperation::DEFAULT
            }
        })
    }

    fn new_from_bytes(_network: Vec<u8>, _prefix_length: u8, _parent: Option<&mut Selection<Schema>>) -> Result<Selection<Schema>, Box<dyn Error>> {
        todo!()
    }

    fn new_from_proto_scope(_network: ProtoScope<IpCidr>, _parent: Option<&mut Selection<Schema>>) -> Result<Selection<Schema>, Box<dyn Error>> {
        todo!()
    }

    fn new_from_selection(_network: Selection<Scope>) -> Result<Selection<Schema>, Box<dyn Error>> {
        todo!()
    }

    fn new_from_json(mut json: std::string::String) -> Result<Selection<Schema>, Box<dyn Error>> {
        Ok(Selection {
            actual: serde_json::from_str(&mut json)?,
            selected_prefix_length: Option::None,
            saved: false,
            operation: SelectionOperation::DEFAULT,
        })
    }

    fn to_proto_scope(&self) -> Result<ProtoScope<IpCidr>, Box<dyn Error>> {
       match self.pool > u32::MAX.into() {
            true => {
                ProtoScope::new_type_backed_proto_scope(
                    IpCidr::new(
                        Ipv6Addr::from(self.pool).into(),
                        self.descriptions[0].unwrap().prefix_length)?)
            },
            false => {
                ProtoScope::new_type_backed_proto_scope(
                    IpCidr::new(
                        Ipv4Addr::from(self.pool as u32).into(),
                        self.descriptions[0].unwrap().prefix_length)?)
            },
       }
    }

    fn new_selection(&self) -> Result<Selection<Schema>, Box<dyn Error>> {
        todo!()
    }
}

impl locking_operations for Selection<Schema> {
    fn lock(&self) -> Result<bool, Box<dyn Error>> {

        todo!()
    }

    fn unlock(&self) -> Result<bool, Box<dyn Error>> {
        todo!()
    }

    fn is_locked(&self) -> Result<bool, Box<dyn Error>> {
        todo!()
    }
}

impl locking_operations for SchemaDescription {
    fn lock(&self) -> Result<bool, Box<dyn Error>> {
        todo!()
    }

    fn unlock(&self) -> Result<bool, Box<dyn Error>> {
        todo!()
    }

    fn is_locked(&self) -> Result<bool, Box<dyn Error>> {
        todo!()
    }
}

fn string_to_ip_cidr(network: String, prefix_length: u8) -> Result<IpCidr, Box<dyn Error>> {
    Ok(IpCidr::new(IpAddr::from_str(network.as_str())?, prefix_length)?)
}

fn string_to_u128_id(network: String) -> Result<u128, Box<dyn Error>> {
    Ok(match IpAddr::from_str(network.as_str())? {
        IpAddr::V4(a) => {
            let v: u32 = a.into();
            v.into()
        }
        IpAddr::V6(a) => {
            let v: u128 = a.into();
            v
        }
    })
}

pub(crate) fn create_initial_scopes(scope_db: &mut UnQLite, _schema_db: &mut UnQLite) -> Result<(), Box<dyn Error>> {
    if Schema::retrieve_all()?
        .into_iter()
        .map(|f| -> Result<Selection<Scope>, Box<dyn Error>> {
            Scope::new_from_selection(f)
        })
        .map(|f| -> Result<(), Box<dyn Error>> {
            Scope::save(&mut f?, scope_db)
        })
        .any(|f| -> bool {
            f.is_err()
        }) {
            Err(ScopeInitError.into())
        }
        else {
            Ok(())
        }
}

#[cfg(test)]
mod tests {
    use log::warn;

    use crate::schema::*;    
    
    #[test]
    fn new_selection_from_string() {
        assert_eq!(Schema::new_from_string("100.64.0.0".to_string(),17,None).unwrap().selected_prefix_length.unwrap(), 17);
        assert_eq!(Schema::new_from_string("100.64.0.0".to_string(),17,None).unwrap().is_locked().unwrap(), false);
    }

    #[test]
    fn selection_to_proto_scope() {
        let pro = Schema::new_from_string("100.64.0.0".to_string(),17,None).unwrap().actual.to_proto_scope().unwrap();
        assert_eq!(pro.children(20).count(), 16);
    }

    #[test]
    fn no_env_file_set() {
        match std::env::var("SCHEMA_DB_FILE")
        {
            Ok(_) => {
                if std::env::var("SCHEMA_DB_FILE").unwrap() != "" {
                    panic!("don't set database SCHEMA_DB_FILE for test")
                }
                else {
                    std::env::set_var("SCHEMA_DB_FILE", "");
                }
            }
            Err(_) => {
                std::env::set_var("SCHEMA_DB_FILE", "");
            }
        }
    }

    #[test]
    fn test_schema_dao_with_env() {                
        let result = Schema::dao();
        assert!(result.is_ok())
    }
    
    #[test]
    fn test_roll_back_tx() {        
        let mut dao = Schema::dao().unwrap();

        match Schema::begin_tx(&mut dao) {
            Ok(_) => {
                match Schema::initialize_db(&mut dao) {
                    Ok(_) => {
                        match Schema::roll_back_tx(&mut dao) {
                            Ok(_) => (),
                            Err(_) => panic!("rollback of init failed"),
                        }
                    }
                    Err(_) => panic!("failed to initialize db"),
                }
            }
            Err(_) => panic!("failed to begin tx"),
        }
    }

    #[test]
    fn test_complete_transaction() {                
        let mut dao = Schema::dao().unwrap();

        match Schema::begin_tx(&mut dao) {
            Ok(_) => {
                match Schema::initialize_db(&mut dao) {
                    Ok(_) => {
                        match Schema::commit(&mut dao) {
                            Ok(_) => (),
                            Err(_) => panic!("failed to commit")
                        }
                    },
                    Err(_) => panic!("initialization failed")
                }
            },
            Err(e) => panic!("failed to begin transaction: {} ", e)
        }
    }

    #[test]
    fn test_begin_tx_within_tx() {        
        let mut dao = Schema::dao().unwrap();

        match Schema::begin_tx(&mut dao) {
            Ok(_) => {
                match Schema::begin_tx(&mut dao) {
                    Ok(_) => warn!("is this bad or good"),
                    Err(_) => panic!("unknown behavior")
                }
            }
            Err(e) => panic!("failed to begin transaction: {} ", e)
        }
    }

    #[test]
    fn test_commit_no_tx() {        
        let mut dao = Schema::dao().unwrap();

        match Schema::commit(&mut dao) {
            Ok(_) => warn!("end tx suceeeded but no transaction existed"),
            Err(_) => (),
        }
    }
}