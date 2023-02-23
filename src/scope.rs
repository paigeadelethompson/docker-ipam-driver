use std::error::Error;
use std::time::SystemTime;
use cidr::IpCidr;
use unqlite::Cursor;
use unqlite::Transaction;
use unqlite::UnQLite;
use crate::model::*;
use crate::interpolate::*;
use crate::schema::*;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ScopeDescription {
    prefix_length: u8,
    locked: bool,
    allocated: bool,
    tags: Vec<String>
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Scope {
    pub id: u128,
    pub parent: Option<u128>,
    pub modified: SystemTime,
    pub created: SystemTime,
    pub descriptions: Vec<ScopeDescription>,
}

impl data_operations<Scope, ScopeDescription> for Scope {
    fn begin_tx(db: &mut UnQLite) -> Result<(), unqlite::Error> {
        match db.begin() {
            Ok(_) => {
                Ok(())
            }
            Err(_) => {
                todo!("begin tx err")
            }
        }
    }

    fn roll_back_tx(db: &mut UnQLite) -> Result<(), unqlite::Error> {
        match db.rollback() {
            Ok(_) => {
                todo!()
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
            Err(e) => Err(e)
        }
    }

    fn initialize_db(_db: &mut UnQLite) -> Result<(), Box<dyn Error>> {
        let mut scope_dao = Scope::dao().unwrap();
        let mut schema_dao = Schema::dao().unwrap();
        crate::util::create_initial_scopes(&mut scope_dao, &mut schema_dao)
    }

    fn dao() -> Result<UnQLite, Box<dyn Error>> {
        Ok(UnQLite::create(std::env::var("SCOPE_DB_FILE")?))
    }

    fn save(_s: &mut Selection<Scope>, _db: &mut UnQLite) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    fn exists_in_database(_id: u128) -> Result<bool, Box<dyn Error>> {
        todo!()
    }

    fn retrieve_all() -> Result<Vec<Selection<Scope>>, Box<dyn Error>> {
        panic!("Not implemented for Scopes")
    }

    fn allocate_pool(_tags: Vec<String>) -> Result<Selection<Scope>, Box<dyn Error>> {
        todo!()
    }

    fn allocate_address(_network: String) -> Result<Selection<Scope>, Box<dyn Error>> {
        todo!()
    }

    fn release_pool(_network: String) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    fn release_address(_network: String) -> Result<(), Box<dyn Error>> {
        todo!()
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

impl crate::model::factory<Scope, ScopeDescription, Schema, SchemaDescription> for Scope {
    fn new_from_string(_network: String, _prefix_length: u8, _parent: Option<&mut Selection<Scope>>) -> Result<Selection<Scope>, Box<dyn Error>> {
        todo!()
    }

    fn new_from_bytes(_network: Vec<u8>, _prefix_length: u8, _parent: Option<&mut Selection<Scope>>) -> Result<Selection<Scope>, Box<dyn Error>> {
        todo!()
    }

    fn new_from_proto_scope(network: ProtoScope<IpCidr>, parent: Option<&mut Selection<Scope>>) -> Result<Selection<Scope>, Box<dyn Error>> {
        network.scope_from_proto_scope(parent)
    }

    fn new_from_selection(_network: Selection<Schema>) -> Result<Selection<Scope>, Box<dyn Error>> {
        Ok(Selection {
            actual: Scope {
                id: _network.actual.pool,
                parent: _network.actual.parent,
                modified: SystemTime::now(),
                created: SystemTime::now(),
                descriptions: Vec::new()
            },
            selected_prefix_length: _network.selected_prefix_length,
            saved: false,
            operation: SelectionOperation::DEFAULT
        })
    }

    fn new_from_json(_json: String) -> Result<Selection<Scope>, Box<dyn Error>> {
        todo!()
    }

    fn to_proto_scope(&self) -> Result<ProtoScope<IpCidr>, Box<dyn Error>> {
        todo!()
    }

    fn new_selection(&self) -> Result<Selection<Scope>, Box<dyn Error>> {
        todo!()
    }
}

#[cfg(test)]
mod data_store_tests {
    use log::warn;
    use crate::scope::*;
    
    #[test]
    fn db_is_not_initialized() {
        std::env::set_var("SCOPE_DB_FILE", "");     
        let mut dao = Schema::dao().unwrap();
        assert_eq!(Scope::is_db_initialized(&mut dao).unwrap(), false);        
    }

    #[test]
    fn db_is_initialized() {
        std::env::set_var("SCOPE_DB_FILE", "");     
        let mut dao = Schema::dao().unwrap();
        todo!()
    }
    #[test]
    fn test_roll_back_tx() {
        std::env::set_var("SCOPE_DB_FILE", "");
        let mut dao = Scope::dao().unwrap();

        match Scope::begin_tx(&mut dao) {
            Ok(_) => {
                match Scope::initialize_db(&mut dao) {
                    Ok(_) => {
                        match Scope::roll_back_tx(&mut dao) {
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
    fn test_begin_tx_within_tx() {
        std::env::set_var("SCOPE_DB_FILE", "");
        let mut dao = Scope::dao().unwrap();

        match Scope::begin_tx(&mut dao) {
            Ok(_) => {
                match Scope::begin_tx(&mut dao) {
                    Ok(_) => warn!("is this bad or good"),
                    Err(_) => panic!("unknown behavior")
                }
            }
            Err(e) => panic!("failed to begin transaction: {} ", e)
        }
    }
}