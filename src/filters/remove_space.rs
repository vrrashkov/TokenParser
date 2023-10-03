use liquid_core::Result;
use liquid_core::Runtime;
use liquid_core::{Display_filter, Filter, FilterReflection, ParseFilter};
use liquid_core::{Value, ValueView};

#[derive(Clone, ParseFilter, FilterReflection)]
#[filter(
    name = "remove_space",
    description = "Removes the space between characters",
    parsed(RemoveSpaceFilter)
)]
pub struct RemoveSpace;

#[derive(Debug, Default, Display_filter)]
#[name = "remove_space"]
struct RemoveSpaceFilter;

impl Filter for RemoveSpaceFilter {
    fn evaluate(&self, input: &dyn ValueView, _runtime: &dyn Runtime) -> Result<Value> {
        if input.is_nil() {
            return Ok(Value::Nil);
        }

        let s = input.to_kstr();
        let mut text = s.to_string();
        text.retain(|c| !c.is_whitespace());
        Ok(Value::scalar(text))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remove_space_success() {
        assert_eq!(
            liquid_core::call_filter!(RemoveSpace, "foo bar").unwrap(),
            liquid_core::value!("foobar")
        );
    }
}
