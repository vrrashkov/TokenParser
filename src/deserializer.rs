use serde::{de, Serialize, Deserialize, Deserializer};
use serde_json::Number;
use std::default;
use std::fmt::Debug;
use std::fmt::Display;
use std::str::FromStr;
use convert_case::{Case, Casing};
use serde::de::{value, IntoDeserializer};

use crate::general;
use crate::global;


#[derive(Default, Deserialize, Debug)]
pub struct TokensConfig { 
    #[serde(default, alias = "global")]
    pub global: ConfigTokensGlobal,
    #[serde(alias = "templates")]
    pub templates: Vec<ConfigTokensTemplates>
}

#[derive(Default, Deserialize, Debug)]
pub struct ConfigTokensTemplates {
    #[serde(alias = "combine")]
    pub combine: Option<Vec<String>>,
    #[serde(alias = "settings_general")]
    pub settings_general: ConfigTemplateSettingsGeneral,
    #[serde(alias = "settings_custom")]
    pub settings_custom: ConfigTemplateSettingsCustom,
    
}

#[derive(Default, Deserialize, Debug)]
pub struct ConfigTemplateSettingsCustom {
    pub header: Option<String>,
    pub sub_header: Option<String>,
    pub class: Option<String>,
    pub class_name: Option<String>,
    pub sub_footer: Option<String>,
    pub footer: Option<String>,
    #[serde(alias = "template_type")]
    pub template_type: Vec<CustomConfigTempalteType>
}
#[derive( Deserialize, Debug)]
pub struct CustomConfigTempalteType {
    #[serde(alias = "type")]
    pub t_type: String,
    pub value: CustomConfigTempalteTypeValue,
}
#[derive(Eq, Clone, PartialEq, Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum CustomConfigTempalteTypeValue {
    Value(String),
    Values(Vec<String>)
}
#[derive(Default, Deserialize, Debug)]
pub struct ConfigTokensGlobal {
    #[serde(alias = "figma_variables_source_paths")]
    pub figma_variables_source_paths: Option<Vec<ConfigTokensGlobalOtherPath>>,
    #[serde(alias = "figma_studio_source_paths")]
    pub figma_studio_source_paths: Option<Vec<ConfigTokensGlobalOtherPath>>,
    #[serde(alias = "figma_output_paths")]
    pub figma_output_paths: Vec<ConfigTokensGlobalOtherPath>,
    #[serde(alias = "output_paths")]
    pub output_paths: Vec<ConfigTokensGlobalOtherPath>,
    #[serde(alias = "style_output_path")]
    pub style_output_path: String
}
#[derive(Default, Deserialize, Debug)]
pub struct ConfigTokensGlobalOtherPath {
    #[serde(alias = "combine")]
    pub combine: ConfigTokensGlobalOtherPathCombine
}

#[derive(Default, Deserialize, Debug)]
pub struct ConfigTokensGlobalOtherPathCombine {
    #[serde(alias = "file_name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub file_name: Option<String>,
    #[serde(alias = "files")]
    pub files: Vec<String>
}
#[derive(Default, Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename_all="lowercase")]
pub enum ConfigGlobalType {
    #[default]
    Default
}

#[derive(Default, Deserialize, Debug)]
pub struct ConfigTemplateSettingsGeneral {
    pub generate_file_path: String,
    #[serde(alias = "file_name")]
    pub file_name: ConfigTemplateSettingsGeneralFileName,
}

#[derive(Default, Deserialize, Debug)]
pub struct ConfigTemplateSettingsGeneralFileName {
    #[serde(alias = "format")]
    pub format: Option<String>,
    #[serde(alias = "extension")]
    pub extension: String,
    #[serde(alias = "case")]
    pub case: Option<String>
}

#[derive(Default, Deserialize, Debug)]
pub struct ConfigTemplateSettingsJetpackCompose {
    pub object_name_prefix: Option<String>,
    pub package_name: Option<String>
}

#[derive(Default, Deserialize, Debug)]
pub struct ConfigTemplateSettingsSwiftUI {
    pub class_name_prefix: Option<String>
}

#[derive(Eq, Clone, PartialEq, Serialize, Deserialize, Default, Debug)]
#[serde(untagged)]
pub enum TokenDataType {
    Value(serde_json::Value),
    Values(Vec<serde_json::Value>),
    #[default]
    None
    // typography { value: TokenDataTypeTypographyValue },
    // boxShadow { value: BoxShadowData},
    // composition { value: TokenDataTypeCompositionValue },
    // pure_value { value: String },
    // #[default]
    // #[serde(other)]
    // Unknown
}

#[derive(Eq, PartialEq, Serialize, Clone, Deserialize, Debug)]
pub struct TokenDataTypeCompositionValue { 
    #[serde(default)]
    pub horizontalPadding: Option<String>,
    #[serde(default)]
    pub verticalPadding: Option<String>,
    #[serde(default)]
    pub itemSpacing: Option<String>,
    #[serde(default)]
    pub paddingBottom: Option<String>,
    #[serde(default)]
    pub paddingTop: Option<String>,
    #[serde(default)]
    pub paddingLeft: Option<String>,
    #[serde(default)]
    pub paddingRight: Option<String>,
    #[serde(default)]
    pub borderRadius: Option<String>,
    #[serde(default)]
    pub borderWidth: Option<String>,
    #[serde(default)]
    pub borderRadiusBottomLeft: Option<String>,
    #[serde(default)]
    pub borderRadiusBottomRight: Option<String>,
    #[serde(default)]
    pub borderRadiusTopLeft: Option<String>,
    #[serde(default)]
    pub borderRadiusTopRight: Option<String>,
    #[serde(default)]
    pub sizing: Option<String>,
    #[serde(default)]
    pub height: Option<String>,
    #[serde(default)]
    pub width: Option<String>
}
#[derive(Eq, Clone, PartialEq, Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum BoxShadowData {
    Shadow(TokenDataTypeBoxShadowValue),
    Shadows(Vec<TokenDataTypeBoxShadowValue>),
}

#[derive(Eq, PartialEq, Serialize, Clone, Deserialize, Debug)]
pub struct TokenDataTypeTypographyValue { 
    #[serde(default)]
    pub fontFamily: Option<String>,
    #[serde(default)]
    pub fontWeight: Option<String>,
    #[serde(default)]
    pub lineHeight: Option<String>,
    #[serde(default)]
    pub fontSize: Option<String>,
    #[serde(default)]
    pub letterSpacing: Option<String>,
    #[serde(default)]
    pub paragraphSpacing: Option<String>,
    #[serde(default)]
    pub paragraphIndent: Option<String>,
    #[serde(default)]
    pub textCase: Option<String>,
    #[serde(default)]
    pub textDecoration: Option<String>
}

#[derive(Eq, PartialEq, Serialize, Clone, Deserialize, Debug)]
pub struct TokenDataTypeColorValue { 
     pub color: TokenDataColorValue,
}
#[derive(Eq, PartialEq, Serialize, Clone, Deserialize, Debug)]
pub struct TokenDataColorValue { 
     pub hex: String,
     // v1 is 0-1
     pub v1_r: String,
     pub v1_g: String,
     pub v1_b: String, 
     pub v1_a: String,
     // v2 is 0-255
     pub v2_r: String,
     pub v2_g: String,
     pub v2_b: String, 
     pub v2_a: String,
}

#[derive(Eq, PartialEq, Serialize, Clone, Deserialize, Debug)]
pub struct TokenDataTypeBoxShadowValue { 
    pub blur: String,
    pub color: String,
    pub spread: String,
    #[serde(alias = "type")]
    pub t_type: String,
    pub x: String,
    pub y: String,
}


#[derive(Eq, Clone, PartialEq, Serialize, Deserialize, Debug)]
pub enum BoxShadowType {
    blur,
    color,
    spread,
    t_type,
    x,
    y
}

#[derive(Eq, PartialEq, Serialize, Clone, Deserialize, Debug)]
pub struct TokenDataBoxShadowValue { 
    #[serde(default)]
    pub blur: Option<String>,
    pub color: TokenDataColorValue,
    #[serde(default)]
    pub spread: Option<String>,
    #[serde(default)]
    pub x: Option<String>,
    #[serde(default)]
    pub y: Option<String>,
}

impl TokensConfig {
    pub fn formatted_class_name(&self, style_name: &str, settings: &ConfigTokensTemplates, file_name: &str) -> Option<String> { 
        let settings_custom =  &settings.settings_custom;
        if let Some(class_name) =  &settings_custom.class_name {
            let mut file_name_formatted = file_name.to_owned();

            Self::format_class_name_templated(&mut file_name_formatted, &class_name.to_string(), file_name, style_name, &settings.settings_general);
        
            Some(file_name_formatted.to_string())
        
        }  else {
                None
        }  
    }

    pub fn format_class_name_templated(file_name_formatted: &mut String, template_text: &str, type_name: &str, style_name: &str, settings_general: &ConfigTemplateSettingsGeneral) { 
        
        file_name_formatted.replace_range(..,&template_text.replace("{type}", type_name.to_case(Case::Pascal).as_str()).replace("{style}", style_name.to_case(Case::Pascal).as_str()));

       let file_name_config= &settings_general.file_name;
        if let Some(file_name_case) = &file_name_config.case {
            file_name_formatted.replace_range(.., &file_name_formatted.to_case(general::case_from_str(file_name_case)));
        }
    }
}

#[derive(Clone, Debug)]
pub struct TemplateFieldData { 
    pub index: usize,
    pub key_full: String,
    pub key_without_index: String,
    pub full_template: String
}

#[derive(Eq, Clone, PartialEq, Serialize, Deserialize, Debug)]
pub enum TemplateFieldDefault {
    default
}
#[derive(Eq, Clone, PartialEq, Serialize, Deserialize, Debug)]
pub enum TemplateFieldVariantFontFamily {
    no_space,
    default
}
#[derive(Eq, Clone, PartialEq, Serialize, Deserialize, Debug)]
pub enum TemplateFieldVariantColor {
    rgb_r_v1,
    rgb_g_v1,
    rgb_b_v1,
    rgb_a_v1,
    rgb_r_v2,
    rgb_g_v2,
    rgb_b_v2,
    rgb_a_v2,
    hex,
}
impl TemplateFieldVariantColor {
    pub fn to_str(&self) -> &str {
      
        match self {
            TemplateFieldVariantColor::rgb_r_v1 => global::color_rgb_r_v1,
            TemplateFieldVariantColor::rgb_g_v1 => global::color_rgb_g_v1,
            TemplateFieldVariantColor::rgb_b_v1 => global::color_rgb_b_v1,
            TemplateFieldVariantColor::rgb_a_v1 => global::color_rgb_a_v1,
            TemplateFieldVariantColor::rgb_r_v2 => global::color_rgb_r_v2,
            TemplateFieldVariantColor::rgb_g_v2 => global::color_rgb_g_v2,
            TemplateFieldVariantColor::rgb_b_v2 => global::color_rgb_b_v2,
            TemplateFieldVariantColor::rgb_a_v2 => global::color_rgb_a_v2,
            TemplateFieldVariantColor::hex => global::color_hex,
        }
        
    }
} 

#[derive(Eq, Clone, PartialEq, Serialize, Deserialize, Debug)]
pub enum TemplateFieldVariantVariableName {
    upper,
    lower,
    camel,
    snake,
    kebab
} 

pub struct AvailableFields { 
    pub name: String,
    pub values: Vec<String>
}
