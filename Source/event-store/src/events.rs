pub use rudimentary::*;
use rudimentary_derive::ConceptSetup;
use uuid::Uuid;

#[derive(ConceptSetup)]
pub struct EventSourceId {
    value: Uuid,
}

impl Concept<Uuid> for EventSourceId {
    fn get_value(&self) -> Uuid {
        self.value
    }
    fn borrow_value(&self) -> &Uuid {
        &self.value
    }
}

impl EventSourceId {
    pub fn create() -> EventSourceId {
        EventSourceId::new(Uuid::new_v4())
    }
}
