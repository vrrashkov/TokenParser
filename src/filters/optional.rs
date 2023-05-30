use easy_color::{ColorError, IntoRGBA};
use easy_color::{Hex, IntoHex};
use liquid_core::Expression;
use liquid_core::Runtime;
use liquid_core::{
    Display_filter, Filter, FilterParameters, FilterReflection, FromFilterParameters, ParseFilter,
};
use liquid_core::{Error, Result};
use liquid_core::{Value, ValueView};

use crate::{deserializer, global};
use crate::utils;

#[derive(Debug, FilterParameters)]
struct OptionalArgs {
    #[parameter(description = "The format to return the optional value in.", arg_type = "str")]
    format: Expression,
}

#[derive(Clone, ParseFilter, FilterReflection)]
#[filter(
    name = "optional",
    description = "Optional filter",
    parameters(OptionalArgs),
    parsed(OptionalFilter)
)]
pub struct Optional;

#[derive(Debug, FromFilterParameters, Display_filter)]
#[name = "optional"]
struct OptionalFilter {
    #[parameters]
    args: OptionalArgs,
}


impl Filter for OptionalFilter {
    fn evaluate(&self, input: &dyn ValueView, runtime: &dyn Runtime) -> Result<Value> {
        let args = self.args.evaluate(runtime)?;
        let value = input.to_kstr();

        if !args.format.is_empty() && !value.is_empty() {
            let formatted_value = args.format.replace(global::optional_value, &value);
            Ok(Value::scalar(formatted_value))
        } else {
            Ok(Value::scalar(""))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn optional_without_args() {
        assert_eq!(
            liquid_core::call_filter!(Optional, "", "test %value").unwrap(),
            liquid_core::value!("")
        );
    }
    
    #[test]
    fn optional_with_args() {
        assert_eq!(
            liquid_core::call_filter!(Optional, "1", "test %value").unwrap(),
            liquid_core::value!("test 1")
        );
    }
}