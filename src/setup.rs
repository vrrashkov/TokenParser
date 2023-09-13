use anyhow::Context;
use liquid::ValueView;
use serde::de::value::MapAccessDeserializer;
use std::cell::RefCell;
use std::rc::Rc;
use std::str::FromStr;
use convert_case::{Case, Casing};

use crate::deserializer;
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
        
            let mut is_data_available = true;
            let template_content = template_content_custom(&mut is_data_available, template_config, token_data_wrapper, token_config, file_data_name, &token_data_wrapper.token_data);
     
            if is_data_available {
                general::create_template(template_config, &token_data_wrapper.style_name, file_data_name, &template_content);
            } 
        }
    }
}

fn template_content_custom(
    is_data_available: &mut bool,
    template_config: &deserializer::ConfigTokensTemplates, 
    token_data_wrapper: &template::TokenDataWrapper, 
    token_config: &deserializer::TokensConfig,
    file_data_name: &str,
    file_data_list: &[template::TokenData]
) -> Option<String>{ 
    let mut template_content: Option<String> = Default::default();

    let custom_template = &template_config.settings_custom;
    let mut current_template: CustomTemplate = CustomTemplate { 
        headers: token_config.format_extra(&token_data_wrapper.style_name, &custom_template.header.to_owned()),
        footers: token_config.format_extra(&token_data_wrapper.style_name, &custom_template.footer.to_owned()),
        values: None,
    }; 

    for template in &custom_template.template_type {
        let template_type = &template.t_type;
        let template_source = &template.source;

        let mut verify_tempalte = true;
        if let Some(source) = template_source  {
            if !source.contains(&token_data_wrapper.style_name.to_string()) {
                verify_tempalte = false;
            }
        }

        // If not "source" is set we add all
        // If there is "source" check if it contains the name and verify 
        // If not continue
        if !verify_tempalte {
            continue;
        }

        match &template.value {
            deserializer::CustomConfigTempalteTypeValue::Value(value) => {
                template_update_list_values(file_data_list, &mut current_template, template_type.to_owned(), value);
            },
            deserializer::CustomConfigTempalteTypeValue::Values(values) => {

                template_list_replaced_values(file_data_list, &mut current_template, template_type.to_owned(),values);
            },
        }
    }

    if current_template.values.is_none() {
        *is_data_available = false;
    }

    if let Some(values) = &current_template.values {
        if values.len() <= 0 {
            *is_data_available = false;
        }
    }

    let file = include_str!("../templates/liquid_template.html");
    let template = liquid::ParserBuilder::with_stdlib()
    .build().with_context(|| format!("Liquid template build error {}", &file)).unwrap()
    .parse(file).with_context(|| format!("Liquid template parse error {}", &file)).unwrap();

    let mut globals = liquid::object!({
        "headers": current_template.headers,
        "footers": current_template.footers,
        "values": current_template.values,
    });
    //dbg!(&globals);
    Some(template.render(&globals).with_context(|| format!("Template could not render with globals {}", &globals.to_kstr())).unwrap())
}

fn template_update_list_values(file_data_list: &[template::TokenData], current_template: &mut CustomTemplate, config_type: String, value: &String) { 
    let pure_values = template_pure_values(file_data_list, config_type.to_owned());
    let mut current = template_replaced_values_single(value, &pure_values);
    current_template.update_template_values(config_type, current);
}

fn template_list_replaced_values(file_data_list: &[template::TokenData], current_template: &mut CustomTemplate, config_type: String, templates: &[String]) { 
    let mut values_content:Vec<String> = Vec::new();
    let pure_values = template_pure_values(file_data_list, config_type.to_owned());

    for (index, template) in templates.iter().enumerate() { 
        let current = template_replaced_values(index, template, &pure_values);    
        current_template.update_template_values(config_type.to_owned(), current);
    }
}

fn template_replaced_values_single(template: &String, pure_values: &Vec<template::TokenValue>) -> Vec<String>  { 
    template_replaced_values(0, template, pure_values)
}

fn template_replaced_values(index: usize, template: &String, pure_values: &Vec<template::TokenValue>) -> Vec<String> { 
    
    let mut default_globals = liquid::object!({});
    let template_fields= template_as_values(template, &mut default_globals);

    let mut values_content:Vec<String> = Vec::new();
    for content in pure_values { 
        let mut globals = default_globals.clone();
        let current_optional = template_set_values(index, content, template, &template_fields, &mut globals);

        if let Some(current) = current_optional {
            //print!("current: {}", &current);
            let template_parsed = liquid::ParserBuilder::with_stdlib()
            .filter(filters::remove_space::RemoveSpace)
            .filter(filters::as_text_or_number::AsTextOrNumber)
            .filter(filters::color::Color)
            .filter(filters::case::CamelCase)
            .filter(filters::case::PascalCase)
            .filter(filters::case::KebabCase)
            .filter(filters::optional::Optional)
            .filter(filters::empty::Empty)
            .build().with_context(|| "Error with template setup build").unwrap()
            .parse(&current).with_context(|| "Error with template setup parse").unwrap();

            let output = template_parsed.render(&globals).with_context(|| "Error with template setup render globals").unwrap();
            //dbg!(&globals);
            if !output.is_empty() {
                values_content.push(output);
            }
            
        }
    }
    values_content
}

fn template_pure_values(file_data_list: &[template::TokenData], template_type: String) -> Vec<template::TokenValue>{
    if let Some(file_data) = file_data_list.iter().find(|f| f.t_type == template_type) {
        //let pure_values = template::values_from_type(&file_data);
        return file_data.token_value.to_owned();
    }

    Vec::new()
}

pub fn template_set_values(index: usize, data: &template::TokenValue, pure_template: &String, fields: &Vec<deserializer::TemplateFieldData>, globals: &mut liquid::Object) -> Option<String> { 
    let token_value = data;
    let mut template: Option<String> = Some(pure_template.to_string());

    for field_data in fields {
        let field_name = field_data.key_full.as_str();
        let field_name_without_index = field_data.key_without_index.as_str();
        let field_index = field_data.index;
        let value = String::new();
        if field_name_without_index == "variable_name" {
            template::set_global(globals, field_name, token_value.variable_name());
        } else if (field_name_without_index == "description") {
            if let Some(desc) = &token_value.description {
                template::set_global(globals, field_name, desc);
            }
        } else {
            match &token_value.value {
                deserializer::TokenDataType::Value(value) => {
                    let mut pure_value = value.get(field_name_without_index);
                  
                    if pure_value.is_none() {
                        if let Some(val) = value.get("value") {
                            match val {
                                serde_json::Value::Array(values) => {
                                    if (index != values.len()-1) {
                                        return None;
                                    }  
                                    pure_value = val.get(field_index)?.get(field_name_without_index);
                                },
                                serde_json::Value::Object(obj_val) => {     
                                    if (index != 0) {
                                        return None;
                                    }  
                                    pure_value = obj_val.get(field_name_without_index);
                                   
                                }
                                _ => {
                           
                                }
                            };
                        }
                    } else {
                       
                    }
                  
                    template::set_optional_global(globals, field_name, pure_value.cloned(), "");
                },
                deserializer::TokenDataType::Values(values) => {
                    let pure_value = values.get(field_index)?.get(field_name_without_index);
                    template::set_optional_global(globals, field_name, pure_value.cloned(), "");
                },
                deserializer::TokenDataType::None => {
                    println!("Something went wrong with template: {}, value: {}", pure_template, &token_value.name);
                },
            }
        }

    }

    template
}

pub fn template_as_values(template: &str, default_globals: &mut liquid_core::Object) ->  Vec<deserializer::TemplateFieldData> { 
    let mut template_fields: Vec<deserializer::TemplateFieldData> = Vec::new();

    let pure_values = utils::between_all(Vec::new(), template, "{{", "}}");
   
    for pure in pure_values {
        let values_split = pure.split('|');
        let values_parts:Vec<&str> = values_split.collect();
        let template_key_name = values_parts[0].trim();
        let key_split:Vec<&str> = template_key_name.split('-').collect();

        let mut index: usize = 0;
        if key_split.len() == 2 {
            index = key_split[1].parse::<usize>().with_context(|| format!("Cannot parse to usize {}", key_split[1])).unwrap();
        }
       
        let template_field_data = deserializer::TemplateFieldData {
            index,
            full_template: pure.to_string(),
            key_full: template_key_name.to_string(),
            key_without_index: key_split[0].to_string()
        };
        template::set_global(default_globals, &template_field_data.key_full, "");
        template_fields.push(template_field_data);

    }

    template_fields
}
