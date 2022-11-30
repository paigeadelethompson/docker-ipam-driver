use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub(crate) struct DBSaveError;

#[derive(Debug, Clone)]
pub(crate) struct SchemaInitError;

#[derive(Debug, Clone)]
pub(crate) struct ScopeInitError;

impl Display for DBSaveError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str("Error saving record")
    }
}

impl Error for DBSaveError {

}

impl Display for SchemaInitError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl Error for SchemaInitError {

}

impl Display for ScopeInitError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl Error for ScopeInitError {
    
}