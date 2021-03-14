pub use rudimentary::*;
use rudimentary_derive::ConceptSetup;
use uuid::Uuid;

#[derive(ConceptSetup)]
pub struct MicroserviceId {
    value: Uuid,
}

impl Concept<Uuid> for MicroserviceId {
    fn get_value(&self) -> Uuid {
        self.value
    }
    fn borrow_value(&self) -> &Uuid {
        &self.value
    }
}

impl MicroserviceId {
    fn create() -> MicroserviceId {
        MicroserviceId::new(Uuid::new_v4())
    }
    fn NOT_SET() -> MicroserviceId {
        MicroserviceId::new(Uuid::parse_str("4a5d2bc3-543f-459a-ab0b-e8e924093260").unwrap())
    }
}

#[derive(ConceptSetup)]
pub struct MicroserviceName {
    value: String,
}

impl Concept<String> for MicroserviceName {
    fn get_value(&self) -> String {
        self.value.clone()
    }
    fn borrow_value(&self) -> &String {
        &self.value
    }
}
