use std::error::Error;
use std::time::SystemTime;
use cidr::IpCidr;
use unqlite::Cursor;
use unqlite::Transaction;
use unqlite::UnQLite;
use crate::model::{data_operations, factory, Selection};
use crate::interpolate::ProtoScope;
use crate::schema::{Schema, SchemaDescription};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ScopeDescription {
    prefix_length: u8,
    locked: bool,
    allocated: bool,
    tags: Vec<String>
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Scope {
    id: u128,
    parent: Option<u128>,
    modified: SystemTime,
    created: SystemTime,
    descriptions: Vec<ScopeDescription>,
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
                todo!()
            }
            Err(_) => {
                todo!()
            }
        }
    }

    fn initialize_db(_db: &mut UnQLite) -> Result<(), Box<dyn Error>> {
        todo!()
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
        todo!("Not implemented for Scopes")
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

impl factory<Scope, ScopeDescription, Schema, SchemaDescription> for Scope {
    fn new_from_string(_network: String, _prefix_length: u8, _parent: Option<&mut Selection<Scope>>) -> Result<Selection<Scope>, Box<dyn Error>> {
        todo!()
    }

    fn new_from_bytes(_network: Vec<u8>, _prefix_length: u8, _parent: Option<&mut Selection<Scope>>) -> Result<Selection<Scope>, Box<dyn Error>> {
        todo!()
    }

    fn new_from_proto_scope(_network: ProtoScope<IpCidr>, _parent: Option<&mut Selection<Scope>>) -> Result<Selection<Scope>, Box<dyn Error>> {
        todo!()
    }

    fn new_from_selection(_network: Selection<Schema>) -> Result<Selection<Scope>, Box<dyn Error>> {
        todo!()
    }

    fn to_proto_scope(&self) -> Result<ProtoScope<IpCidr>, Box<dyn Error>> {
        todo!()
    }

    fn new_selection(&self) -> Result<Selection<Scope>, Box<dyn Error>> {
        todo!()
    }

    fn new_from_json(_json: String) -> Result<Selection<Scope>, Box<dyn Error>> {
        todo!()
    }
}