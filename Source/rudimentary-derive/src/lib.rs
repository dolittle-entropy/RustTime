// Taken from https://github.com/PrismaPhonic/domain_patterns/master/domain_derive/src/lib.rs

mod concept_as;
use syn::parse_macro_input;
use quote::quote;
use proc_macro::TokenStream;
use syn::DeriveInput;

const FAILED_PRECONDITION: &str = "ConceptSetup macro failed preconditions";

#[proc_macro_derive(ConceptSetup)]
pub fn derive_concept_setup(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let concept_name = &input.ident;

    concept_as::precondition(&input).expect(FAILED_PRECONDITION);

    let value_type_name = &concept_as::value_type_name(&input.data).unwrap();

    let expanded = quote! {
        impl std::fmt::Display for #concept_name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", self.value)
            }
        }

        impl std::cmp::PartialEq for #concept_name {
            fn eq(&self, other: &Self) -> bool {
                self.value == other.value
            }
        }

        impl std::clone::Clone for #concept_name {
            fn clone(&self) -> Self {
                #concept_name {
                    value: self.value.clone()
                }
            }
        }

        impl std::convert::From<#value_type_name> for #concept_name {

            fn from(value: #value_type_name) -> Self {
                #concept_name {value}
            }
        }
        impl rudimentary::ConceptBase<#value_type_name> for #concept_name {
            fn value(&self) -> #value_type_name {
                self.value
            }
        }
        impl rudimentary::ConceptAs<#value_type_name> for #concept_name {
            fn new(value: #value_type_name) -> Self {
                Self::from(value)
            }
        }
    };

    TokenStream::from(expanded)
}
#[proc_macro_derive(ValidatedConceptSetup)]
pub fn derive_validated_concept_setup(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let concept_name = &input.ident;

    concept_as::precondition(&input).expect(FAILED_PRECONDITION);

    let value_type_name = &concept_as::value_type_name(&input.data).unwrap();

    let expanded = quote! {
        impl std::fmt::Display for #concept_name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", self.value)
            }
        }

        impl std::cmp::PartialEq for #concept_name {
            fn eq(&self, other: &Self) -> bool {
                self.value == other.value
            }
        }

        impl std::clone::Clone for #concept_name {
            fn clone(&self) -> Self {
                #concept_name {
                    value: self.value.clone()
                }
            }
        }

        impl std::convert::TryFrom<#value_type_name> for #concept_name {
            type Error = &'static str;

            fn try_from(value: #value_type_name) -> std::result::Result<Self, Self::Error> {
                Self::validate(&value)?;

                Ok(#concept_name {value})
            }
        }

        impl rudimentary::ConceptBase<#value_type_name> for #concept_name {
            fn value(&self) -> #value_type_name {
                self.value
            }
        }
    };

    TokenStream::from(expanded)
}
