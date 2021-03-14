pub use rudimentary::*;
use rudimentary_derive::ConceptSetup;
use uuid::Uuid;

#[derive(ConceptSetup)]
pub struct TenantId { value: Uuid }

impl Concept<Uuid> for TenantId {
    fn get_value(&self) -> Uuid {
        self.value
    }
    fn borrow_value(&self) -> &Uuid {
        &self.value
    }
}

impl TenantId {
    
    fn UNKNOWN() -> TenantId {
        TenantId::new(Uuid::parse_str("762a4bd5-2ee8-4d33-af06-95806fb73f4e").unwrap())
    }
    fn SYSTEM() -> TenantId {
        TenantId::new(Uuid::parse_str("08831584-e016-42f6-bc5e-c4f098fed42b").unwrap())
    }
    fn DEVELOPMENT() -> TenantId {
        TenantId::new(Uuid::parse_str("445f8ea8-1a6f-40d7-b2fc-796dba92dc44").unwrap())
    }
}
