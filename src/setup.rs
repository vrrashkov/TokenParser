use ::askama::Template;
use liquid::ValueView;
use serde::de::value::MapAccessDeserializer;
use std::cell::RefCell;
use std::rc::Rc;
use std::str::FromStr;
use convert_case::{Case, Casing};

use crate::deserializer;
use crate::deserializer::AvailableFields;
use crate::deserializer::ConfigTemplateType;
use crate::general;
use crate::askama;
use crate::template;
use crate::utils;

pub fn init(token_config: &deserializer::TokensConfig) { 

    let token_data_wrapper_list = general::generate_tokens(&token_config);
    // different styles light/dark
    for token_data_wrapper in token_data_wrapper_list.iter() {
        for template_config in token_config.templates.iter() {
        
            let file_data_name = "custom";
        
            let template_content = template_content_custom(&template_config, &token_data_wrapper, &token_config, &file_data_name, &token_data_wrapper.token_data);
            
            general::create_template(&template_config, &token_data_wrapper.style_name, file_data_name, &template_content);
            
        }
    }
}

fn template_content_custom(
    template_config: &deserializer::ConfigTokensTemplates, 
    token_data_wrapper: &template::TokenDataWrapper, 
    token_config: &deserializer::TokensConfig,
    file_data_name: &str,
    file_data_list: &Vec<template::TokenData>
) -> Option<String>{ 
    let mut template_content: Option<String> = Default::default();

    let custom_template = &template_config.settings_custom;
    let mut current_template: askama::CustomTemplate = askama::CustomTemplate { 
        header: custom_template.header.to_owned(),
        footer: custom_template.footer.to_owned(),
        sub_header: custom_template.sub_header.to_owned(),
        sub_footer: custom_template.sub_footer.to_owned(),
        class: custom_template.class.to_owned(),
        class_name: token_config.formatted_class_name(&token_data_wrapper.style_name, &template_config, file_data_name),
        color_values: None,
        font_values: None,
        spacing_values: None,
        border_width_values: None,
        border_radius_values: None, 
        letter_spacing_values: None, 
        line_height_values: None, 
        font_sizes_values: None, 
        font_weights_values: None, 
        font_families_values: None, 
        box_shadow_values: None, 
        composition_values: None, 
    }; 
   // dbg!(&file_data_list);

    for template in &custom_template.template_type {
        //dbg!(&custom_template.template_type);
        let available_fields = template.available_fields();
        match &template {
            deserializer::CustomConfigTempalteType::color(value) => {
                let pure_values = template_pure_values(file_data_list, deserializer::ConfigTemplateType::color);
                current_template.color_values = Some(template_replaced_values(value, &pure_values, &available_fields));
            },
            deserializer::CustomConfigTempalteType::typography(value) => {
                let pure_values = template_pure_values(file_data_list, deserializer::ConfigTemplateType::typography);
                current_template.font_values = Some(template_replaced_values(value, &pure_values, &available_fields));
            },
            deserializer::CustomConfigTempalteType::spacing(value) => {
                let pure_values = template_pure_values(file_data_list, deserializer::ConfigTemplateType::spacing);
                current_template.spacing_values = Some(template_replaced_values(value, &pure_values, &available_fields));
            },
            deserializer::CustomConfigTempalteType::borderWidth(value) => {
                let pure_values = template_pure_values(file_data_list, deserializer::ConfigTemplateType::borderWidth);
                current_template.border_width_values = Some(template_replaced_values(value, &pure_values, &available_fields));
            },
            deserializer::CustomConfigTempalteType::borderRadius(value) => {
                let pure_values = template_pure_values(file_data_list, deserializer::ConfigTemplateType::borderRadius);
                current_template.border_radius_values = Some(template_replaced_values(value, &pure_values, &available_fields));
            },
            deserializer::CustomConfigTempalteType::letterSpacing(value) => {
                let pure_values = template_pure_values(file_data_list, deserializer::ConfigTemplateType::letterSpacing);
                current_template.letter_spacing_values = Some(template_replaced_values(value, &pure_values, &available_fields));
            },
            deserializer::CustomConfigTempalteType::lineHeights(value) => {
                let pure_values = template_pure_values(file_data_list, deserializer::ConfigTemplateType::lineHeights);
                current_template.line_height_values = Some(template_replaced_values(value, &pure_values, &available_fields));
            },
            deserializer::CustomConfigTempalteType::fontSizes(value) => {
                let pure_values = template_pure_values(file_data_list, deserializer::ConfigTemplateType::fontSizes);
                current_template.font_sizes_values = Some(template_replaced_values(value, &pure_values, &available_fields));
            },
            deserializer::CustomConfigTempalteType::fontWeights(value) => {
                let pure_values = template_pure_values(file_data_list, deserializer::ConfigTemplateType::fontWeights);
                current_template.font_weights_values = Some(template_replaced_values(value, &pure_values, &available_fields));
            },
            deserializer::CustomConfigTempalteType::fontFamilies(value) => {
                let pure_values = template_pure_values(file_data_list, deserializer::ConfigTemplateType::fontFamilies);
                current_template.font_families_values = Some(template_replaced_values(value, &pure_values, &available_fields));
            },
            deserializer::CustomConfigTempalteType::boxShadow(value) => {
                let pure_values = template_pure_values(file_data_list, deserializer::ConfigTemplateType::boxShadow);
                current_template.box_shadow_values = Some(template_list_replaced_values(value, &pure_values, &available_fields));
            },
            deserializer::CustomConfigTempalteType::composition(value) => {
                let pure_values = template_pure_values(file_data_list, deserializer::ConfigTemplateType::composition);
                current_template.composition_values = Some(template_replaced_values(value, &pure_values, &available_fields));
            },
            deserializer::CustomConfigTempalteType::none => todo!(),
        }
    }
    
    template_content = current_template.render().ok();
    return template_content;
}
fn template_list_replaced_values(templates: &Vec<String>, pure_values: &Vec<template::TokenValue>, available_fields: &AvailableFields) -> Vec<String>{ 
    let mut values_content:Vec<String> = Vec::new();
    for template in templates {
        // let template_fields = template_as_values(template);

        // for content in pure_values { 
        //     let current = template_set_values(content, &template, available_fields, &template_fields);
        //     values_content.push(current);
        // }
        
        let values = template_replaced_values(&template, &pure_values, &available_fields);    
        for value in values {
            values_content.push(value);
        }   
    }
    
    return values_content;
}
fn template_replaced_values(template: &String, pure_values: &Vec<template::TokenValue>, available_fields: &AvailableFields) -> Vec<String>{ 
    let template_fields = template_as_values(template);

    let mut values_content:Vec<String> = Vec::new();
    for content in pure_values { 
        let mut globals = liquid::object!({});
        let current = template_set_values(content, &template, available_fields, &template_fields, &mut globals);
        //print!("current: {}", &current);
        let template_parsed = liquid::ParserBuilder::with_stdlib()
        .build().unwrap()
        .parse(&current).unwrap();

        let output = template_parsed.render(&globals).unwrap();
        dbg!(&globals);
        values_content.push(output);
    }
    return values_content;
}


fn template_pure_values(file_data_list: &Vec<template::TokenData>, template_type: deserializer::ConfigTemplateType) -> Vec<template::TokenValue>{
    if let Some(file_data) = file_data_list.into_iter().find(|f| f.t_type == template_type) {
        //let pure_values = template::values_from_type(&file_data);
        return file_data.token_value.to_owned();
    }

    return Vec::new();
}
pub fn template_set_values(data: &template::TokenValue, pure_template: &String, available_fields: &AvailableFields, fields: &Vec<deserializer::TemplateFieldData>, globals: &mut liquid::Object) -> String { 
    let token_value = data;
    let mut template: String = format!("{}", pure_template);
    for field_data in fields {
        let field_name = field_data.key_full.as_str();

        if available_fields.values.contains(&field_data.key_without_index.to_string()) {
         match &field_data.special {
            ConfigTemplateType::spacing => {
                if let deserializer::TokenDataType::spacing { value } = &token_value.value {
                    globals.insert(field_name.to_owned().into(), liquid::model::Value::scalar(value.to_string()));
                    //template::format_template(&mut template, &field_name, &value.to_string());
                }
            },
            _ => {}
            // ConfigTemplateType::color => todo!(),
            // ConfigTemplateType::typography => todo!(),
            // ConfigTemplateType::borderWidth => todo!(),
            // ConfigTemplateType::borderRadius => todo!(),
            // ConfigTemplateType::letterSpacing => todo!(),
            // ConfigTemplateType::lineHeights => todo!(),
            // ConfigTemplateType::fontSizes => todo!(),
            // ConfigTemplateType::fontWeights => {},
            // ConfigTemplateType::fontFamilies => {}
            // ConfigTemplateType::boxShadow =>{},
            // ConfigTemplateType::composition => {},
            // ConfigTemplateType::none => {},
        }
            // match &field_data.name {
            //     deserializer::TemplateField::variable_name { value } => {
            //         template::variable_name(&mut template, &field_template_pattern, &value, &token_value);
            //     },
            //     deserializer::TemplateField::color { value } => {
            //         let variant_value = value;
            //         if let deserializer::TokenDataType::color { value } = &token_value.value {
            //             template::color(&mut template, &field_template_pattern, &variant_value, &value);
            //         }
            //         if let deserializer::TokenDataType::boxShadow { value } = &token_value.value {
            //             template::box_shadow_color(&mut template, &field_template_pattern, &variant_value, &value);
            //         }
            //     },
            //     deserializer::TemplateField::font_family { value } => {
            //         let variant_value = value;
            //         if let deserializer::TokenDataType::typography { value } = &token_value.value {
            //             let typography_value = value;
            //             template::font_family(&mut template, typography_value.fontFamily.to_owned(), &field_template_pattern, &variant_value);
            //         }

            //         if let deserializer::TokenDataType::fontFamilies { value } = &token_value.value {
            //             let font_families_value = value;
            //             template::font_family(&mut template, font_families_value.to_owned(), &field_template_pattern, &variant_value);
            //         }
            //     },
            //     deserializer::TemplateField::font_size { value } => {
            //         let variant_value = value;
            //         if let deserializer::TokenDataType::typography { value } = &token_value.value {
            //             template::default(&mut template, &field_template_pattern, variant_value, &value.fontSize.to_string());
            //         }
            //         if let deserializer::TokenDataType::fontSizes { value } = &token_value.value {
            //             template::default(&mut template, &field_template_pattern, variant_value, &value.to_string());
            //         }
            //     },
            //     deserializer::TemplateField::font_weight { value } => {
            //         let variant_value = value;
            //         if let deserializer::TokenDataType::typography { value } = &token_value.value {
            //             template::default(&mut template, &field_template_pattern, variant_value, &value.fontWeight.to_string());
            //         }
            //         if let deserializer::TokenDataType::fontWeights { value } = &token_value.value {
            //             template::default(&mut template, &field_template_pattern, variant_value, &value.to_string());
            //         }
            //     },
            //     deserializer::TemplateField::spacing { value } => {
            //         let variant_value = value;
            //         if let deserializer::TokenDataType::typography { value } = &token_value.value {
            //             template::default(&mut template, &field_template_pattern, variant_value, &value.letterSpacing.to_string());
            //         }
            //         if let deserializer::TokenDataType::letterSpacing { value } = &token_value.value {
            //             template::default(&mut template, &field_template_pattern, variant_value, &value.to_string());
            //         }
            //         if let deserializer::TokenDataType::spacing { value } = &token_value.value {
            //             template::default(&mut template, &field_template_pattern, variant_value, &value.to_string());
            //         }
            //         if let deserializer::TokenDataType::composition { value } = &token_value.value {
            //             template::default_option_number(&mut template, &field_template_pattern, variant_value, &value.itemSpacing);
            //         }
            //     },
            //     deserializer::TemplateField::line_height { value } => {
            //         let variant_value = value;
            //         if let deserializer::TokenDataType::typography { value } = &token_value.value {
            //             template::default(&mut template, &field_template_pattern, variant_value, &value.lineHeight.to_string());
            //         }
            //         if let deserializer::TokenDataType::lineHeights { value } = &token_value.value {
            //             template::default(&mut template, &field_template_pattern, variant_value, &value.to_string());
            //         }
            //     },
            //     deserializer::TemplateField::horizontal_padding { value } => {
            //         let variant_value = value;
            //         if let deserializer::TokenDataType::composition { value } = &token_value.value {
            //             template::default_option_number(&mut template, &field_template_pattern, variant_value, &value.horizontalPadding);
            //         }
            //     },
            //     deserializer::TemplateField::vertical_padding { value } => {
            //         let variant_value = value;
            //         if let deserializer::TokenDataType::composition { value } = &token_value.value {
            //             template::default_option_number(&mut template, &field_template_pattern, variant_value, &value.verticalPadding);
            //         }
            //     },
            //     deserializer::TemplateField::padding_bottom { value } => {
            //         let variant_value = value;
            //         if let deserializer::TokenDataType::composition { value } = &token_value.value {
            //             template::default_option_number(&mut template, &field_template_pattern, variant_value, &value.paddingBottom);
            //         }
            //     },
            //     deserializer::TemplateField::padding_top { value } => {
            //         let variant_value = value;
            //         if let deserializer::TokenDataType::composition { value } = &token_value.value {
            //             template::default_option_number(&mut template, &field_template_pattern, variant_value, &value.paddingTop);
            //         }
            //     },
            //     deserializer::TemplateField::padding_left { value } =>  {
            //         let variant_value = value;
            //         if let deserializer::TokenDataType::composition { value } = &token_value.value {
            //             template::default_option_number(&mut template, &field_template_pattern, variant_value, &value.paddingLeft);
            //         }
            //     },
            //     deserializer::TemplateField::padding_right { value } => {
            //         let variant_value = value;
            //         if let deserializer::TokenDataType::composition { value } = &token_value.value {
            //             template::default_option_number(&mut template, &field_template_pattern, variant_value, &value.paddingRight);
            //         }
            //     },
            //     deserializer::TemplateField::sizing { value } => {
            //         let variant_value = value;
            //         if let deserializer::TokenDataType::composition { value } = &token_value.value {
            //             template::default_option_number(&mut template, &field_template_pattern, variant_value, &value.sizing);
            //         }
            //     },
            //     deserializer::TemplateField::height { value } => {
            //         let variant_value = value;
            //         if let deserializer::TokenDataType::composition { value } = &token_value.value {
            //             template::default_option_number(&mut template, &field_template_pattern, variant_value, &value.height);
            //         }
            //     },
            //     deserializer::TemplateField::width { value } => {
            //         let variant_value = value;
            //         if let deserializer::TokenDataType::composition { value } = &token_value.value {
            //             template::default_option_number(&mut template, &field_template_pattern, variant_value, &value.width);
            //         }
            //     },
            //     deserializer::TemplateField::border_radius { value } => {
            //         let variant_value = value;
            //         if let deserializer::TokenDataType::borderRadius { value } = &token_value.value {
            //             template::default(&mut template, &field_template_pattern, variant_value, &value.to_string());
            //         }
            //         if let deserializer::TokenDataType::composition { value } = &token_value.value {
            //             template::default_option_number(&mut template, &field_template_pattern, variant_value, &value.borderRadius);
            //         }
            //     },
            //     deserializer::TemplateField::border_width { value } =>  {
            //         let variant_value = value;
            //         if let deserializer::TokenDataType::borderWidth { value } = &token_value.value {
            //             template::default(&mut template, &field_template_pattern, variant_value, &value.to_string());
            //         }
            //         if let deserializer::TokenDataType::composition { value } = &token_value.value {
            //             template::default_option_number(&mut template, &field_template_pattern, variant_value, &value.borderWidth);
            //         }
            //     },
            //     deserializer::TemplateField::border_radius_bottom_left { value } => {
            //         let variant_value = value;
            //         if let deserializer::TokenDataType::composition { value } = &token_value.value {
            //             template::default_option_number(&mut template, &field_template_pattern, variant_value, &value.borderRadiusBottomLeft);
            //         }
            //     },
            //     deserializer::TemplateField::border_radius_bottom_right { value } =>  {
            //         let variant_value = value;
            //         if let deserializer::TokenDataType::composition { value } = &token_value.value {
            //             template::default_option_number(&mut template, &field_template_pattern, variant_value, &value.borderRadiusBottomRight);
            //         }
            //     },
            //     deserializer::TemplateField::border_radius_top_left { value } => {
            //         let variant_value = value;
            //         if let deserializer::TokenDataType::composition { value } = &token_value.value {
            //             template::default_option_number(&mut template, &field_template_pattern, variant_value, &value.borderRadiusTopLeft);
            //         }
            //     },
            //     deserializer::TemplateField::border_radius_top_right { value } => {
            //         let variant_value = value;
            //         if let deserializer::TokenDataType::composition { value } = &token_value.value {
            //             template::default_option_number(&mut template, &field_template_pattern, variant_value, &value.borderRadiusTopRight);
            //         }
            //     },
            //     deserializer::TemplateField::blur { value } => {
            //         let variant_value = value;
            //         if let deserializer::TokenDataType::boxShadow { value } = &token_value.value {
            //             template::box_shadow_blur(&mut template, &field_template_pattern, variant_value, &value);
            //         }
            //     },
            //     deserializer::TemplateField::spread { value } => {
            //         let variant_value = value;
            //         if let deserializer::TokenDataType::boxShadow { value } = &token_value.value {
            //             template::box_shadow_spread(&mut template, &field_template_pattern, variant_value, &value);
            //         }
            //     },
            //     deserializer::TemplateField::t_type { value } => {
            //         let variant_value = value;
            //         if let deserializer::TokenDataType::boxShadow { value } = &token_value.value {
            //             template::box_shadow_type(&mut template, &field_template_pattern, variant_value, &value);
            //         }
            //     },
            //     deserializer::TemplateField::x { value } => {
            //         let variant_value = value;
            //         if let deserializer::TokenDataType::boxShadow { value } = &token_value.value {
            //             template::box_shadow_x(&mut template, &field_template_pattern, variant_value, &value);
            //         }
            //     },
            //     deserializer::TemplateField::y { value } => {
            //         let variant_value = value;
            //         if let deserializer::TokenDataType::boxShadow { value } = &token_value.value {
            //             template::box_shadow_y(&mut template, &field_template_pattern, variant_value, &value);
            //         }
            //     },
            //     deserializer::TemplateField::NONE => todo!(),
            // }
        } else { 
            println!("The only available fields for the variant /{}/ are /{}/", available_fields.name, available_fields.values.join(", "));
        }
    }

    return template;
}

pub fn template_as_values(template: &str) -> Vec<deserializer::TemplateFieldData> { 
    let mut template_fields: Vec<deserializer::TemplateFieldData> = Vec::new();

    let pure_values = utils::between_all(Vec::new(), template, "{{", "}}");

    for pure in pure_values {
        let values_split = pure.split("|");
        let values_parts:Vec<&str> = values_split.collect();
        let template_key_name = values_parts[0].trim();
        let key_split:Vec<&str> = template_key_name.split("-").collect();

        let mut index: Option<usize> = None;
        if key_split.len() == 2 {
            index = Some(key_split[1].parse::<usize>().unwrap());
        }
       
        let template_field_data = deserializer::TemplateFieldData {
            index: index,
            full_template: pure.to_string(),
            special: ConfigTemplateType::from_str(&key_split[0]),
            key_full: template_key_name.to_string(),
            key_without_index: key_split[0].to_string()
        };

        //dbg!(&template_field_data);
        template_fields.push(template_field_data);
        // let template_parse = liquid::ParserBuilder::with_stdlib()
        // .build().unwrap()
        // .parse("Liquid! {{num | minus: 2}}").unwrap();
    }
    // for pure in pure_values {
    //     let segments = pure.split(":").collect::<Vec<&str>>();
        
    //     let mut index: Option<usize> = None;
    //     let mut name: String = format!("");
    //     let mut variant: String = format!("");

    //     if (segments.len() >= 3) {
    //         name = segments[0].parse::<String>().unwrap();
    //         variant = format!(r#","value": "{}""#, segments[1].parse::<String>().unwrap());
    //         index = Some(segments[2].parse::<usize>().unwrap());
    //     }

    //     if (segments.len() >= 2) {
    //         name = segments[0].parse::<String>().unwrap();
    //         variant = format!(r#","value": "{}""#, segments[1].parse::<String>().unwrap());
    //     }

    //     let mut index_format = format!("");
    //     if let Some(value) = index {
    //         index_format = format!(r#","index": {}"#, value);
    //     }
        
    //     let data =  format!(r#"{{"name": {{"type": "{}" {} }} {} }}"#, name, variant, index_format);
    //     //dbg!(&data);
    //     let value_object: deserializer::TemplateFieldData = serde_json::from_str(&data.to_owned()).expect("Unable to read the json");
    //     //(&value_object);
    //     template_fields.push((format!("{{{{{}}}}}",pure), value_object));
    // }

    return template_fields;
    // let res: serde_json::Value = serde_json::from_str(&data).expect("Unable to parse");
    // dbg!(res);
}
