use std::{fmt, fmt::Display, fmt::Formatter};

use application_model::*;
pub use rudimentary::*;
use rudimentary_derive::ConceptSetup;
use security::*;
use uuid::Uuid;
use versioning::*;

#[derive(ConceptSetup)]
pub struct CorrelationId {
    value: Uuid,
}

impl Concept<Uuid> for CorrelationId {
    fn get_value(&self) -> Uuid {
        self.value
    }
    fn borrow_value(&self) -> &Uuid {
        &self.value
    }
}

#[derive(ConceptSetup)]
pub struct Environment {
    value: String,
}

impl Concept<String> for Environment {
    fn get_value(&self) -> String {
        self.value.clone()
    }
    fn borrow_value(&self) -> &String {
        &self.value
    }
}

#[derive(PartialEq)]
pub struct ExecutionContext {
    microservice: MicroserviceId,
    tenant: TenantId,
    version: Version,
    environment: Environment,
    correlation: CorrelationId,
    claims: Vec<Claim>,
}

impl Display for ExecutionContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "ExecutionContext(Microservice {}, Tenant {}, Version {}, Environment {}, Correlation {}, Claims {:?})", self.microservice, self.tenant, self.version, self.environment, self.correlation, self.claims )
    }
}
