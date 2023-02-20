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
            Err(_) => {
                todo!()
            }
        }
    }

    fn roll_back_tx(db: &mut UnQLite) -> Result<(), unqlite::Error> {
        match db.rollback() {
            Ok(_) => {
                Ok(())
            }
            Err(_) => {
                todo!()
            }
        }
    }

    fn commit(db: &mut UnQLite) -> Result<(), unqlite::Error> {
        match db.commit() {
            Ok(_) => {
                Ok(())
            }
            Err(_) => {
                todo!()
            }
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
                        return Err(DBSaveError.into())
                    }
                    false => {
                        Ok(())
                    }
                }
            }
        }
    }

    fn dao() -> Result<UnQLite, Box<dyn Error>> {
        Ok(UnQLite::create(std::env::var("SCHEMA_DB_FILE")?))
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

    fn exists_in_database(id: u128) -> Result<bool, Box<dyn Error>> {
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
                let (key, mut value) = record.key_value();

                let v = String::from_utf8_lossy(value.as_mut_slice()).to_string();
                let selection = Schema::new_from_json(v)?;

                ret.push(selection);

                entry = record.next();
            }
        }

        Ok(ret)
    }

    fn allocate_pool(tags: Vec<String>) -> Result<Selection<Schema>, Box<dyn Error>> {
        todo!("not implemented for schema")
    }

    fn allocate_address(network: String) -> Result<Selection<Schema>, Box<dyn Error>> {
        todo!("not implemented for schema")
    }

    fn release_pool(network: String) -> Result<(), Box<dyn Error>> {
        todo!("not implemented for schema")
    }

    fn release_address(network: String) -> Result<(), Box<dyn Error>> {
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

        let update_parent = parent_id != None && string_to_u128_id(net.first_address().to_string())? == parent_id.unwrap();

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

    fn new_from_bytes(network: Vec<u8>, prefix_length: u8, parent: Option<&mut Selection<Schema>>) -> Result<Selection<Schema>, Box<dyn Error>> {
        todo!()
    }

    fn new_from_proto_scope(network: ProtoScope<IpCidr>, parent: Option<&mut Selection<Schema>>) -> Result<Selection<Schema>, Box<dyn Error>> {
        todo!()
    }

    fn new_from_selection(network: Selection<Scope>) -> Result<Selection<Schema>, Box<dyn Error>> {
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

pub(crate) fn create_initial_scopes(scope_db: &mut UnQLite, schema_db: &mut UnQLite) -> Result<(), Box<dyn Error>> {
    if Schema::retrieve_all()?
        .into_iter()
        .map(|f| -> Result<Selection<Scope>, Box<dyn Error>> {
            Scope::new_from_selection(f)
        })
        .map(|f| -> Result<(), Box<dyn Error>> {
            Ok(Scope::save(&mut f?, scope_db)?)
        })
        .any(|f| -> bool {
            f.is_err()
        }) {
            return Err(ScopeInitError.into())
        }
        else {
            Ok(())
        }
}

#[cfg(test)]
mod tests {
    use crate::schema::*;
    use tempfile::NamedTempFile;
    use log::warn;
        
    #[test]
    fn test_schema_dao_no_env() {
        let result = Schema::dao();
        assert!(result.is_err())
    }

    #[test]
    fn test_schema_dao_with_env() {        
        let file = NamedTempFile::new().unwrap();
        std::env::set_var("SCHEMA_DB_FILE", file.into_temp_path());
        let result = Schema::dao();
        assert!(result.is_ok())
    }
    
    #[test]
    fn test_complete_transaction() {
        
        let file = NamedTempFile::new().unwrap();
        std::env::set_var("SCHEMA_DB_FILE", file.into_temp_path());
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
            Err(_) => panic!("failed to begin transaction")
        }
    }

    #[test]
    fn test_begin_tx_within_tx() {
        
        let file = NamedTempFile::new().unwrap();
        std::env::set_var("SCHEMA_DB_FILE", file.into_temp_path());
        let mut dao = Schema::dao().unwrap();

        match Schema::begin_tx(&mut dao) {
            Ok(_) => {
                match Schema::begin_tx(&mut dao) {
                    Ok(_) => todo!("is this bad or good"),
                    Err(_) => todo!("is this good or bad"),
                }
            }
            Err(_) => panic!("failed to start a tx")        
        }
    }

    #[test]
    fn test_end_no_tx() {
        let file = NamedTempFile::new().unwrap();
        std::env::set_var("SCHEMA_DB_FILE", file.into_temp_path());
        let mut dao = Schema::dao().unwrap();
        match Schema::begin_tx(&mut dao) {
            Ok(_) => todo!("end tx suceeeded but no transaction existed"),
            Err(_) => (),
        }
    }


}