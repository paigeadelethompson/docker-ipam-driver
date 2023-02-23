#![feature(iter_array_chunks)]
#![feature(generator_trait)]
#![feature(generators)]
#![feature(decl_macro)]

use std::error::Error;
use database::initialize_databases;
extern crate core;

mod scope;
mod schema;
mod model;
mod interpolate;
mod error;
mod http;
mod database; 
mod util;

fn main() -> Result<(), Box<dyn Error>> {
    initialize_databases()?;
    Ok(())
}