use std::error::Error;
use cidr::IpCidr;
use unqlite::UnQLite;
use crate::interpolate::ProtoScope;

pub enum SelectionOperation {
    UPDATE_PARENT_DESCRIPTIONS,
    DEFAULT
}

pub struct Selection<RECORD_TYPE> {
    pub actual: RECORD_TYPE,
    pub selected_prefix_length: Option<u8>,
    pub saved: bool,
    pub operation: SelectionOperation
}

pub trait data_operations<RECORD_TYPE, DESCRIPTION_TYPE> {
    fn begin_tx(db: &mut UnQLite) -> Result<(), unqlite::Error>;
    fn roll_back_tx(db: &mut UnQLite) -> Result<(), unqlite::Error>;
    fn commit(db: &mut UnQLite) -> Result<(), unqlite::Error>;
    fn initialize_db(db: &mut UnQLite) -> Result<(), Box<dyn Error>>;
    fn dao() -> Result<UnQLite, Box<dyn Error>>;
    fn save(s: &mut Selection<RECORD_TYPE>, db: &mut UnQLite) -> Result<(), Box<dyn Error>>;
    fn scope_exists_in_database(id: u128) -> Result<bool, Box<dyn Error>>;
    fn retrieve_all() -> Result<Vec<Selection<RECORD_TYPE>>, Box<dyn Error>>;
    fn is_db_initialized(db: &mut UnQLite) -> Result<bool, Box<dyn Error>>;
}

pub trait locking_operations {
    fn lock(&self) -> Result<bool, Box<dyn Error>>;
    fn unlock(&self) -> Result<bool, Box<dyn Error>>;
    fn is_locked(&self) -> Result<bool, Box<dyn Error>>;
}

pub trait factory<RECORD_TYPE, DESCRIPTION_TYPE, FROM_SELECTION_TYPE, FROM_SELECTION_DESCRIPTION_TYPE> {
    fn new_from_string(network: String, prefix_length: u8, parent: Option<&mut Selection<RECORD_TYPE>>) -> Result<Selection<RECORD_TYPE>, Box<dyn Error>>;
    fn new_from_bytes(network: Vec<u8>, prefix_length: u8, parent: Option<&mut Selection<RECORD_TYPE>>) -> Result<Selection<RECORD_TYPE>, Box<dyn Error>>;
    fn new_from_proto_scope(network: ProtoScope<IpCidr>, parent: Option<&mut Selection<RECORD_TYPE>>) -> Result<Selection<RECORD_TYPE>, Box<dyn Error>>;
    fn new_from_selection(network: Selection<FROM_SELECTION_TYPE>) -> Result<Selection<RECORD_TYPE>, Box<dyn Error>>;
    fn new_from_json(json: String) -> Result<Selection<RECORD_TYPE>, Box<dyn Error>>;
    fn to_proto_scope(&self) -> Result<ProtoScope<IpCidr>, Box<dyn Error>>;
    fn new_selection(&self) -> Result<Selection<RECORD_TYPE>, Box<dyn Error>>;
}
