use std::path::{Path, self};
use std::{collections::HashMap, fs};
use serde::{Serialize, Deserialize, Deserializer};
use serde_json::{Number, json};
use std::str::FromStr;
use convert_case::{Case, Casing};
use json_comments::StripComments;
use evalexpr::*;
use json_value_merge::Merge;
use easy_color::{RGBA, RGB, HSL, Hex, ColorMix, IntoHex};
use jsonptr::Pointer;

use crate::general;
use crate::deserializer::{self, ConfigTokensGlobalOtherPath};
use crate::template;
use crate::utils;
use crate::global;

pub fn filter_properties(token_config: &deserializer::TokensConfig) { 
            
    let mut allSources: Vec<(String, Vec<String>)> = vec![];

    if let Some(json_figma_source) = &token_config.global.figma_source_paths {
        // get all keys with their values
        // key contains the full path of the tree
        // for example core.natural.fr.c1
        for file in json_figma_source {
            let files = file.combine.files.to_owned();
            let file_name = file.combine.file_name.to_owned().unwrap();
            allSources.push((file_name, files));
        }
    }

    for data in allSources {
        let files = data.1;
        let sourceFileName = data.0;

        let mut pure_values: HashMap<String, String> = HashMap::new();
        
        for file in &files {
            let data_object: serde_json::Value = general::get_json(&file);

            filter_sub_properties("", &data_object, &mut pure_values, vec![]);
        }

        // Calculating all the values to usable ones
        // some values are referencing to other keys for example {spacing.right.1} * 2
        // which we are converting to usable value by eval() the math equation
        let mut calculated_values: HashMap<String, evalexpr::Value> = HashMap::new();
        for (key, val) in &pure_values {
            if !val.contains('{') && !val.contains('}') {
                //println!("don't calculate this val: {}", val);
                continue;
            }

            let mut template_values_list:Vec<String> = Vec::new();

            let (template_values, template_list) = find_all_between(val.to_owned(), &mut template_values_list, &pure_values);

            if !template_list.is_empty() { 
                let mut math_eval = Value::from(template_values.to_owned());

                if let Ok(ev) = eval(template_values.to_owned().as_str()) {
                    math_eval = ev;
                }

                calculated_values.insert(key.to_string(), math_eval);
            } else {
                let mut math_eval = Value::from(val.to_owned());
                
                if let Ok(ev) = eval(template_values.to_owned().as_str()) {
                    math_eval = ev;
                }

                calculated_values.insert(key.to_string(), math_eval);
            }
        }

        let create_styles_directory = &token_config.global.style_output_path;//"assets/generated_styles/";
        // merging files and updating the values
        // the merged files are dependant on the config
        
        'figma_output: for group in &token_config.global.figma_output_paths {
            
            let mut data_object: serde_json::Value = serde_json::Value::Null;
            let mut file_name = String::from("");
            for (index, fileData) in group.combine.files.iter().enumerate() {
                let combineFileName = &group.combine.file_name.to_owned().unwrap();
                let currentList = &files;
                if combineFileName.to_owned() != sourceFileName.to_owned() {
                    continue 'figma_output
                }

                file_name = combineFileName.to_owned();
                
                let uniqueName = Path::new(&fileData.path).file_stem().unwrap().to_str().unwrap().to_owned();

                let mut data_to_merge_with: serde_json::Value = general::get_json(&fileData.path);

                for (key_path, key_value) in &calculated_values {
                    let mut path_list = &key_path.split('.').collect::<Vec<&str>>();
                    let path_list_count = path_list.len();
                    
                    let pointer_value = format!("/{}", path_list.join("/"));

                    let ptr = Pointer::new(path_list);
        
                    // replace the values from json
                    data_to_merge_with.pointer_mut(ptr.as_str()).map(|v| {
                        match key_value {
                            Value::String(val) => {
                                *v = json!(val)
                            },
                            Value::Float(val) => {
                                *v = json!(val.to_string())
                            },
                            Value::Int(val) => {
                                *v = json!(val.to_string())
                            },
                            Value::Boolean(val) => {
                                *v = json!(val.to_string())
                            },
                            Value::Tuple(val) => {},
                            Value::Empty => {},
                        }
                        
                    });
                }
                
                if let Some(modeName) = &fileData.mode {
                    let wrapperJson = json!({ modeName : data_to_merge_with});
                    data_object.merge(wrapperJson);
                } else {
                    data_object.merge(data_to_merge_with);
                }
            }

           
            // for (key_path, key_value) in &calculated_values {
            //     let mut path_list = &key_path.split('.').collect::<Vec<&str>>();
            //     let path_list_count = path_list.len();
                
            //     let pointer_value = format!("/{}", path_list.join("/"));

            //     let ptr = Pointer::new(path_list);
    
            //     // replace the values from json
            //     data_object.pointer_mut(ptr.as_str()).map(|v| {
            //         match key_value {
            //             Value::String(val) => {
            //                 *v = json!(val)
            //             },
            //             Value::Float(val) => {
            //                 *v = json!(val.to_string())
            //             },
            //             Value::Int(val) => {
            //                 *v = json!(val.to_string())
            //             },
            //             Value::Boolean(val) => {
            //                 *v = json!(val.to_string())
            //             },
            //             Value::Tuple(val) => {},
            //             Value::Empty => {},
            //         }
                    
            //     });
            // }
            
            let file = format!("{}{}.json",&create_styles_directory, &file_name);
            println!("heree: {}", file);
            std::fs::write(
                &file,
                serde_json::to_string_pretty(&data_object).unwrap(),
            );
        }
        
    }
}


fn find_all_between(search_inside: String, list: &mut Vec<String>, pure_values: &HashMap<String, String> ) -> (String, Vec<String>) { 

    let found_template = utils::between(search_inside.as_str(), "{","}");

    if found_template.is_empty() {
        return (search_inside, list.to_owned());
    }

    let full_template_value = format!("{{{}}}", found_template);
    let full_template_value_v2 = format!("{{{}.value}}", found_template);

    let pure_value = pure_values.get(found_template);
    let pure_value_v2 = pure_values.get(format!("{}.value",found_template).as_str());

    if let Some(number_text) = pure_value { 
        list.push(found_template.to_string());
        let mut update_search = search_inside.replace(&full_template_value_v2, number_text);
        update_search = update_search.replace(&full_template_value, number_text);
        return find_all_between(update_search, list, pure_values);
    }
    
    if let Some(number_text) = pure_value_v2 { 
        list.push(found_template.to_string());
        let mut update_search = search_inside.replace(&full_template_value_v2, number_text);
        update_search = update_search.replace(&full_template_value, number_text);
        return find_all_between(update_search, list, pure_values);
    }

    (search_inside, list.to_owned())
}

pub fn filter_sub_properties(key: &str, val: &serde_json::Value, pure_values: &mut HashMap<String, String>, path: Vec<String>) { 

    for (ikey, ival) in val.as_object().iter().flat_map(|f|  f.iter()) {
       
        let template_type = ival["type"].as_str();
    
        let mut p: Vec<String> = path.clone();

        p.push(ikey.to_string());
        
        if (ival.is_object() && template_type.is_some()) {
           
            let token_type = ival["type"].as_str();
            if !ival["value"].is_object() {
                if (ival["value"].is_array()) {
                    for (vi, v) in ival["value"].as_array().unwrap().iter().enumerate() {
                        let mut p_cloned = p.clone();
                        p_cloned.push("value".to_string());
                        p_cloned.push(vi.to_string());
                        generate_figma_token_value(v.clone(), pure_values, p_cloned.to_owned(), false);
                    }
                } else if let Some(token_value) = ival["value"].as_str() {
                    let mut p_cloned = p.clone();
                    p_cloned.push("value".to_string());
                    let pure_values_key = p_cloned.join(".");

                    pure_values.insert(pure_values_key, token_value.to_owned());
                }
            } else {
                generate_figma_token_value(ival["value"].to_owned(), pure_values, p.to_owned(), true);
            }
        } else {
            self::filter_sub_properties(ikey.to_owned().as_str(), ival, pure_values, p);
        }
    }
}

fn generate_figma_token_value(json_string: serde_json::Value, pure_values: &mut HashMap<String, String>, p: Vec<String>, add_val_path: bool) { 
    let value: serde_json::Value = serde_json::from_value(json_string).expect("Unable to read the json");
  
    match &value {
        serde_json::Value::Object(value_map) => {
            for (key, val) in value_map {
                add_pure_value(val.as_str(), key, pure_values, &p, &add_val_path);
            }
        },
        _ => {}
    }
}

fn add_pure_value(value: Option<&str>, path: &str, pure_values: &mut HashMap<String, String>, p: &[String], add_val_path: &bool) { 
    if let Some(v) = value {
        let np = add_path_value_get_full(p, path,add_val_path);
        pure_values.insert(np, v.to_string());
    }
}

fn add_path_value_get_full(path: &[String], newPath: &str, add_value: &bool) -> String { 
    let mut p = path.to_vec();
    if *add_value {
        p.push("value".to_string());
    }
    p.push(newPath.to_string());
    
    p.join(".")
}
