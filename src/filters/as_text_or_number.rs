use liquid_core::Result;
use liquid_core::Runtime;
use liquid_core::{Display_filter, Filter, FilterReflection, ParseFilter};
use liquid_core::{Value, ValueView};

#[derive(Clone, ParseFilter, FilterReflection)]
#[filter(
    name = "as_text_or_number",
    description = "Add quotes if text if not display as number",
    parsed(AsTextOrNumberFilter)
)]
pub struct AsTextOrNumber;

#[derive(Debug, Default, Display_filter)]
#[name = "as_text_or_number"]
struct AsTextOrNumberFilter;

impl Filter for AsTextOrNumberFilter {
    fn evaluate(&self, input: &dyn ValueView, _runtime: &dyn Runtime) -> Result<Value> {
        if input.is_nil() {
            return Ok(Value::Nil);
        }

        let num: std::result::Result<f64, std::num::ParseFloatError> = input.to_kstr().parse::<f64>();
        return match num {
            Ok(val) => Ok(Value::scalar(val)),
            Err(why) => Ok(Value::scalar(format!("\"{}\"", input.to_kstr().to_string())))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_without_quotes() {
        assert_eq!(
            liquid_core::call_filter!(AsTextOrNumber, 2.4).unwrap(),
            liquid_core::value!(2.4)
        );
    }
    
    #[test]
    fn number_in_quotes() {
        assert_eq!(
            liquid_core::call_filter!(AsTextOrNumber, "2.4").unwrap(),
            liquid_core::value!(2.4)
        );
    }
    
    #[test]
    fn text() {
        assert_eq!(
            liquid_core::call_filter!(AsTextOrNumber, "test").unwrap(),
            liquid_core::value!("\"test\"")
        );
    }
}