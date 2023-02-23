use std::error::Error;
use std::time::SystemTime;
use cidr::Cidr;
use cidr::IpCidr;
use log::warn;
use crate::schema::*;
use crate::scope::*;
use crate::model::{SelectionOperation};
use crate::{util, model::{Selection}};

pub struct ProtoScope<T> {
    pub cidr: Option<T>
}

pub trait factory<T> {
    fn null() -> Result<ProtoScope<T>, Box<dyn Error>>;
    fn new_type_backed_proto_scope(backing: T) -> Result<ProtoScope<T>, Box<dyn Error>>;
}

impl factory<IpCidr> for ProtoScope<IpCidr> {
    fn null() -> Result<ProtoScope<IpCidr>, Box<dyn Error>> {
        Ok(ProtoScope{
            cidr: None,
        })
    }

    fn new_type_backed_proto_scope(backing: IpCidr) -> Result<ProtoScope<IpCidr>, Box<dyn Error>> {
        Ok(ProtoScope { 
            cidr: Some(backing) 
        })
    }
}

impl ProtoScope<IpCidr> {
    pub fn scope_from_proto_scope(&self, _parent: Option<&mut Selection<Scope>>) -> Result<Selection<Scope>, Box<dyn Error>> {    
        match self.cidr {
            Some(v) => Ok(Selection {
                actual: Scope {
                    id: util::string_to_u128_id(v.to_string())?,
                    parent: match _parent {
                        Some(parent) => {
                            Some(parent.actual.id)
                        },
                        None => None
                    },
                    modified: SystemTime::now(),
                    created: SystemTime::now(),
                    descriptions: Vec::new(),
                },
                selected_prefix_length: Some(v.network_length()),
                saved: false,
                operation: SelectionOperation::DEFAULT,
            }),
            None => Err("uninitalized protoscope can't convert to scope".into())
        }
    }

    pub fn schema_from_proto_scope(&self, _parent: Option<&mut Selection<Schema>>) -> Result<Selection<Schema>, Box<dyn Error>> {    
        match self.cidr {
            Some(v) => {
                Ok(Selection {
                    actual: Schema {
                        pool: util::string_to_u128_id(v.first_address().to_string())?,
                        descriptions: [None, None],
                        parent: match _parent {
                            Some(parent) => Some(parent.actual.pool),
                            None => None,
                        }
                    },
                    selected_prefix_length: Some(v.network_length()),
                    saved: false,
                    operation: SelectionOperation::DEFAULT,
                })
            }
            None => Err("uninitalized protoscope can't convert to schema".into())
        }
    }

    pub fn children(&self, new_prefix_length: u8) -> impl Iterator<Item = ProtoScope<IpCidr>> {
        let mut ret = vec![ProtoScope::null().unwrap()];
        ret.truncate(0);

        let mut cur = IpCidr::new(
            self.cidr
                .unwrap()
                .first_address(),
            new_prefix_length);


        if self.cidr.is_none() {
            warn!("empty prefixlen: {}", cur.unwrap_err());
            return ret.into_iter();
        }
        else if cur.is_err() {
            warn!("interpolation error: {}", cur.unwrap_err());
            return ret.into_iter();
        }

        loop {
            let next = cur.unwrap();
            ret.push(ProtoScope::new_type_backed_proto_scope(next).unwrap());
            let inc = util::increment_address(next.last_address()).unwrap();

            if self.cidr.unwrap().last_address() > inc {
                cur = IpCidr::new(util::increment_address(next.last_address()).unwrap(), new_prefix_length);
                if cur.is_err() {
                    warn!("interpolation error: {}", cur.unwrap_err());
                    return ret.into_iter();
                }
                
            }
            else {
                return ret.into_iter();
            }
        }
    }
}

#[cfg(test)]
mod interpolation_tests {
    use std::str::FromStr;
    use crate::interpolate::*;

    #[test]
    fn new_type_backed_proto_scope() {
        let ps = ProtoScope::new_type_backed_proto_scope(
            IpCidr::from_str("100.64.0.0/17").unwrap()).unwrap();        
        let count = ps.children(20).count();
        assert_eq!(count, 8);
    }
}