use ::askama::Error;
use easy_color::{ColorError, IntoRGBA};
use easy_color::{Hex, IntoHex};
use serde::de::value::MapAccessDeserializer;
use std::cell::RefCell;
use std::rc::Rc;
use std::str::FromStr;
use convert_case::{Case, Casing};

use crate::deserializer;
use crate::deserializer::BoxShadowData;
use crate::deserializer::ConfigTemplateType;
use crate::deserializer::TokenDataColorValue;
use crate::general;
use crate::askama;
use crate::utils;

#[derive(Clone, Debug, Default)]
pub struct FigmaTokenValue { 
    pub name: String,
    pub value: String,
    pub path: Vec<String>,
    pub token_type: TokenValueType
}
#[derive(Clone, Debug, Default)]
pub struct TokenValue { 
    pub name: String,
    pub value: deserializer::TokenDataType,
    pub path: Vec<String>,
    pub token_type: TokenValueType
}
#[derive(Clone, Debug, Default)]
pub struct TokenValueType { 
    pub text: String, 
    pub special: ConfigTemplateType
}

impl TokenValue { 
    // pub fn variable_name(&self, case: Case) -> String { 
    //     return format!("{}_{}",self.path.join("_"),self.name).to_case(case);
    // }
    pub fn variable_name(&self) -> String { 
        return format!("{}_{}",self.path.join("_"),self.name);
    }
}
#[derive(Clone, Debug, Default)]
pub struct TokenData { 
    // The general name it can be used for naming the file
    pub name: String,
    // Key = the token type so we can sort and extract all types where we want to
    // so we sore it in a map with duplicated keyslike MultiMap crate  
    //pub token_value: MultiMap<String, TokenValue>
    pub t_type: deserializer::ConfigTemplateType,
    pub token_value: Vec<TokenValue>
}

#[derive(Clone, Debug)]
pub struct TokenDataWrapper { 
    // The name of the file
    // It is used for separating different styles
    // For example dark/light etc..
    pub style_name: String,
    pub token_data: Vec<TokenData>
}

// pub fn values_from_type(file_data: &TokenData) -> Vec<askama::TemplateValue> {
//     let mut values: Vec<askama::TemplateValue> = Vec::new();
//     for token_value in &file_data.token_value  {
//         if let deserializer::TokenDataType::color { value } = &token_value.value {
        
//             values.push(
//                 askama::TemplateValue { 
//                     name: token_value.to_owned(), 
//                     value: value.to_owned()
//                 }
//             );
//         }
//     }
//     return values;
// }

pub fn box_shadow_to_list(pure_value: &BoxShadowData, box_shadow_values: &mut Vec<deserializer::TokenDataTypeBoxShadowValue>) {

    if let deserializer::BoxShadowData::Shadow(value) = &pure_value { 
        &box_shadow_values.push(value.clone());
    }
    
    if let deserializer::BoxShadowData::Shadows(values) = &pure_value { 
        for value in values { 
            &box_shadow_values.push(value.clone());
        }
    }

}

pub fn set_optional_global<T: ToString>(globals: &mut liquid_core::Object,  key: &str, value: Option<T>, default: &str) { 
    if let Some(val) = value {
        globals.insert(key.to_owned().into(), liquid::model::Value::scalar(val.to_string()));
    } else {
        globals.insert(key.to_owned().into(), liquid::model::Value::scalar(default.to_string()));
    }
}
// pub fn box_shadow_blur(template: &mut String, key: &String, variant_value: &deserializer::TemplateFieldDefault, pure_value: &BoxShadowData) {
//     let mut box_shadow_values: Vec<deserializer::TokenDataTypeBoxShadowValue> = Vec::new();
//     box_shadow_to_list(&pure_value, &mut box_shadow_values);
    
//     for value in &box_shadow_values { 
//         update_template(template, &value.blur.to_string(), &key);
//     }
// }
// pub fn box_shadow_x(template: &mut String, key: &String, variant_value: &deserializer::TemplateFieldDefault, pure_value: &BoxShadowData) {
//     let mut box_shadow_valeus: Vec<deserializer::TokenDataTypeBoxShadowValue> = Vec::new();
//     box_shadow_to_list(&pure_value, &mut box_shadow_valeus);
    
//     for value in &box_shadow_valeus { 
//         update_template(template, &value.x.to_string(), &key);
//     }
// }
// pub fn box_shadow_y(template: &mut String, key: &String, variant_value: &deserializer::TemplateFieldDefault, pure_value: &BoxShadowData) {
//     let mut box_shadow_valeus: Vec<deserializer::TokenDataTypeBoxShadowValue> = Vec::new();
//     box_shadow_to_list(&pure_value, &mut box_shadow_valeus);
    
//     for value in &box_shadow_valeus { 
//         update_template(template, &value.y.to_string(), &key);
//     }
// }
// pub fn box_shadow_spread(template: &mut String, key: &String, variant_value: &deserializer::TemplateFieldDefault, pure_value: &BoxShadowData) {
//     let mut box_shadow_valeus: Vec<deserializer::TokenDataTypeBoxShadowValue> = Vec::new();
//     box_shadow_to_list(&pure_value, &mut box_shadow_valeus);
    
//     for value in &box_shadow_valeus { 
//         update_template(template, &value.spread.to_string(), &key);
//     }
// }
// pub fn box_shadow_type(template: &mut String, key: &String, variant_value: &deserializer::TemplateFieldDefault, pure_value: &BoxShadowData) {
//     let mut box_shadow_valeus: Vec<deserializer::TokenDataTypeBoxShadowValue> = Vec::new();
//     box_shadow_to_list(&pure_value, &mut box_shadow_valeus);
    
//     for value in &box_shadow_valeus { 
//         update_template(template, &value.t_type.to_string(), &key);
//     }
// }
// pub fn box_shadow_color(template: &mut String, key: &String, variant_value: &deserializer::TemplateFieldVariantColor, pure_value: &BoxShadowData) {
//     let mut box_shadow_valeus: Vec<deserializer::TokenDataTypeBoxShadowValue> = Vec::new();
//     box_shadow_to_list(&pure_value, &mut box_shadow_valeus);
    
//     for value in &box_shadow_valeus { 
//         color(template, &key, &variant_value,&value.color.to_string());
//     }
// }

// pub fn format_template(template: &mut String, key: &str, value: &str) { 
//     *template = template.replacen(key, value, 1);
// }
// pub fn update_template(template: &mut String, value: &String, key: &String) { 
//     //println!("template: {}, key: {}, value: {}", &template, &key, &value);
//     *template = template.replacen(key.as_str(), value.as_str(), 1);
//     //println!("template replaced: {}, key: {}, value: {}", &template, &key, &value);
// }
// pub fn default(template: &mut String, key: &String, variant_value: &deserializer::TemplateFieldDefault, pure_value: &String) { 
//     let value = pure_value.to_string();
//     match &variant_value {
//         deserializer::TemplateFieldDefault::default => {
//             update_template(template, &value, &key);
//         },
//     }
// }
// pub fn default_option(template: &mut String, key: &String, variant_value: &deserializer::TemplateFieldDefault, pure_value: &Option<String>) { 
//     if let Some(value) = &pure_value {
//         match &variant_value {
//             deserializer::TemplateFieldDefault::default => {
//                 update_template(template, &value, &key);
//             },
//         }
//     }
// }

// pub fn default_option_number(template: &mut String, key: &String, variant_value: &deserializer::TemplateFieldDefault, pure_value: &Option<serde_json::Number>) { 
//     if let Some(value) = &pure_value {
//         match &variant_value {
//             deserializer::TemplateFieldDefault::default => {
//                 update_template(template, &value.to_string(), &key);
//             },
//         }
//     }
// }
// pub fn color(template: &mut String, key: &String, variant_value: &deserializer::TemplateFieldVariantColor, color_value: &String) { 

//     let v: Result<Hex, ColorError> = color_value.as_str().try_into();
//     match v {
//         Ok(formatted_color_value) => {
//             let rgba = formatted_color_value.to_rgba();
//             let rgba_r = rgba.red() as f32;
//             let rgba_g = rgba.green() as f32;
//             let rgba_b = rgba.blue() as f32;
//             let color_rgba_v1 = (rgba_r/255., rgba_g/255., rgba_b/255., rgba.alpha());
//             let color_rgba_v2 = (rgba.red(), rgba.green(), rgba.blue(), rgba.alpha());

//             match &variant_value {
//                 deserializer::TemplateFieldVariantColor::rgb_r_v1 => {
//                     update_template(template, &format!("{:.3}", color_rgba_v1.0), key);
//                 },
//                 deserializer::TemplateFieldVariantColor::rgb_g_v1 => {
//                     update_template(template, &format!("{:.3}", color_rgba_v1.1), key);
//                 },
//                 deserializer::TemplateFieldVariantColor::rgb_b_v1 => {
//                     update_template(template, &format!("{:.3}", color_rgba_v1.2), key);
//                 },
//                 deserializer::TemplateFieldVariantColor::rgb_a_v1 => {
//                     update_template(template, &format!("{:.3}", color_rgba_v1.3), key);
//                 },
//                 deserializer::TemplateFieldVariantColor::rgb_r_v2 => {
//                     update_template(template, &format!("{:.3}", color_rgba_v2.0), key);
//                 },
//                 deserializer::TemplateFieldVariantColor::rgb_g_v2 => {
//                     update_template(template, &format!("{:.3}", color_rgba_v2.1), key);
//                 },
//                 deserializer::TemplateFieldVariantColor::rgb_b_v2 => {
//                     update_template(template, &format!("{:.3}", color_rgba_v2.2), key);
//                 },
//                 deserializer::TemplateFieldVariantColor::rgb_a_v2 => {
//                     update_template(template, &format!("{:.3}", color_rgba_v2.3), key);
//                 },
//                 deserializer::TemplateFieldVariantColor::hex => {
//                     update_template(template, &color_value, key);
//                 },
//             }
//         },
//         Err(_) => {},
//     }
// }

// pub fn font_family(template: &mut String, pure_value: String, key: &String, variant_value: &deserializer::TemplateFieldVariantFontFamily) { 
//     match &variant_value {
//         deserializer::TemplateFieldVariantFontFamily::no_space => {
//             let mut value = format!("{}",pure_value);
//             utils::remove_white_spaces(&mut value);

//             update_template(template, &value, key);
//         },
//         deserializer::TemplateFieldVariantFontFamily::default =>  {
//             update_template(template, &pure_value, &key);
//         },
//     }
// }