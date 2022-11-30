use rocket::*;

struct IpamConf {
    PreferredPool: String,
    SubPool: String,
    Options: Vec<String>,
    Gateway: String,
    AuxAddresses: Vec<String>
}

#[get("/IpamDriver.GetDefaultAddressSpaces")]
fn get_default_address_spaces() -> &'static str {
    todo!()
}

#[post("/IpamDriver.RequestPool")]
fn request_pool() -> &'static str {
    todo!()
}

#[post("/IpamDriver.ReleasePool")]
fn release_pool() -> &'static str {
    todo!()
}

#[get("/IpamDriver.RequestAddress")]
fn request_address() -> &'static str {
    todo!()
}

#[get("/IpamDriver.ReleaseAddress")]
fn release_address() -> &'static str {
    todo!()
}

pub(crate) fn http_server() {
    rocket::ignite()
    .mount("/IpamDriver.GetDefaultAddressSpaces", routes![get_default_address_spaces])
    .mount("/IpamDriver.RequestPool", routes![request_pool])
    .mount("/IpamDriver.ReleasePool", routes![release_pool])
    .mount("/IpamDriver.RequestAddress", routes![request_address]);
}