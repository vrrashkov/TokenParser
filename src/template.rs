use anyhow::Context;
use convert_case::{Case, Casing};
use easy_color::{ColorError, IntoRGBA};
use easy_color::{Hex, IntoHex};
use liquid_core::ValueView;
use serde::de::value::MapAccessDeserializer;
use std::cell::RefCell;
use std::rc::Rc;
use std::str::FromStr;

use crate::deserializer;
use crate::deserializer::TokenDataColorValue;
use crate::general;
use crate::utils;

//#[derive(Template, Debug)] // this will generate the code...
//#[template(path = "custom_template.html", escape = "none")] // using the template in this path, relative
pub struct CustomTemplate {
    // the name of the struct can be anything
    pub headers: Vec<String>,
    pub footers: Vec<String>,
    pub values: Option<Vec<String>>,
}

impl CustomTemplate {
    fn set_template_value(current: &mut Option<Vec<String>>, mut values: Vec<String>) {
        if let Some(val) = current {
            val.append(&mut values);
        } else {
            *current = Some(values);
        }
    }
    pub fn update_template_values(&mut self, config_type: String, mut values: Vec<String>) {
        let mut current_values: Vec<String> = Vec::new();
        Self::set_template_value(&mut self.values, values);
    }
}

#[derive(Clone, Debug, Default)]
pub struct TokenValue {
    pub name: String,
    pub value: deserializer::TokenDataType,
    pub path: Vec<String>,
    pub token_type: TokenValueType,
    pub description: Option<String>,
}
#[derive(Clone, Debug, Default)]
pub struct TokenValueType {
    pub text: String,
}

impl TokenValue {
    pub fn variable_name(&self) -> String {
        format!("{}_{}", self.path.join("_"), self.name.replace('/', " "))
    }
}
#[derive(Clone, Debug, Default)]
pub struct TokenData {
    // The general name it can be used for naming the file
    pub name: String,
    // Key = the token type so we can sort and extract all types where we want to
    // so we sore it in a map with duplicated keyslike MultiMap crate
    //pub token_value: MultiMap<String, TokenValue>
    pub t_type: String, //deserializer::ConfigTemplateType,
    pub token_value: Vec<TokenValue>,
}

#[derive(Clone, Debug)]
pub struct TokenDataWrapper {
    // The name of the file
    // It is used for separating different styles
    // For example dark/light etc..
    pub style_name: String,
    pub token_data: Vec<TokenData>,
}

pub fn set_global<T: ToString>(globals: &mut liquid_core::Object, key: &str, value: T) {
    globals.insert(
        key.to_owned().into(),
        liquid::model::Value::scalar(value.to_string()),
    );
}

pub fn set_optional_global(
    globals: &mut liquid_core::Object,
    key: &str,
    value: Option<serde_json::Value>,
    default: &str,
) {
    if let Some(val) = value {
        let mut value_transformed = liquid::model::Value::scalar(default.to_string());
        match val {
            serde_json::Value::Bool(pure_value) => {
                value_transformed = liquid::model::Value::scalar(pure_value);
            }
            serde_json::Value::Number(pure_value) => {
                let pure_value_number: f64 = pure_value
                    .as_f64()
                    .with_context(|| format!("Cannot parse to f64 {}", pure_value))
                    .unwrap();
                value_transformed = liquid::model::Value::scalar(pure_value_number);
            }
            serde_json::Value::String(pure_value) => {
                if let Ok(ev) = evalexpr::eval(&pure_value) {
                    value_transformed = liquid::model::Value::scalar(
                        ev.as_number()
                            .with_context(|| format!("Cannot set as_number {}", ev))
                            .unwrap(),
                    );
                } else {
                    value_transformed = liquid::model::Value::scalar(pure_value);
                }
            }
            serde_json::Value::Array(pure_value) => {
                let obj = liquid_core::to_value(&pure_value)
                    .with_context(|| "Cannot convert to value".to_string())
                    .unwrap();
                value_transformed = obj.to_owned();
            }
            serde_json::Value::Object(pure_value) => {
                let obj = liquid_core::to_value(&pure_value)
                    .with_context(|| "Cannot convert to value".to_string())
                    .unwrap();
                value_transformed = obj.to_owned();
            }
            _ => {
                value_transformed = liquid::model::Value::scalar(default.to_string());
            }
        }
        globals.insert(key.to_owned().into(), value_transformed);
    } else {
        globals.insert(
            key.to_owned().into(),
            liquid::model::Value::scalar(default.to_string()),
        );
    }
}
