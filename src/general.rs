
use std::fs;
use std::io::Write;
use std::fs::File;
use std::path::Path;
use std::str::FromStr;
use convert_case::{Case, Casing};
use json_comments::StripComments;
use json_value_merge::Merge;
use serde_json::Value;

use crate::{deserializer, template};
use crate::template::TokenValueType;
use crate::askama;

pub fn get_config(config_file: &str) -> deserializer::TokensConfig { 
    
    let design_tokens_config = &config_file;
 
    let data = fs::read_to_string(&design_tokens_config).expect("Unable to read file");
    
    let data_strip_comments = StripComments::new(data.as_bytes());
    let token_config: deserializer::TokensConfig = serde_yaml::from_reader(data_strip_comments).expect("Unable to read the json");

    //println!("deserialized token_config = {:?}", token_config);

    return token_config;
}

pub fn generate_tokens(tokens_config: &deserializer::TokensConfig) -> Vec<template::TokenDataWrapper> { 
    let mut token_data_wrapper_list = Vec::new();

    let style_files = &tokens_config.global.style_path;
    for style in style_files {
        let file_path = &style;
        let file_name = Path::new(&style).file_stem().unwrap().to_str().unwrap();
        let style_output_path = format!("{}/{}.json", &tokens_config.global.style_output_path, &file_name);
        let core_output_path = format!("{}/{}.json", &tokens_config.global.style_output_path, "core");

        println!("style_output_path {}: core_output_path: {}", &style_output_path, &core_output_path);
        let style_json = get_json(&style_output_path);
        let core_json = get_json(&core_output_path);
        let mut res: Value = style_json;
        res.merge(core_json);

        let token_data_list = filter_properties(&res);
    
        let token_data_list_combined = combine_tokens(&token_data_list, &tokens_config);

        let token_data_wrapper: template::TokenDataWrapper = template::TokenDataWrapper { 
            style_name : file_name.to_string(),
            token_data : token_data_list_combined
        };
        
        &token_data_wrapper_list.push(token_data_wrapper);
    }
   
    return token_data_wrapper_list;
}

pub fn combine_tokens(token_data_list: &Vec<template::TokenData>, tokens_config: &deserializer::TokensConfig) -> Vec<template::TokenData> { 

    let mut updated_token_data_list: Vec<template::TokenData> = Vec::new();
  
    for template_config in &tokens_config.templates { 
        let mut token_data_combined: template::TokenData = Default::default();
        
        for token_data in token_data_list { 
            if let Some(combine_with) = &template_config.combine{
                if combine_with.contains(&token_data.name) {
                    // we use the first one as the name
                    // it's also not bad idea to combine all 
                    // or add this as another setting
                    token_data_combined.name = combine_with[0].to_string();
                    token_data_combined.token_value.extend(token_data.token_value.to_owned());
                } else {
                    updated_token_data_list.push(token_data.to_owned());
                }
            } else {
                updated_token_data_list.push(token_data.to_owned());
            }
        }

        if (token_data_combined.token_value.len() > 0) {
            updated_token_data_list.push(token_data_combined);
        }
    }
    return updated_token_data_list;
}

pub fn get_json(path: &str) -> serde_json::Value {
    let data = fs::read_to_string(path).expect("Unable to read file");
    let res: serde_json::Value = serde_json::from_str(&data).expect("Unable to parse");
    return res
}

pub fn create_template(template_config: &deserializer::ConfigTokensTemplates, style_name: &str,type_name: &str, current_template: &Option<String>) { 
    if let Some(template_content_value) = current_template {
        let mut file_name_formatted = format!("{}_{}", type_name, style_name);

        let file_name_config = &template_config.settings_general.file_name;
        
        if let Some(file_name_format) = &file_name_config.format {
            deserializer::TokensConfig::format_class_name_templated(&mut file_name_formatted, &file_name_format, &type_name, &style_name, &template_config.settings_general);
        }

        if let Some(file_name_case) = &file_name_config.case {
            file_name_formatted = file_name_formatted.to_case(case_from_str(file_name_case));
        }

        // add extension
        file_name_formatted = format!("{}.{}", file_name_formatted, &template_config.settings_general.file_name.extension);
        

        create_template_file(&template_config, file_name_formatted.as_str(), template_content_value);
    }
}

pub fn create_template_file(template_config: &deserializer::ConfigTokensTemplates, template_name: &str, contents: &str) { 
    let directory = &template_config.settings_general.generate_file_path;
    fs::create_dir_all(directory);

    let file_name = format!("{}/{}",directory,template_name);
    let mut file = File::create(file_name)
        .expect("Error encountered while creating file!");

    file.write_all(contents.as_bytes());
}

pub fn filter_properties(json: &serde_json::Value) -> Vec<template::TokenData> {
    let data_object = &json.as_object();
    
    let mut token_data_list:Vec<template::TokenData> = vec![];
    for (key, val) in data_object.iter().flat_map(|d| d.iter()) {

        //println!("{}", key);
        //println!("----------------------");

        if (val.is_object()) {

            filter_sub_properties(key.to_owned(), val, &mut token_data_list, vec![]);
        }

    }
     
    return token_data_list;
}

pub fn deserialize_token_data_value(data: &Value) -> deserializer::TokenDataType { 
    let value_object: deserializer::TokenDataType = serde_json::from_value(data.to_owned()).expect("Unable to read the json");

    return value_object;
}
pub fn filter_sub_properties(key: String, val: &serde_json::Value, token_data_list: &mut Vec<template::TokenData>, path: Vec<String>) { 

    for (ikey, ival) in val.as_object().iter().flat_map(|f| f.iter()) {
        let template_type = ival["type"].as_str();
    
        if (ival.is_object() && template_type.is_some()) {

            let token_type = ival["type"].as_str();

            let mut p: Vec<String> = path.clone();
            p.push(key.to_owned());

            let token_value_type = token_type.unwrap_or("").to_string();
            let value_object = ival;

            let token_value = template::TokenValue {
                path: p,
                name: ikey.to_owned(),
                token_type: TokenValueType {
                    text: token_value_type.to_owned(),
                    special: deserializer::ConfigTemplateType::from_str(&token_value_type)
                },
                value: deserialize_token_data_value(&value_object),
            };

            let mut token = template::TokenData { 
                name: key.to_owned(),
                t_type: deserializer::ConfigTemplateType::none,
                token_value: Vec::new()//MultiMap::new(),
            };

            let mut token_exist = token_data_list.iter_mut().find(|f| f.t_type == token_value.token_type.special);
            if let Some(token_e) = token_exist {
                token_e.token_value.push(token_value);
            } else {
                token.t_type = deserializer::ConfigTemplateType::from_str(&token_value.token_type.text);
                token.token_value.push(token_value);

                &token_data_list.push(token.to_owned());
            }


        } else {
            let mut p: Vec<String> = path.clone();
            p.push(key.to_owned());
            self::filter_sub_properties(ikey.to_owned(), ival, token_data_list, p);
        }
    }
}

pub fn case_from_str(input: &str) -> Case {
    let value = match input {
        "camel"   => Case::Camel,
        "snake"   => Case::Snake,
        "upper"   => Case::Upper,
        "lower"   => Case::Lower,
        "kebab"   => Case::Kebab,
        "pascal"  => Case::Pascal,
        _         => Case::Camel
    };

    return value
}