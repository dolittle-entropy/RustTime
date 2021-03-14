// From https://github.com/PrismaPhonic/domain_patterns/blob/master/domain_derive/src/value_object.rs
pub use std::fmt::Display;
pub use std::convert::{From, TryFrom};

pub trait Concept<TValue>: Clone + PartialEq + Display {
    fn get_value(&self) -> TValue;
    fn borrow_value(&self) -> &TValue;
}
pub trait ConceptAs<TValue>: Concept<TValue> + From<TValue> {
    fn new(value: TValue) -> Self;
}
pub trait ValidatedConceptAs<TValue>: Concept<TValue> + TryFrom<TValue> {
    type ValidationError;
    fn new(value: TValue) -> Result<Self, Self::ValidationError>;
    fn validate(value: &TValue) -> Result<(), Self::ValidationError>;
}
