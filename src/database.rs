use std::error::Error;
use crate::schema::create_initial_scopes;
use crate::model::data_operations;
use crate::schema::Schema;
use crate::scope::Scope;

pub(crate) fn initialize_databases() -> Result<(), Box<dyn Error>> {
    match Schema::dao() {
        Ok(mut _db) => {
            match Schema::begin_tx(&mut _db) {
                Ok(_) => {
                    match Schema::initialize_db(&mut _db) {
                        Ok(_) => {
                            match Schema::commit(&mut _db) {
                                Ok(_) => {
                                    match Scope::dao() {
                                        Ok(mut _s_db) => {
                                            match Scope::begin_tx(&mut _s_db) {
                                                Ok(_) => {
                                                    match create_initial_scopes(&mut _s_db, &mut _db) {
                                                        Ok(_) => {
                                                            match Scope::commit(&mut _s_db) {
                                                                Ok(_) => Ok(()),
                                                                Err(e) => {
                                                                    match Scope::roll_back_tx(&mut _s_db) {
                                                                        Ok(_) => {
                                                                            panic!("Scope DB commit failed, rolled back: {}", e)
                                                                        }
                                                                        Err(ee) => {
                                                                            panic!("Scope DB commit failed, failed to roll back: {} {}", e, ee)
                                                                        },
                                                                    }
                                                                }
                                                            }
                                                        },
                                                        Err(_) => todo!(),
                                                    }
                                                }
                                                Err(e) => {
                                                    panic!("failed to begin tx for scope database: {}", e)
                                                }
                                            }
                                        }
                                        Err(e) => {
                                        panic!("failed to open scope database: {}", e)
                                        }
                                    }
                                }
                                Err(e) => {
                                    panic!("failed to commit schema database: {}", e)
                                }
                            }
                        }
                        Err(e) => {
                            match Schema::roll_back_tx(&mut _db) {
                                Ok(_) => {
                                    panic!("initialization failed, schema changes rolled back: {}", e)
                                }
                                Err(ee) => {
                                   panic!("failed to rollback schema tx after initialization of the db: {} {}", e, ee)
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    panic!("failed to begin schema tx: {}", e)
                }
            }
        }
        Err(e) => {
            panic!("failed to open schema database: {}", e)
        }
    }
}