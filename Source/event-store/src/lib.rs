mod aggregate_root;

pub fn hello() {
    println!("Hello event store")
}

pub use aggregate_root::*;

pub fn new_version(value: u32) -> AggregateRootVersion  {
    let try_from = AggregateRootVersion::try_from(value);
    try_from.unwrap()
}
