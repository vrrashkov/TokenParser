use askama::Template;

use crate::{deserializer, template}; // bring trait in scope

#[derive(Template, Debug)] // this will generate the code...
#[template(path = "custom_template.html", escape = "none")] // using the template in this path, relative
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
impl<'a> CustomTemplate {
    pub fn update_font_values(&mut self, values: Option<Vec<String>>) {
        self.font_values = values;
    }
}

#[derive(Debug)]
pub struct TemplateValue {
    pub name: template::TokenValue,
    pub value: String
}
#[derive(Debug)]
pub struct TemplateValueTypography {
    pub name: template::TokenValue,
    pub value: deserializer::TokenDataTypeTypographyValue
}
#[derive(Debug)]
pub struct TemplateValueCommon {
    pub name: template::TokenValue,
    pub value: String
}
#[derive(Debug)]
pub struct TemplateValueColor {
    pub name: template::TokenValue,
    pub value: deserializer::TokenDataTypeColorValue
}
#[derive(Debug)]
pub struct TemplateValueBoxShadow {
    pub name: template::TokenValue,
    pub value: Vec<deserializer::TokenDataBoxShadowValue>
}

