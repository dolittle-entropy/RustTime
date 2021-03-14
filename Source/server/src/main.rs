use event_store::Concept;

fn main() {
    println!("Hello world!");
    let event_source  = event_store::EventSourceId::create();
    println!("{}", event_source);

}
