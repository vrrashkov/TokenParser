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

#[derive(Default, Deserialize, Debug)]
pub struct ConfigTokensGlobal {
    #[serde(alias = "figma_variables_source_paths")]
    pub figma_variables_source_paths: Vec<String>,
    #[serde(alias = "figma_studio_source_paths")]
    pub figma_studio_source_paths: Vec<String>,
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
    pub combine: Vec<String>
}

#[derive(Default, Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename_all="lowercase")]
pub enum ConfigGlobalType {
    #[default]
    Default
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
    paragraphSpacing(String),
    paragraphIndent(String),
    textCase(String),
    textDecoration(String),
    lineHeights(String),
    fontSizes(String),
    fontWeights(String),
    fontFamilies(String),
    composition(String),
    boxShadow(Vec<String>),
    text(String),
    number(String),
    boolean(String),
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

    paragraphSpacing,
    paragraphIndent,
    textCase,
    textDecoration,

    lineHeights,
    fontSizes,
    fontWeights,
    fontFamilies,
    boxShadow,
    composition,
    
    text,
    number,
    boolean,
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

#[derive(Eq, Clone, PartialEq, Serialize, Deserialize, Default, Debug)]
#[serde(tag = "type")]
pub enum TokenDataType {
    typography { value: TokenDataTypeTypographyValue },
    boxShadow { value: BoxShadowData},
    composition { value: TokenDataTypeCompositionValue },
    #[serde(alias = "color", alias = "borderWidth", alias = "sizing", alias = "spacing", alias = "border_radius",
    alias = "opacity", alias = "fontFamilies", alias = "fontWeights", alias = "fontSizes", alias = "lineHeights",
    alias = "letterSpacing", alias = "paragraphSpacing", alias = "paragraphIndent", alias = "textCase",
    alias = "textDecoration ", alias = "asset", alias = "dimension", alias = "border",
    alias = "text", alias = "number", alias = "boolean")]
    pure_value { value: String },
    #[default]
    #[serde(other)]
    Unknown
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
    pub index: Option<usize>,
    // pub name: TemplateField,
    pub key_full: String,
    pub key_without_index: String,
    pub special: TemplateField,
    pub full_template: String
}

#[derive(Eq, Clone, PartialEq, Default, Debug)]
pub enum TemplateField {
    variable_name,
    color,
    font_family,
    font_size,
    font_weight,
    line_height,
    horizontal_padding,
    vertical_padding,
    spacing,
    paragraph_spacing,
    paragraph_indent,
    text_case,
    text_decoration,
    padding_bottom,
    padding_top,
    padding_left,
    padding_right,
    sizing,
    height,
    width,
    border_radius,
    border_width,
    border_radius_bottom_left,
    border_radius_bottom_right,
    border_radius_top_left,
    border_radius_top_right,
    blur,
    spread,
    t_type,
    x,
    description,
    y,
    text,
    number,
    boolean,
    #[default]
    None
}

impl TemplateField {
    pub fn from_str(input: &str) -> TemplateField {
      
        match input {
            global::field_variable_name => TemplateField::variable_name,
            global::field_color => TemplateField::color,
            global::field_value_font_family => TemplateField::font_family,
            global::field_value_font_size => TemplateField::font_size,
            global::field_value_font_weight => TemplateField::font_weight,
            global::field_value_spacing => TemplateField::spacing,
            global::field_value_paragraph_spacing => TemplateField::paragraph_spacing,
            global::field_value_paragraph_indent => TemplateField::paragraph_indent,
            global::field_value_text_case => TemplateField::text_case,
            global::field_value_text_decoration => TemplateField::text_decoration,
            global::field_value_line_height => TemplateField::line_height,
            global::field_value_horizontal_padding => TemplateField::horizontal_padding,
            global::field_value_vertical_padding => TemplateField::vertical_padding,
            global::field_value_padding_bottom => TemplateField::padding_bottom,
            global::field_value_padding_top => TemplateField::padding_top,
            global::field_value_padding_left => TemplateField::padding_left,
            global::field_value_padding_right => TemplateField::padding_right,
            global::field_value_sizing => TemplateField::sizing,
            global::field_value_height => TemplateField::height,
            global::field_value_width => TemplateField::width,
            global::field_value_border_radius => TemplateField::border_radius,
            global::field_value_border_width => TemplateField::border_width,
            global::field_value_border_radius_bottom_left => TemplateField::border_radius_bottom_left,
            global::field_value_border_radius_bottom_right => TemplateField::border_radius_bottom_right,
            global::field_value_border_radius_top_left => TemplateField::border_radius_top_left,
            global::field_value_border_radius_top_right => TemplateField::border_radius_top_right,
            global::field_value_blur => TemplateField::blur,
            global::field_value_spread => TemplateField::spread,
            global::field_value_type => TemplateField::t_type,
            global::field_value_x => TemplateField::x,
            global::field_value_y => TemplateField::y,
            global::field_description => TemplateField::description,
            global::field_value_text => TemplateField::text,
            global::field_value_number => TemplateField::number,
            global::field_value_boolean => TemplateField::boolean,
            _ => TemplateField::None
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
impl ConfigTemplateType {
    pub fn from_str(input: &str) -> ConfigTemplateType {
        match input {
            global::type_color         => ConfigTemplateType::color,
            global::type_typography    => ConfigTemplateType::typography,
            global::type_spacing       => ConfigTemplateType::spacing,
            global::type_paragraph_spacing      => ConfigTemplateType::paragraphSpacing,
            global::type_paragraph_ident        => ConfigTemplateType::paragraphIndent,
            global::type_text_case              => ConfigTemplateType::textCase,
            global::type_text_decoration        => ConfigTemplateType::textDecoration,
            global::type_spacing       => ConfigTemplateType::spacing,
            global::type_spacing       => ConfigTemplateType::spacing,
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
            global::type_color         => ConfigTemplateType::color,
            global::type_text         => ConfigTemplateType::text,
            global::type_number         => ConfigTemplateType::number,
            global::type_boolean         => ConfigTemplateType::boolean,
            _                          => ConfigTemplateType::none,
        }
    }
} 

pub struct AvailableFields { 
    pub name: String,
    pub values: Vec<String>
}

impl CustomConfigTempalteType {

    pub fn global_available_fields() -> AvailableFields { 
        return AvailableFields {
            name: global::global.to_string(),
            values: vec![global::field_variable_name.to_string(), global::field_description.to_string()]
        };
    }
    pub fn available_fields(&self) -> AvailableFields { 
        match &self {
            CustomConfigTempalteType::color(_) => AvailableFields {
                name: global::type_color.to_string(),
                values: vec![global::field_color.to_string()]
            },
            CustomConfigTempalteType::typography(_) => AvailableFields {
                name: global::type_typography.to_string(),
                values: vec![
                    global::field_value_font_family.to_string(), 
                    global::field_value_font_size.to_string(), 
                    global::field_value_font_weight.to_string(), 
                    global::field_value_spacing.to_string(),
                    global::field_value_paragraph_spacing.to_string(),
                    global::field_value_paragraph_indent.to_string(),
                    global::field_value_text_case.to_string(),
                    global::field_value_text_decoration.to_string(),
                    global::field_value_line_height.to_string()
                ]
            },
            CustomConfigTempalteType::spacing(_) => AvailableFields {
                name: global::type_spacing.to_string(),
                values: vec![global::field_value_spacing.to_string()]
            },
            CustomConfigTempalteType::borderWidth(_) => AvailableFields {
                name: global::type_borderWidth.to_string(),
                values: vec![global::field_value_border_width.to_string()]
            },
            CustomConfigTempalteType::borderRadius(_) => AvailableFields {
                name: global::type_borderRadius.to_string(),
                values: vec![global::field_value_border_radius.to_string()]
            },
            CustomConfigTempalteType::letterSpacing(_) => AvailableFields {
                name: global::type_letterSpacing.to_string(),
                values: vec![global::field_value_spacing.to_string()]
            },
            CustomConfigTempalteType::lineHeights(_) => AvailableFields {
                name: global::type_lineHeights.to_string(),
                values: vec![global::field_value_line_height.to_string()]
            },
            CustomConfigTempalteType::fontSizes(_) => AvailableFields {
                name: global::type_fontSizes.to_string(),
                values: vec![global::field_value_font_size.to_string()]
            },
            CustomConfigTempalteType::fontWeights(_) => AvailableFields {
                name: global::type_fontWeights.to_string(),
                values: vec![global::field_value_font_weight.to_string()]
            },
            CustomConfigTempalteType::fontFamilies(_) => AvailableFields {
                name: global::type_fontFamilies.to_string(),
                values: vec![global::field_value_font_family.to_string()]
            },
            CustomConfigTempalteType::boxShadow(_) => AvailableFields {
                name: global::type_boxShadow.to_string(),
                values: vec![global::field_color.to_string(), 
                global::field_value_blur.to_string(), 
                global::field_value_spread.to_string(), 
                global::field_value_type.to_string(), 
                global::field_value_x.to_string(), 
                global::field_value_y.to_string()]
            },
            CustomConfigTempalteType::composition(_) => AvailableFields {
                name: global::type_composition.to_string(),
                values: vec![global::field_value_padding_bottom.to_string(),
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
                name: global::type_unknown.to_string(),
                values: vec![]
            },
            CustomConfigTempalteType::paragraphSpacing(_) => AvailableFields {
                name: global::type_paragraph_spacing.to_string(),
                values: vec![global::field_value_paragraph_spacing.to_string(),]
            },
            CustomConfigTempalteType::paragraphIndent(_) => AvailableFields {
                name: global::type_paragraph_ident.to_string(),
                values: vec![global::field_value_paragraph_indent.to_string(),]
            },
            CustomConfigTempalteType::textCase(_) => AvailableFields {
                name: global::type_text_case.to_string(),
                values: vec![global::field_value_text_case.to_string(),]
            },
            CustomConfigTempalteType::textDecoration(_) => AvailableFields {
                name: global::type_text_decoration.to_string(),
                values: vec![global::field_value_text_decoration.to_string(),]
            },
            CustomConfigTempalteType::text(_) => AvailableFields {
                name: global::type_text.to_string(),
                values: vec![global::field_value_text.to_string()]
            },
            CustomConfigTempalteType::number(_) => AvailableFields {
                name: global::type_number.to_string(),
                values: vec![global::field_value_number.to_string()]
            },
            CustomConfigTempalteType::boolean(_) => AvailableFields {
                name: global::type_boolean.to_string(),
                values: vec![global::field_value_boolean.to_string()]
            },
        }
    }
}