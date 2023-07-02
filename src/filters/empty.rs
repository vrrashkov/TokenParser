use liquid_core::Result;
use liquid_core::Runtime;
use liquid_core::{Display_filter, Filter, FilterReflection, ParseFilter};
use liquid_core::{Value, ValueView};

#[derive(Clone, ParseFilter, FilterReflection)]
#[filter(
    name = "empty",
    description = "Return empty only intiialize",
    parsed(EmptyFilter)
)]
pub struct Empty;

#[derive(Debug, Default, Display_filter)]
#[name = "initialize"]
struct EmptyFilter;

impl Filter for EmptyFilter {
    fn evaluate(&self, input: &dyn ValueView, _runtime: &dyn Runtime) -> Result<Value> {
        return Ok(Value::Nil);
    }
}
