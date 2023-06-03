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
use crate::utils;


//#[derive(Template, Debug)] // this will generate the code...
//#[template(path = "custom_template.html", escape = "none")] // using the template in this path, relative
pub struct CustomTemplate { // the name of the struct can be anything
    pub header: Option<String>,
    pub class: Option<String>,
    pub class_name: Option<String>,
    pub sub_header: Option<String>,
    pub sub_footer: Option<String>,
    pub footer: Option<String>,
    pub color_values: Option<Vec<String>>, 
    pub font_values: Option<Vec<String>>, 
    pub spacing_values: Option<Vec<String>>, 
    pub border_width_values: Option<Vec<String>>, 
    pub border_radius_values: Option<Vec<String>>, 
    pub letter_spacing_values: Option<Vec<String>>, 
    pub line_height_values: Option<Vec<String>>, 
    pub font_sizes_values: Option<Vec<String>>, 
    pub font_weights_values: Option<Vec<String>>, 
    pub font_families_values: Option<Vec<String>>, 
    pub box_shadow_values: Option<Vec<String>>, 
    pub composition_values: Option<Vec<String>>, 
}

impl CustomTemplate {
    pub fn update_template_values(&mut self, config_type: deserializer::ConfigTemplateType, mut values: Vec<String>) { 
        let mut current_values: Vec<String> = Vec::new();
        match &config_type {
            deserializer::ConfigTemplateType::color => {
                if let Some(val) = &mut self.color_values {
                    val.append(&mut values);
                } else {
                    self.color_values = Some(values);
                }
            },
            deserializer::ConfigTemplateType::typography => {
                if let Some(val) = &mut self.font_sizes_values {
                    val.append(&mut values);
                } else {
                    self.font_sizes_values = Some(values);
                }
            },
            deserializer::ConfigTemplateType::spacing => {
                if let Some(val) = &mut self.spacing_values {
                    val.append(&mut values);
                } else {
                    self.spacing_values = Some(values);
                }
            },
            deserializer::ConfigTemplateType::borderWidth => {
                if let Some(val) = &mut self.border_width_values {
                    val.append(&mut values);
                } else {
                    self.border_width_values = Some(values);
                }
            },
            deserializer::ConfigTemplateType::borderRadius => {
                if let Some(val) = &mut self.border_radius_values {
                    val.append(&mut values);
                } else {
                    self.border_radius_values = Some(values);
                }
            },
            deserializer::ConfigTemplateType::letterSpacing => {
                if let Some(val) = &mut self.letter_spacing_values {
                    val.append(&mut values);
                } else {
                    self.letter_spacing_values = Some(values);
                }
            },
            deserializer::ConfigTemplateType::lineHeights => {
                if let Some(val) = &mut self.line_height_values {
                    val.append(&mut values);
                } else {
                    self.line_height_values = Some(values);
                }
            },
            deserializer::ConfigTemplateType::fontSizes => {
                if let Some(val) = &mut self.font_sizes_values {
                    val.append(&mut values);
                } else {
                    self.font_sizes_values = Some(values);
                }
            },
            deserializer::ConfigTemplateType::fontWeights => {
                if let Some(val) = &mut self.font_weights_values {
                    val.append(&mut values);
                } else {
                    self.font_weights_values = Some(values);
                }
            },
            deserializer::ConfigTemplateType::fontFamilies => {
                if let Some(val) = &mut self.font_families_values {
                    val.append(&mut values);
                } else {
                    self.font_families_values = Some(values);
                }
            },
            deserializer::ConfigTemplateType::boxShadow => {
                if let Some(val) = &mut self.box_shadow_values {
                    val.append(&mut values);
                } else {
                    self.box_shadow_values = Some(values);
                }
            },
            deserializer::ConfigTemplateType::composition => {
                if let Some(val) = &mut self.composition_values {
                    val.append(&mut values);
                } else {
                    self.composition_values = Some(values);
                }
            },
            deserializer::ConfigTemplateType::none => {},
        }
    }

}
impl CustomTemplate {
    pub fn update_font_values(&mut self, values: Option<Vec<String>>) {
        self.font_values = values;
    }
}

#[derive(Debug)]
pub struct TemplateValue {
    pub name: TokenValue,
    pub value: String
}
#[derive(Debug)]
pub struct TemplateValueTypography {
    pub name: TokenValue,
    pub value: deserializer::TokenDataTypeTypographyValue
}
#[derive(Debug)]
pub struct TemplateValueCommon {
    pub name: TokenValue,
    pub value: String
}
#[derive(Debug)]
pub struct TemplateValueColor {
    pub name: TokenValue,
    pub value: deserializer::TokenDataTypeColorValue
}
#[derive(Debug)]
pub struct TemplateValueBoxShadow {
    pub name: TokenValue,
    pub value: Vec<deserializer::TokenDataBoxShadowValue>
}

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
        format!("{}_{}",self.path.join("_"),self.name)
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

pub fn set_box_shadow_global(globals: &mut liquid_core::Object, index: usize, data: &BoxShadowData, data_type: deserializer::BoxShadowType,  field_data: &deserializer::TemplateFieldData, template: &mut Option<String>) { 
    let mut box_shadow_values: Vec<deserializer::TokenDataTypeBoxShadowValue> = Vec::new();
    box_shadow_to_list(data, &mut box_shadow_values);
    if index+1 == box_shadow_values.len() {
        for (index, value) in box_shadow_values.iter().enumerate() { 
            let key = format!("{}-{}",field_data.key_without_index, index);
            match &data_type {
                deserializer::BoxShadowType::blur => {
                    set_global(globals, &key, value.blur.to_owned());
                },
                deserializer::BoxShadowType::color => {
                    set_global(globals, &key, value.color.to_owned());
                },
                deserializer::BoxShadowType::spread => {
                    set_global(globals, &key, value.spread.to_owned());
                },
                deserializer::BoxShadowType::t_type => {
                    set_global(globals, &key, value.t_type.to_owned());
                },
                deserializer::BoxShadowType::x => {
                    set_global(globals, &key, value.x.to_owned());
                },
                deserializer::BoxShadowType::y => {
                    set_global(globals, &key, value.y.to_owned());
                },
            }
            
        }
    } else { 
        *template = None;
    }
}

pub fn box_shadow_to_list(pure_value: &BoxShadowData, box_shadow_values: &mut Vec<deserializer::TokenDataTypeBoxShadowValue>) {

    if let deserializer::BoxShadowData::Shadow(value) = &pure_value { 
        box_shadow_values.push(value.clone());
    }
    
    if let deserializer::BoxShadowData::Shadows(values) = &pure_value { 
        for value in values { 
            box_shadow_values.push(value.clone());
        }
    }

}

pub fn set_global<T: ToString>(globals: &mut liquid_core::Object,  key: &str, value: T) { 
    globals.insert(key.to_owned().into(), liquid::model::Value::scalar(value.to_string()));
}

pub fn set_optional_global<T: ToString>(globals: &mut liquid_core::Object,  key: &str, value: Option<T>, default: &str) { 
    if let Some(val) = value {
        globals.insert(key.to_owned().into(), liquid::model::Value::scalar(val.to_string()));
    } else {
        globals.insert(key.to_owned().into(), liquid::model::Value::scalar(default.to_string()));
    }
}