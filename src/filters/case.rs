#![allow(clippy::invisible_characters)]

use liquid_core::Result;
use liquid_core::Runtime;
use liquid_core::{Display_filter, Filter, FilterReflection, ParseFilter};
use liquid_core::{Value, ValueView};

use convert_case::{Case, Casing};

#[derive(Clone, ParseFilter, FilterReflection)]
#[filter(
    name = "kebab",
    description = "Makes each character in a string kebab case.",
    parsed(KebabCaseFilter)
)]
pub struct KebabCase;

#[derive(Debug, Default, Display_filter)]
#[name = "kebab"]
struct KebabCaseFilter;

impl Filter for KebabCaseFilter {
    fn evaluate(&self, input: &dyn ValueView, _runtime: &dyn Runtime) -> Result<Value> {
        let s = input.to_kstr();
        Ok(Value::scalar(s.to_string().to_case(Case::Kebab)))
    }
}

#[derive(Clone, ParseFilter, FilterReflection)]
#[filter(
    name = "pascal",
    description = "Makes each character in a string pascal case.",
    parsed(PascalCaseFilter)
)]
pub struct PascalCase;

#[derive(Debug, Default, Display_filter)]
#[name = "pascal"]
struct PascalCaseFilter;

impl Filter for PascalCaseFilter {
    fn evaluate(&self, input: &dyn ValueView, _runtime: &dyn Runtime) -> Result<Value> {
        let s = input.to_kstr();
        Ok(Value::scalar(s.to_string().to_case(Case::Pascal)))
    }
}

#[derive(Clone, ParseFilter, FilterReflection)]
#[filter(
    name = "camel",
    description = "Makes each character in a string camel case.",
    parsed(CamelCaseFilter)
)]
pub struct CamelCase;

#[derive(Debug, Default, Display_filter)]
#[name = "camel"]
struct CamelCaseFilter;

impl Filter for CamelCaseFilter {
    fn evaluate(&self, input: &dyn ValueView, _runtime: &dyn Runtime) -> Result<Value> {
        let s = input.to_kstr();
        Ok(Value::scalar(s.to_string().to_case(Case::Camel)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_kebab() {
        assert_eq!(
            liquid_core::call_filter!(KebabCase, "test_test").unwrap(),
            liquid_core::value!("test-test")
        );
    }

    #[test]
    fn unit_pascal() {
        assert_eq!(
            liquid_core::call_filter!(PascalCase, "test_test").unwrap(),
            liquid_core::value!("TestTest")
        );
    }

    #[test]
    fn unit_camel() {
        assert_eq!(
            liquid_core::call_filter!(CamelCase, "test_test").unwrap(),
            liquid_core::value!("testTest")
        );
    }
}
