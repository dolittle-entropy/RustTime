use rudimentary_derive::ConceptSetup;
pub use rudimentary::*;

#[derive(ConceptSetup)]
pub struct AggregateRootVersion { value: u32 }

impl ConceptAs<u32> for AggregateRootVersion {
    fn validate(_value: &u32) -> Result<(), &'static str> {
        Ok(())
    }

    fn value(&self) -> u32 {
        return self.value;
    }
}
