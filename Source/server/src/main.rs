use event_store::*;
fn main() {
    println!("Hello, world!");
    event_store::hello();
    let val: u32 = 2;
    let version: AggregateRootVersion = val.into();
    println!("{}", version);
}
