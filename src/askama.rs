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

