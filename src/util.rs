use std::{net::IpAddr, error::Error, str::FromStr};
use cidr::IpCidr;
use unqlite::UnQLite;
use crate::error::ScopeInitError;
use crate::model::factory;
use crate::scope::*;
use crate::schema::*;
use crate::model::*;

pub fn string_to_ip_cidr(network: String, prefix_length: u8) -> Result<IpCidr, Box<dyn Error>> {
    Ok(IpCidr::new(IpAddr::from_str(network.as_str())?, prefix_length)?)
}

pub fn string_to_u128_id(network: String) -> Result<u128, Box<dyn Error>> {
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

pub fn increment_address(network: IpAddr) -> Result<IpAddr, Box<dyn Error>> {
    Ok(match IpAddr::from_str(&mut network.to_string())? {
        IpAddr::V4(a) => {
            let mut v: u32 = a.into();
            v = v + 1;
            IpAddr::from(std::net::Ipv4Addr::from(v))
        }
        IpAddr::V6(a) => {
            let mut v: u128 = a.into();
            v = v + 1;
            IpAddr::from(std::net::Ipv6Addr::from(v))
        }
    })
}
pub fn create_initial_scopes(scope_db: &mut UnQLite, _schema_db: &mut UnQLite) -> Result<(), Box<dyn Error>> {
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