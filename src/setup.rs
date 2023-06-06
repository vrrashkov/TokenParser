use liquid::ValueView;
use serde::de::value::MapAccessDeserializer;
use std::cell::RefCell;
use std::rc::Rc;
use std::str::FromStr;
use convert_case::{Case, Casing};

use crate::deserializer;
use crate::deserializer::AvailableFields;
use crate::deserializer::ConfigTemplateType;
use crate::deserializer::TemplateField;
use crate::general;
use crate::template;
use crate::template::CustomTemplate;
use crate::template::TokenValue;
use crate::utils;

use crate::filters;

pub fn init(token_config: &deserializer::TokensConfig) { 

    let token_data_wrapper_list = general::generate_tokens(token_config);
    // different styles light/dark
    for token_data_wrapper in token_data_wrapper_list.iter() {
        for template_config in token_config.templates.iter() {
        
            let file_data_name = "custom";
        
            let template_content = template_content_custom(template_config, token_data_wrapper, token_config, file_data_name, &token_data_wrapper.token_data);
            
            general::create_template(template_config, &token_data_wrapper.style_name, file_data_name, &template_content);
            
        }
    }
}

fn template_content_custom(
    template_config: &deserializer::ConfigTokensTemplates, 
    token_data_wrapper: &template::TokenDataWrapper, 
    token_config: &deserializer::TokensConfig,
    file_data_name: &str,
    file_data_list: &[template::TokenData]
) -> Option<String>{ 
    let mut template_content: Option<String> = Default::default();

    let custom_template = &template_config.settings_custom;
    let mut current_template: CustomTemplate = CustomTemplate { 
        header: custom_template.header.to_owned(),
        footer: custom_template.footer.to_owned(),
        sub_header: custom_template.sub_header.to_owned(),
        sub_footer: custom_template.sub_footer.to_owned(),
        class: custom_template.class.to_owned(),
        class_name: token_config.formatted_class_name(&token_data_wrapper.style_name, template_config, file_data_name),
        color_values: None,
        font_values: None,
        spacing_values: None,
        border_width_values: None,
        border_radius_values: None, 
        letter_spacing_values: None, 
        paragraph_spacing_values: None, 
        paragraph_indent_values: None, 
        text_case_values: None, 
        text_decoration_values: None, 
        line_height_values: None, 
        font_sizes_values: None, 
        font_weights_values: None, 
        font_families_values: None, 
        box_shadow_values: None, 
        composition_values: None, 
    }; 

    for template in &custom_template.template_type {
        //dbg!(&custom_template.template_type);
        let available_fields = template.available_fields();
        match &template {
            deserializer::CustomConfigTempalteType::color(value) => {
                template_update_list_values(file_data_list, &mut current_template, deserializer::ConfigTemplateType::color, value, &available_fields);
            },
            deserializer::CustomConfigTempalteType::typography(value) => {
                template_update_list_values(file_data_list, &mut current_template, deserializer::ConfigTemplateType::typography, value, &available_fields);
            },
            deserializer::CustomConfigTempalteType::spacing(value) => {
                template_update_list_values(file_data_list, &mut current_template, deserializer::ConfigTemplateType::spacing, value, &available_fields);
            },
            deserializer::CustomConfigTempalteType::borderWidth(value) => {
                template_update_list_values(file_data_list, &mut current_template, deserializer::ConfigTemplateType::borderWidth, value, &available_fields);
            },
            deserializer::CustomConfigTempalteType::borderRadius(value) => {
                template_update_list_values(file_data_list, &mut current_template, deserializer::ConfigTemplateType::borderRadius, value, &available_fields);
            },
            deserializer::CustomConfigTempalteType::letterSpacing(value) => {
                template_update_list_values(file_data_list, &mut current_template, deserializer::ConfigTemplateType::letterSpacing, value, &available_fields);
            },
            deserializer::CustomConfigTempalteType::paragraphSpacing(value) => {
                template_update_list_values(file_data_list, &mut current_template, deserializer::ConfigTemplateType::paragraphSpacing, value, &available_fields);
            },
            deserializer::CustomConfigTempalteType::paragraphIndent(value) => {
                template_update_list_values(file_data_list, &mut current_template, deserializer::ConfigTemplateType::paragraphIndent, value, &available_fields);
            },
            deserializer::CustomConfigTempalteType::textCase(value) => {
                template_update_list_values(file_data_list, &mut current_template, deserializer::ConfigTemplateType::textCase, value, &available_fields);
            },
            deserializer::CustomConfigTempalteType::textDecoration(value) => {
                template_update_list_values(file_data_list, &mut current_template, deserializer::ConfigTemplateType::textDecoration, value, &available_fields);
            },
            deserializer::CustomConfigTempalteType::lineHeights(value) => {
                template_update_list_values(file_data_list, &mut current_template, deserializer::ConfigTemplateType::lineHeights, value, &available_fields);
            },
            deserializer::CustomConfigTempalteType::fontSizes(value) => {
                template_update_list_values(file_data_list, &mut current_template, deserializer::ConfigTemplateType::fontSizes, value, &available_fields);
            },
            deserializer::CustomConfigTempalteType::fontWeights(value) => {
                template_update_list_values(file_data_list, &mut current_template, deserializer::ConfigTemplateType::fontWeights, value, &available_fields);
            },
            deserializer::CustomConfigTempalteType::fontFamilies(value) => {
                template_update_list_values(file_data_list, &mut current_template, deserializer::ConfigTemplateType::fontFamilies, value, &available_fields);
            },
            deserializer::CustomConfigTempalteType::boxShadow(value) => {
                template_list_replaced_values(file_data_list, &mut current_template, deserializer::ConfigTemplateType::boxShadow,value, &available_fields);
            },
            deserializer::CustomConfigTempalteType::composition(value) => {
                template_update_list_values(file_data_list, &mut current_template, deserializer::ConfigTemplateType::composition, value, &available_fields);
            },
            deserializer::CustomConfigTempalteType::none => todo!(),
        }
    }
    let template = liquid::ParserBuilder::with_stdlib()
    .build().unwrap()
    .parse_file("templates/liquid_template.html").unwrap();

    let mut globals = liquid::object!({
        "header": current_template.header,
        "footer": current_template.footer,
        "sub_header": current_template.sub_header,
        "sub_footer": current_template.sub_footer,
        "class": current_template.class,
        "class_name": current_template.class_name,
        "color_values": current_template.color_values,
        "font_values": current_template.font_values,
        "spacing_values": current_template.spacing_values,
        "border_width_values": current_template.border_width_values,
        "border_radius_values": current_template.border_radius_values,
        "letter_spacing_values": current_template.letter_spacing_values,
        "line_height_values": current_template.line_height_values,
        "font_sizes_values": current_template.font_sizes_values,
        "font_weights_values": current_template.font_weights_values,
        "font_families_values": current_template.font_families_values,
        "box_shadow_values": current_template.box_shadow_values,
        "composition_values": current_template.composition_values,
        "paragraph_spacing_values": current_template.paragraph_spacing_values,
        "paragraph_indent_values": current_template.paragraph_indent_values,
        "text_case_values": current_template.text_case_values,
        "text_decoration_values": current_template.text_decoration_values,
    });
    
    Some(template.render(&globals).unwrap())
}

fn template_update_list_values(file_data_list: &[template::TokenData], current_template: &mut CustomTemplate, config_type: deserializer::ConfigTemplateType, value: &String, available_fields: &AvailableFields) { 
    let pure_values = template_pure_values(file_data_list, config_type.to_owned());
    let mut current = template_replaced_values_single(value, &pure_values, available_fields);
    current_template.update_template_values(config_type, current);
}

fn template_list_replaced_values(file_data_list: &[template::TokenData], current_template: &mut CustomTemplate, config_type: deserializer::ConfigTemplateType, templates: &[String], available_fields: &AvailableFields) { 
    let mut values_content:Vec<String> = Vec::new();
    let pure_values = template_pure_values(file_data_list, config_type.to_owned());
    for (index, template) in templates.iter().enumerate() { 
        let current = template_replaced_values(index, template, &pure_values, available_fields);    
        current_template.update_template_values(config_type.to_owned(), current);
    }
}

fn template_replaced_values_single(template: &String, pure_values: &Vec<template::TokenValue>, available_fields: &AvailableFields) -> Vec<String>  { 
    template_replaced_values(0, template, pure_values, available_fields)
}

fn template_replaced_values(index: usize, template: &String, pure_values: &Vec<template::TokenValue>, available_fields: &AvailableFields) -> Vec<String> { 
    let template_fields= template_as_values(template);

    let mut values_content:Vec<String> = Vec::new();
    for content in pure_values { 
        let mut globals = liquid::object!({});
        let current_optional = template_set_values(index, content, template, available_fields, &template_fields, &mut globals);

        if let Some(current) = current_optional {
            //print!("current: {}", &current);
            let template_parsed = liquid::ParserBuilder::with_stdlib()
            .filter(filters::remove_space::RemoveSpace)
            .filter(filters::color::Color)
            .filter(filters::case::CamelCase)
            .filter(filters::case::PascalCase)
            .filter(filters::case::KebabCase)
            .filter(filters::optional::Optional)
            .build().unwrap()
            .parse(&current).unwrap();

            let output = template_parsed.render(&globals).unwrap();
            //dbg!(&globals);
            if !output.is_empty() {
                values_content.push(output);
            }
            
        }
    }
    values_content
}

fn template_pure_values(file_data_list: &[template::TokenData], template_type: deserializer::ConfigTemplateType) -> Vec<template::TokenValue>{
    if let Some(file_data) = file_data_list.iter().find(|f| f.t_type == template_type) {
        //let pure_values = template::values_from_type(&file_data);
        return file_data.token_value.to_owned();
    }

    Vec::new()
}

pub fn template_set_values(index: usize, data: &template::TokenValue, pure_template: &String, available_fields: &AvailableFields, fields: &Vec<deserializer::TemplateFieldData>, globals: &mut liquid::Object) -> Option<String> { 
    let token_value = data;
    let mut template: Option<String> = Some(pure_template.to_string());
    for field_data in fields {
        let field_name = field_data.key_full.as_str();

        if available_fields.values.contains(&field_data.key_without_index.to_string()) {
         match &field_data.special {
            TemplateField::variable_name => {
                template::set_global(globals, field_name, token_value.variable_name());
            },
            TemplateField::spacing => {
                if let deserializer::TokenDataType::spacing { value } = &token_value.value {
                    template::set_global(globals, field_name, value);
                }
                if let deserializer::TokenDataType::typography { value } = &token_value.value {
                    template::set_optional_global(globals, field_name, value.letterSpacing.to_owned(), "");
                }
                if let deserializer::TokenDataType::letterSpacing { value } = &token_value.value {
                    template::set_global(globals, field_name, value);
                }
                if let deserializer::TokenDataType::composition { value } = &token_value.value {
                    template::set_optional_global(globals, field_name, value.itemSpacing.to_owned(), "");
                }
            },
            TemplateField::font_family => {
                if let deserializer::TokenDataType::fontFamilies { value } = &token_value.value {
                    template::set_global(globals, field_name, value);
                }
                if let deserializer::TokenDataType::typography { value } = &token_value.value {
                    let typography_value = value;
                    template::set_optional_global(globals, field_name, typography_value.fontFamily.to_owned(), "");
                }
            },
            TemplateField::color => {
                if let deserializer::TokenDataType::color { value } = &token_value.value {
                    template::set_global(globals, field_name, value);
                }
                if let deserializer::TokenDataType::boxShadow { value } = &token_value.value {
                    template::set_box_shadow_global(globals, index, value, deserializer::BoxShadowType::color, field_data, &mut template);
                }
            }
            TemplateField::font_size => {
                if let deserializer::TokenDataType::typography { value } = &token_value.value {
                    template::set_optional_global(globals, field_name, value.fontFamily.to_owned(), "");
                }
                if let deserializer::TokenDataType::fontSizes { value } = &token_value.value {
                    template::set_global(globals, field_name, value);
                }
            },
            TemplateField::font_weight => {
                if let deserializer::TokenDataType::typography { value } = &token_value.value {
                    template::set_optional_global(globals, field_name, value.fontWeight.to_owned(), "");
                }
                if let deserializer::TokenDataType::fontWeights { value } = &token_value.value {
                    template::set_global(globals, field_name, value);
                }
            },
            TemplateField::line_height => {
                if let deserializer::TokenDataType::typography { value } = &token_value.value {
                    template::set_optional_global(globals, field_name, value.lineHeight.to_owned(), "");
                }
                if let deserializer::TokenDataType::lineHeights { value } = &token_value.value {
                    template::set_global(globals, field_name, value);
                }
            },
            TemplateField::horizontal_padding => {
                if let deserializer::TokenDataType::composition { value } = &token_value.value {
                    template::set_optional_global(globals, field_name, value.horizontalPadding.to_owned(), "");
                }
            },
            TemplateField::vertical_padding => {
                if let deserializer::TokenDataType::composition { value } = &token_value.value {
                    template::set_optional_global(globals, field_name, value.verticalPadding.to_owned(), "");
                }
            },
            TemplateField::padding_bottom => {
                if let deserializer::TokenDataType::composition { value } = &token_value.value {
                    template::set_optional_global(globals, field_name, value.paddingBottom.to_owned(), "");
                }
            },
            TemplateField::padding_top => {
                if let deserializer::TokenDataType::composition { value } = &token_value.value {
                    template::set_optional_global(globals, field_name, value.paddingTop.to_owned(), "");
                }
            },
            TemplateField::padding_left => {
                if let deserializer::TokenDataType::composition { value } = &token_value.value {
                    template::set_optional_global(globals, field_name, value.paddingLeft.to_owned(), "");
                }
            },
            TemplateField::padding_right => {
                if let deserializer::TokenDataType::composition { value } = &token_value.value {
                    template::set_optional_global(globals, field_name, value.paddingRight.to_owned(), "");
                }
            },
            TemplateField::sizing => {
                if let deserializer::TokenDataType::composition { value } = &token_value.value {
                    template::set_optional_global(globals, field_name, value.sizing.to_owned(), "");
                }
            },
            TemplateField::height => {
                if let deserializer::TokenDataType::composition { value } = &token_value.value {
                    template::set_optional_global(globals, field_name, value.height.to_owned(), "");
                }
            },
            TemplateField::width => {
                if let deserializer::TokenDataType::composition { value } = &token_value.value {
                    template::set_optional_global(globals, field_name, value.width.to_owned(), "");
                }
            },
            TemplateField::border_radius => {
                if let deserializer::TokenDataType::borderRadius { value } = &token_value.value {
                    template::set_global(globals, field_name, value);
                }
                if let deserializer::TokenDataType::composition { value } = &token_value.value {
                    template::set_optional_global(globals, field_name, value.borderRadius.to_owned(), "");
                }
            },
            TemplateField::border_width => {
                if let deserializer::TokenDataType::borderWidth { value } = &token_value.value {
                    template::set_global(globals, field_name, value);
                }
                if let deserializer::TokenDataType::composition { value } = &token_value.value {
                    template::set_optional_global(globals, field_name, value.borderWidth.to_owned(), "");
                }
            },
            TemplateField::border_radius_bottom_left => {
                if let deserializer::TokenDataType::composition { value } = &token_value.value {
                    template::set_optional_global(globals, field_name, value.borderRadiusBottomLeft.to_owned(), "");
                }
            },
            TemplateField::border_radius_bottom_right => {
                if let deserializer::TokenDataType::composition { value } = &token_value.value {
                    template::set_optional_global(globals, field_name, value.borderRadiusBottomRight.to_owned(), "");
                }
            },
            TemplateField::border_radius_top_left => {
                if let deserializer::TokenDataType::composition { value } = &token_value.value {
                    template::set_optional_global(globals, field_name, value.borderRadiusTopLeft.to_owned(), "");
                }
            },
            TemplateField::border_radius_top_right => {
                if let deserializer::TokenDataType::composition { value } = &token_value.value {
                    template::set_optional_global(globals, field_name, value.borderRadiusTopRight.to_owned(), "");
                }
            },
            TemplateField::blur => {
                if let deserializer::TokenDataType::boxShadow { value } = &token_value.value {
                    template::set_box_shadow_global(globals, index, value, deserializer::BoxShadowType::blur, field_data, &mut template);
                }
            },
            TemplateField::spread => {
                if let deserializer::TokenDataType::boxShadow { value } = &token_value.value {
                    template::set_box_shadow_global(globals, index, value, deserializer::BoxShadowType::spread, field_data, &mut template);
                }
            },
            TemplateField::t_type => {
                if let deserializer::TokenDataType::boxShadow { value } = &token_value.value {
                    template::set_box_shadow_global(globals, index, value, deserializer::BoxShadowType::t_type, field_data, &mut template);
                }
            },
            TemplateField::x => {
                if let deserializer::TokenDataType::boxShadow { value } = &token_value.value {
                    template::set_box_shadow_global(globals, index, value, deserializer::BoxShadowType::x, field_data, &mut template);
                }
            },
            TemplateField::y => {
                if let deserializer::TokenDataType::boxShadow { value } = &token_value.value {
                    template::set_box_shadow_global(globals, index, value, deserializer::BoxShadowType::y, field_data, &mut template);
                }
            },
            TemplateField::paragraph_spacing => {
                if let deserializer::TokenDataType::typography { value } = &token_value.value {
                    template::set_optional_global(globals, field_name, value.paragraphSpacing.to_owned(), "");
                }
                if let deserializer::TokenDataType::paragraphSpacing { value } = &token_value.value {
                    template::set_global(globals, field_name, value);
                }
            },
            TemplateField::paragraph_indent => {
                if let deserializer::TokenDataType::typography { value } = &token_value.value {
                    template::set_optional_global(globals, field_name, value.paragraphIndent.to_owned(), "");
                }
                if let deserializer::TokenDataType::paragraphIndent { value } = &token_value.value {
                    template::set_global(globals, field_name, value);
                }
            },
            TemplateField::text_case => {
                if let deserializer::TokenDataType::typography { value } = &token_value.value {
                    template::set_optional_global(globals, field_name, value.textCase.to_owned(), "");
                }
                if let deserializer::TokenDataType::textCase { value } = &token_value.value {
                    template::set_global(globals, field_name, value);
                }
            },
            TemplateField::text_decoration => {
                if let deserializer::TokenDataType::typography { value } = &token_value.value {
                    template::set_optional_global(globals, field_name, value.textDecoration.to_owned(), "");
                }
                if let deserializer::TokenDataType::textDecoration { value } = &token_value.value {
                    template::set_global(globals, field_name, value);
                }
            },
            _ => {}
        }
        } else { 
            println!("The field /{}/ is not available for this type, the available fields are /{}/", field_data.key_without_index, available_fields.values.join(", "));
        }
    }

    template
}

pub fn template_as_values(template: &str) ->  Vec<deserializer::TemplateFieldData> { 
    let mut template_fields: Vec<deserializer::TemplateFieldData> = Vec::new();

    let pure_values = utils::between_all(Vec::new(), template, "{{", "}}");
   
    for pure in pure_values {
        let values_split = pure.split('|');
        let values_parts:Vec<&str> = values_split.collect();
        let template_key_name = values_parts[0].trim();
        let key_split:Vec<&str> = template_key_name.split('-').collect();

        let mut index: Option<usize> = None;
        if key_split.len() == 2 {
            index = Some(key_split[1].parse::<usize>().unwrap());
        }
       
        let template_field_data = deserializer::TemplateFieldData {
            index,
            full_template: pure.to_string(),
            special: deserializer::TemplateField::from_str(key_split[0]),
            key_full: template_key_name.to_string(),
            key_without_index: key_split[0].to_string()
        };
  
        template_fields.push(template_field_data);

    }

    template_fields
}
