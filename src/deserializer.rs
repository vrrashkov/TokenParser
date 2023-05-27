use serde::{Serialize, Deserialize, Deserializer};
use serde_json::Number;
use std::str::FromStr;
use convert_case::{Case, Casing};
use serde_this_or_that::{as_string};

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

#[derive(Default, Deserialize, Debug)]
pub struct ConfigTokensGlobal {
    #[serde(alias = "core_path")]
    pub core_path: Vec<String>,
    #[serde(alias = "style_path")]
    pub style_path: Vec<String>,
    #[serde(alias = "style_output_path")]
    pub style_output_path: String
}

#[derive(Default, Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename_all="lowercase")]
pub enum ConfigGlobalType {
    #[default]
    DEFAULT
}

#[derive(Eq, Clone, PartialEq, Default, Serialize, Deserialize, Debug)]
#[serde(tag="type", content="value")]
pub enum CustomConfigTempalteType {
    color(String),
    typography(String),
    spacing(String),
    borderWidth(String),
    borderRadius(String),
    letterSpacing(String),
    lineHeights(String),
    fontSizes(String),
    fontWeights(String),
    fontFamilies(String),
    composition(String),
    boxShadow(Vec<String>),
    #[default]
    none
}
#[derive(Eq, Clone, PartialEq, Default, Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum ConfigTemplateType {
    color,
    typography,
    spacing,
    borderWidth,
    borderRadius,
    letterSpacing,
    lineHeights,
    fontSizes,
    fontWeights,
    fontFamilies,
    boxShadow,
    composition,
    #[default]
    none
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

#[derive(Eq, Clone, PartialEq, Default, Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum TokenDataType {
    color { value: String },
    typography { value: TokenDataTypeTypographyValue },
    borderWidth { value: Number },
    sizing { value: String },
    spacing { value: Number },
    borderRadius { value: Number },
    boxShadow { value: BoxShadowData},
    opacity { value: String },
    fontFamilies { value: String },
    fontWeights { value: Number },
    fontSizes { value: Number },
    lineHeights { value: Number },
    letterSpacing { value: Number },
    paragraphSpacing { value: Number },
    textCase { value: String },
    textDecoration { value: String },
    asset { value: String },
    composition { value: TokenDataTypeCompositionValue },
    dimension { value: String },
    border { value: String },
    #[default]
    NONE
}
#[derive(Eq, PartialEq, Serialize, Clone, Deserialize, Debug)]
pub struct TokenDataTypeCompositionValue { 
    pub horizontalPadding: Option<Number>,
    pub verticalPadding: Option<Number>,
    pub itemSpacing: Option<Number>,
    pub paddingBottom: Option<Number>,
    pub paddingTop: Option<Number>,
    pub paddingLeft: Option<Number>,
    pub paddingRight: Option<Number>,
    pub borderRadius: Option<Number>,
    pub borderWidth: Option<Number>,
    pub borderRadiusBottomLeft: Option<Number>,
    pub borderRadiusBottomRight: Option<Number>,
    pub borderRadiusTopLeft: Option<Number>,
    pub borderRadiusTopRight: Option<Number>,
    pub sizing: Option<Number>,
    pub height: Option<Number>,
    pub width: Option<Number>,
}
#[derive(Eq, Clone, PartialEq, Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum BoxShadowData {
    Shadow(TokenDataTypeBoxShadowValue),
    Shadows(Vec<TokenDataTypeBoxShadowValue>),
}

#[derive(Eq, PartialEq, Serialize, Clone, Deserialize, Debug)]
pub struct TokenDataTypeTypographyValue { 
    pub fontFamily: String,
    #[serde(deserialize_with = "as_string")]
    pub fontWeight: String,
    pub lineHeight: Number,
    pub fontSize: Number,
    pub letterSpacing: Number,
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
    pub blur: Number,
    pub color: String,
    pub spread: Number,
    #[serde(alias = "type")]
    pub t_type: String,
    pub x: Number,
    pub y: Number,
}
#[derive(Eq, PartialEq, Serialize, Clone, Deserialize, Debug)]
pub struct TokenDataBoxShadowValue { 
    pub blur: Number,
    pub color: TokenDataColorValue,
    pub spread: Number,
    pub x: Number,
    pub y: Number,
}

impl TokensConfig {
    pub fn formatted_class_name(&self, style_name: &str, settings: &ConfigTokensTemplates, file_name: &str) -> Option<String> { 
        let settings_custom =  &settings.settings_custom;
        if let Some(class_name) =  &settings_custom.class_name {
            let mut file_name_formatted = file_name.to_owned();

            Self::format_class_name_templated(&mut file_name_formatted, &class_name.to_string(), &file_name, &style_name, &settings.settings_general);
        
            return Some(file_name_formatted.to_string());
        
        }  else {
                return None;
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TemplateFieldData { 
    pub index: Option<usize>,
    pub name: TemplateField
}

#[derive(Eq, Clone, PartialEq, Default, Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum TemplateField {
    variable_name { value: TemplateFieldVariantVariableName },
    color { value: TemplateFieldVariantColor },
    font_family { value: TemplateFieldVariantFontFamily },
    font_size { value: TemplateFieldDefault },
    font_weight { value: TemplateFieldDefault },
    line_height { value: TemplateFieldDefault },
    horizontal_padding { value: TemplateFieldDefault },
    vertical_padding { value: TemplateFieldDefault },
    spacing { value: TemplateFieldDefault },
    padding_bottom { value: TemplateFieldDefault },
    padding_top { value: TemplateFieldDefault },
    padding_left { value: TemplateFieldDefault },
    padding_right { value: TemplateFieldDefault },
    sizing { value: TemplateFieldDefault },
    height { value: TemplateFieldDefault },
    width { value: TemplateFieldDefault },
    border_radius { value: TemplateFieldDefault },
    border_width { value: TemplateFieldDefault },
    border_radius_bottom_left { value: TemplateFieldDefault },
    border_radius_bottom_right { value: TemplateFieldDefault },
    border_radius_top_left { value: TemplateFieldDefault },
    border_radius_top_right { value: TemplateFieldDefault },
    blur { value: TemplateFieldDefault },
    spread { value: TemplateFieldDefault },
    #[serde(alias = "type")]
    t_type { value: TemplateFieldDefault },
    x { value: TemplateFieldDefault },
    y { value: TemplateFieldDefault },
    #[default]
    NONE
}

impl TemplateField {
    pub fn as_str(&self) -> &str { 
        match &self {
            TemplateField::variable_name { value } => global::field_variable_name,
            TemplateField::color { value } => global::field_color,
            TemplateField::font_family { value } => global::field_value_font_family,
            TemplateField::font_size { value } => global::field_value_font_size,
            TemplateField::font_weight { value } => global::field_value_font_weight,
            TemplateField::spacing { value } => global::field_value_spacing,
            //TemplateField::letter_spacing { value } => global::fiel,
            TemplateField::line_height { value } => global::field_value_line_height,
            TemplateField::horizontal_padding { value } => global::field_value_horizontal_padding,
            TemplateField::vertical_padding { value } => global::field_value_vertical_padding,
            //TemplateField::item_spacing { value } => global::field_value_item_spacing,
            TemplateField::padding_bottom { value } => global::field_value_padding_bottom,
            TemplateField::padding_top { value } => global::field_value_padding_top,
            TemplateField::padding_left { value } => global::field_value_padding_left,
            TemplateField::padding_right { value } => global::field_value_padding_right,
            TemplateField::sizing { value } => global::field_value_sizing,
            TemplateField::height { value } => global::field_value_height,
            TemplateField::width { value } => global::field_value_width,
            TemplateField::border_radius { value } => global::field_value_border_radius,
            TemplateField::border_width { value } => global::field_value_border_width,
            TemplateField::border_radius_bottom_left { value } => global::field_value_border_radius_bottom_left,
            TemplateField::border_radius_bottom_right { value } => global::field_value_border_radius_bottom_right,
            TemplateField::border_radius_top_left { value } => global::field_value_border_radius_top_left,
            TemplateField::border_radius_top_right { value } => global::field_value_border_radius_top_right,
            TemplateField::blur { value } => global::field_value_blur,
            TemplateField::spread { value } => global::field_value_spread,
            TemplateField::t_type { value } => global::field_value_type,
            TemplateField::x { value } => global::field_value_x,
            TemplateField::y { value } => global::field_value_y,
            TemplateField::NONE => todo!(),
        }
    }
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

#[derive(Eq, Clone, PartialEq, Serialize, Deserialize, Debug)]
pub enum TemplateFieldVariantVariableName {
    upper,
    lower,
    camel,
    snake,
    kebab
}

impl ConfigTemplateType {
    pub fn from_str(input: &str) -> ConfigTemplateType {
        
        let value = match input {
            global::type_color         => ConfigTemplateType::color,
            global::type_typography    => ConfigTemplateType::typography,
            global::type_spacing       => ConfigTemplateType::spacing,
            global::type_borderWidth   => ConfigTemplateType::borderWidth,
            global::type_borderRadius  => ConfigTemplateType::borderRadius,
            global::type_letterSpacing => ConfigTemplateType::letterSpacing,
            global::type_composition   => ConfigTemplateType::composition,
            global::type_lineHeights   => ConfigTemplateType::lineHeights,
            global::type_fontSizes     => ConfigTemplateType::fontSizes,
            global::type_fontWeights   => ConfigTemplateType::fontWeights,
            global::type_fontFamilies  => ConfigTemplateType::fontFamilies,
            global::type_boxShadow     => ConfigTemplateType::boxShadow,
            _                          => ConfigTemplateType::none,
        };

        return value
    }
} 

pub struct AvailableFields { 
    pub name: String,
    pub values: Vec<String>
}

impl CustomConfigTempalteType {
    pub fn available_fields(&self) -> AvailableFields { 
        match &self {
            CustomConfigTempalteType::color(_) => AvailableFields {
                name: global::type_color.to_string(),
                values: vec![global::field_variable_name.to_string(), global::field_color.to_string()]
            },
            CustomConfigTempalteType::typography(_) => AvailableFields {
                name: global::type_color.to_string(),
                values: vec![
                    global::field_variable_name.to_string(), 
                    global::field_value_font_family.to_string(), 
                    global::field_value_font_size.to_string(), 
                    global::field_value_font_weight.to_string(), 
                    global::field_value_spacing.to_string(),
                    global::field_value_line_height.to_string()
                ]
            },
            CustomConfigTempalteType::spacing(_) => AvailableFields {
                name: global::type_color.to_string(),
                values: vec![global::field_variable_name.to_string(), global::field_value_spacing.to_string()]
            },
            CustomConfigTempalteType::borderWidth(_) => AvailableFields {
                name: global::type_color.to_string(),
                values: vec![global::field_variable_name.to_string(), global::field_value_border_width.to_string()]
            },
            CustomConfigTempalteType::borderRadius(_) => AvailableFields {
                name: global::type_color.to_string(),
                values: vec![global::field_variable_name.to_string(), global::field_value_border_radius.to_string()]
            },
            CustomConfigTempalteType::letterSpacing(_) => AvailableFields {
                name: global::type_color.to_string(),
                values: vec![global::field_variable_name.to_string(), global::field_value_spacing.to_string()]
            },
            CustomConfigTempalteType::lineHeights(_) => AvailableFields {
                name: global::type_color.to_string(),
                values: vec![global::field_variable_name.to_string(), global::field_value_line_height.to_string()]
            },
            CustomConfigTempalteType::fontSizes(_) => AvailableFields {
                name: global::type_color.to_string(),
                values: vec![global::field_variable_name.to_string(), global::field_value_font_size.to_string()]
            },
            CustomConfigTempalteType::fontWeights(_) => AvailableFields {
                name: global::type_color.to_string(),
                values: vec![global::field_variable_name.to_string(), global::field_value_font_weight.to_string()]
            },
            CustomConfigTempalteType::fontFamilies(_) => AvailableFields {
                name: global::type_color.to_string(),
                values: vec![global::field_variable_name.to_string(), global::field_value_font_family.to_string()]
            },
            CustomConfigTempalteType::boxShadow(_) => AvailableFields {
                name: global::type_color.to_string(),
                values: vec![global::field_variable_name.to_string(), 
                global::field_color.to_string(), 
                global::field_value_blur.to_string(), 
                global::field_value_spread.to_string(), 
                global::field_value_type.to_string(), 
                global::field_value_x.to_string(), 
                global::field_value_y.to_string()]
            },
            CustomConfigTempalteType::composition(_) => AvailableFields {
                name: global::type_color.to_string(),
                values: vec![global::field_variable_name.to_string(),
                global::field_value_padding_bottom.to_string(),
                global::field_value_padding_top.to_string(),
                global::field_value_padding_left.to_string(),
                global::field_value_padding_right.to_string(),
                global::field_value_sizing.to_string(),
                global::field_value_width.to_string(),
                global::field_value_height.to_string(),
                global::field_value_border_radius.to_string(),
                global::field_value_border_width.to_string(),
                global::field_value_border_radius_bottom_left.to_string(),
                global::field_value_border_radius_bottom_right.to_string(),
                global::field_value_border_radius_top_left.to_string(),
                global::field_value_border_radius_top_right.to_string(),
                global::field_value_vertical_padding.to_string(),
                global::field_value_horizontal_padding.to_string(),
                global::field_value_item_spacing.to_string()]
            },
            CustomConfigTempalteType::none => AvailableFields {
                name: global::type_color.to_string(),
                values: vec![global::field_variable_name.to_string(),]
            },
        }
    }
}