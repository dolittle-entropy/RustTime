// From https://github.com/PrismaPhonic/domain_patterns/blob/master/domain_derive/src/value_object.rs
pub use std::fmt::Display;
pub use std::convert::{From, TryFrom};

pub trait ConceptBase<TValue> {
    /// `value` return a reference to the internal value held in the value object. This should be the only
    /// way that we access the internal data.  Mutation methods should always generate a new value object.
    /// Note: It's intentional that value returns an owned type.  This is necessary for enums, where we likely
    /// want to return a String after matching (since a string is how we match to figure out the variant upon value object
    /// creation), but in that case the string is created on the match in value(), and therefore we must pass back an owned
    /// value, not a ref (the string that was freshly created would be dropped at the end of value() if we try to pass
    /// back a ref of it).
    fn value(&self) -> TValue;
}

pub trait ConceptAs<TValue>: ConceptBase<TValue> + Clone + PartialEq + From<TValue> + Display{
    fn new(value: TValue) -> Self;
}

pub trait ValidatedConceptAs<TValue>: ConceptBase<TValue> + Clone + PartialEq + TryFrom<TValue> + Display {

    /// `validate` takes in incoming data used to construct the value object, and validates it against
    /// given constraints.  An example would be if we had an `Email` struct that implements `ValueObject`.
    /// The constraints we would check would ensure that the incoming data is a valid email address.
    fn validate(value: &TValue) -> Result<(), &'static str>;
}
