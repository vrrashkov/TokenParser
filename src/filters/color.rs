use easy_color::{ColorError, IntoRGBA};
use easy_color::{Hex, IntoHex};
use liquid_core::Expression;
use liquid_core::Runtime;
use liquid_core::{
    Display_filter, Filter, FilterParameters, FilterReflection, FromFilterParameters, ParseFilter,
};
use liquid_core::{Error, Result};
use liquid_core::{Value, ValueView};

use crate::deserializer;
use crate::utils;

#[derive(Debug, FilterParameters)]
struct ColorArgs {
    #[parameter(description = "The format to return the color in.", arg_type = "str")]
    format: Expression,
}

#[derive(Clone, ParseFilter, FilterReflection)]
#[filter(
    name = "color",
    description = "Color filter",
    parameters(ColorArgs),
    parsed(ColorFilter)
)]
pub struct Color;

#[derive(Debug, FromFilterParameters, Display_filter)]
#[name = "color"]
struct ColorFilter {
    #[parameters]
    args: ColorArgs,
}


impl Filter for ColorFilter {
    
    fn evaluate(&self, input: &dyn ValueView, runtime: &dyn Runtime) -> Result<Value> {
        let args = self.args.evaluate(runtime)?;
        let color_value = input.to_kstr();
        let v: Result<Hex, ColorError> = color_value.as_str().try_into();
        match v {
            Ok(formatted_color_value) => {
                let rgba = formatted_color_value.to_rgba();
                let rgba_r = rgba.red() as f32;
                let rgba_g = rgba.green() as f32;
                let rgba_b = rgba.blue() as f32;
                let color_rgba_v1 = (rgba_r/255., rgba_g/255., rgba_b/255., rgba.alpha());
                let color_rgba_v2 = (rgba.red(), rgba.green(), rgba.blue(), rgba.alpha());

                if !args.format.is_empty() {
                    //println!("color args: {}", args.format);
                    let mut color_formatted = args.format.to_string();
                    let rgb_r_v1 = format!("{:.3}", color_rgba_v1.0);
                    color_formatted = color_formatted.replace(deserializer::TemplateFieldVariantColor::rgb_r_v1.to_str(), &rgb_r_v1);
                    let rgb_g_v1 = format!("{:.3}", color_rgba_v1.1);
                    color_formatted = color_formatted.replace(deserializer::TemplateFieldVariantColor::rgb_g_v1.to_str(), &rgb_g_v1);
                    let rgb_b_v1 = format!("{:.3}", color_rgba_v1.2);
                    color_formatted = color_formatted.replace(deserializer::TemplateFieldVariantColor::rgb_b_v1.to_str(), &rgb_b_v1);
                    let rgb_a_v1 = format!("{:.3}", color_rgba_v1.3);
                    color_formatted = color_formatted.replace(deserializer::TemplateFieldVariantColor::rgb_a_v1.to_str(), &rgb_a_v1);
                    let rgb_r_v2 = format!("{:.3}", color_rgba_v2.0);
                    color_formatted = color_formatted.replace(deserializer::TemplateFieldVariantColor::rgb_r_v2.to_str(), &rgb_r_v2);
                    let rgb_g_v2 = format!("{:.3}", color_rgba_v2.1);
                    color_formatted = color_formatted.replace(deserializer::TemplateFieldVariantColor::rgb_g_v2.to_str(), &rgb_g_v2);
                    let rgb_b_v2 = format!("{:.3}", color_rgba_v2.2);
                    color_formatted = color_formatted.replace(deserializer::TemplateFieldVariantColor::rgb_b_v2.to_str(), &rgb_b_v2);
                    let rgb_a_v2 = format!("{:.3}", color_rgba_v2.3);
                    color_formatted = color_formatted.replace(deserializer::TemplateFieldVariantColor::rgb_a_v2.to_str(), &rgb_g_v2);
                    let hex = formatted_color_value.to_string();
                    color_formatted = color_formatted.replace(deserializer::TemplateFieldVariantColor::hex.to_str(), &hex);
                    Ok(Value::scalar(color_formatted))
                } else {
                    Ok(Value::scalar(color_value.as_str().to_string()))
                }

            },
            _ => Ok(Value::scalar(color_value.as_str().to_string()))
            
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_without_args() {
        assert_eq!(
            liquid_core::call_filter!(Color, "#FFFFFF", "").unwrap(),
            liquid_core::value!("#FFFFFF")
        );
    }
    
    #[test]
    fn color_with_args_rgb() {
        assert_eq!(
            liquid_core::call_filter!(Color, "#FFFFFF", "rgb_r_v2").unwrap(),
            liquid_core::value!("255")
        );
    }
}