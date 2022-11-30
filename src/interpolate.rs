use std::error::Error;
use cidr::IpCidr;

pub struct ProtoScope<T> {
    cidr: Option<T>
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
    pub fn children(&self, new_prefix_length: u8) -> impl Iterator<Item=ProtoScope<IpCidr>> {
        let mut ret = vec![ProtoScope::null().unwrap()];

        ret.truncate(0);

        let mut cur = IpCidr::new(
            self.cidr
                .unwrap()
                .first_address(),
            new_prefix_length);


        if self.cidr == None {
            return ret.into_iter();
        }
        else if cur.is_err() {
            return ret.into_iter();
        }

        loop {
            let mut next = cur.unwrap();
            ret.push(ProtoScope::new_type_backed_proto_scope(next).unwrap());
            cur = IpCidr::new(next.last_address(), new_prefix_length);
            if cur.is_err() {
                return Vec::new().into_iter();
            }
        }
    }
}
