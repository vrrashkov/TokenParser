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
use crate::deserializer::{self};
use crate::template;
use crate::utils;
use crate::global;

pub fn filter_properties(token_config: &deserializer::TokensConfig) { 

    let mut pure_values: HashMap<String, String> = HashMap::new();
    // FIGMA VARIABLES
    let mut json_figma_variable_source:Vec<String> = token_config.global.figma_variables_source_paths.to_owned();
    // get all keys with their values
    // key contains the full path of the tree
    // for example core.natural.fr.c1
    for file in &json_figma_variable_source {
        let data_object: serde_json::Value = general::get_json(file);

        filter_sub_properties("", &data_object, &mut pure_values, vec![]);
 
    }

    // FIGMA STUDIO
    let mut json_figma_studio_source:Vec<String> = token_config.global.figma_studio_source_paths.to_owned();
    // get all keys with their values
    // key contains the full path of the tree
    // for example core.natural.fr.c1
    for file in &json_figma_studio_source {
        let data_object: serde_json::Value = general::get_json(file);

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
    
    for group in &token_config.global.figma_output_paths {
        
        let mut data_object: serde_json::Value = serde_json::Value::Null;
        let mut file_name = String::from("");
        for (index, path) in group.combine.files.iter().enumerate() {
            let mut current_file_name = Path::new(path).file_stem().unwrap().to_str().unwrap().to_owned();
            if index == 0 {
                if let Some(custom_file_name) = &group.combine.file_name {
                    file_name = custom_file_name.to_string()
                } else {
                    file_name = current_file_name.to_string();
                }
            }

            let data_to_merge_with: serde_json::Value = general::get_json(path);
            data_object.merge(data_to_merge_with);
        }

        for (key_path, key_value) in &calculated_values {
            let mut path_list = &key_path.split('.').collect::<Vec<&str>>();
            let path_list_count = path_list.len();
            
            let pointer_value = format!("/{}", path_list.join("/"));

            let ptr = Pointer::new(path_list);
   
            // replace the values from json
            data_object.pointer_mut(ptr.as_str()).map(|v| {
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
        
        let file = format!("{}{}.json",&create_styles_directory, &file_name);
        std::fs::write(
            &file,
            serde_json::to_string_pretty(&data_object).unwrap(),
        );
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
   
   //dbg!(&json_string);
    let value: FigmaTokenValueSingle = serde_json::from_value(json_string).expect("Unable to read the json");

    add_pure_value(&value.fontFamily, global::field_value_font_family, pure_values, &p, &add_val_path);
    add_pure_value(&value.fontSize, global::field_value_font_size, pure_values, &p, &add_val_path);
    add_pure_value(&value.fontWeight, global::field_value_font_weight, pure_values, &p, &add_val_path);
    add_pure_value(&value.letterSpacing, global::field_value_letter_spacing, pure_values, &p, &add_val_path);
    add_pure_value(&value.paragraphSpacing, global::field_value_paragraph_spacing, pure_values, &p, &add_val_path);
    add_pure_value(&value.paragraphIndent, global::field_value_paragraph_indent, pure_values, &p, &add_val_path);
    add_pure_value(&value.textCase, global::field_value_text_case, pure_values, &p, &add_val_path);
    add_pure_value(&value.textDecoration, global::field_value_text_decoration, pure_values, &p, &add_val_path);
    add_pure_value(&value.lineHeight, global::field_value_line_height, pure_values, &p, &add_val_path);
    add_pure_value(&value.horizontalPadding, global::field_value_horizontal_padding, pure_values, &p, &add_val_path);
    add_pure_value(&value.verticalPadding, global::field_value_vertical_padding, pure_values, &p, &add_val_path);
    add_pure_value(&value.itemSpacing, global::field_value_item_spacing, pure_values, &p, &add_val_path);
    add_pure_value(&value.paddingBottom, global::field_value_padding_bottom, pure_values, &p, &add_val_path);
    add_pure_value(&value.paddingTop, global::field_value_padding_top, pure_values, &p, &add_val_path);
    add_pure_value(&value.paddingLeft, global::field_value_padding_left, pure_values, &p, &add_val_path);
    add_pure_value(&value.paddingRight, global::field_value_padding_right, pure_values, &p, &add_val_path);
    add_pure_value(&value.sizing, global::field_value_sizing, pure_values, &p, &add_val_path);
    add_pure_value(&value.height, global::field_value_height, pure_values, &p, &add_val_path);
    add_pure_value(&value.width, global::field_value_width, pure_values, &p, &add_val_path);
    add_pure_value(&value.borderRadius, global::field_value_border_radius, pure_values, &p, &add_val_path);
    add_pure_value(&value.borderWidth, global::field_value_border_width, pure_values, &p, &add_val_path);
    add_pure_value(&value.borderRadiusBottomLeft, global::field_value_border_radius_bottom_left, pure_values, &p, &add_val_path);
    add_pure_value(&value.borderRadiusBottomRight, global::field_value_border_radius_bottom_right, pure_values, &p, &add_val_path);
    add_pure_value(&value.borderRadiusTopLeft, global::field_value_border_radius_top_left, pure_values, &p, &add_val_path);
    add_pure_value(&value.borderRadiusTopRight, global::field_value_border_radius_top_right, pure_values, &p, &add_val_path);
    add_pure_value(&value.blur, global::field_value_blur, pure_values, &p, &add_val_path);
    add_pure_value(&value.color, global::field_value_color, pure_values, &p, &add_val_path);
    add_pure_value(&value.string, global::field_value_string, pure_values, &p, &add_val_path);
    add_pure_value(&value.float, global::field_value_float, pure_values, &p, &add_val_path);
    add_pure_value(&value.boolen, global::field_value_boolean, pure_values, &p, &add_val_path);
    add_pure_value(&value.spread, global::field_value_spread, pure_values, &p, &add_val_path);
    add_pure_value(&value.t_type, global::field_value_type, pure_values, &p, &add_val_path);
    add_pure_value(&value.x, global::field_value_x, pure_values, &p, &add_val_path);
    add_pure_value(&value.y, global::field_value_y, pure_values, &p, &add_val_path);
    
}

fn add_pure_value(value: &Option<String>, path: &str, pure_values: &mut HashMap<String, String>, p: &[String], add_val_path: &bool) { 
    if let Some(v) = &value {
        pure_values.insert(add_path_value_get_full(p, path,add_val_path), v.to_string());
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

#[derive(Eq, PartialEq, Serialize, Clone, Deserialize, Debug)]
pub struct FigmaTokenValueSingle { 
    #[serde(default, deserialize_with="parse_to_optional_string")]
    pub horizontalPadding: Option<String>,
    #[serde(default, deserialize_with="parse_to_optional_string")]
    pub verticalPadding: Option<String>,
    #[serde(default, deserialize_with="parse_to_optional_string")]
    pub itemSpacing: Option<String>,
    #[serde(default, deserialize_with="parse_to_optional_string")]
    pub paddingBottom: Option<String>,
    #[serde(default, deserialize_with="parse_to_optional_string")]
    pub paddingTop: Option<String>,
    #[serde(default, deserialize_with="parse_to_optional_string")]
    pub paddingLeft: Option<String>,
    #[serde(default, deserialize_with="parse_to_optional_string")]
    pub paddingRight: Option<String>,
    #[serde(default, deserialize_with="parse_to_optional_string")]
    pub borderRadius: Option<String>,
    #[serde(default, deserialize_with="parse_to_optional_string")]
    pub borderWidth: Option<String>,
    #[serde(default, deserialize_with="parse_to_optional_string")]
    pub borderRadiusBottomLeft: Option<String>,
    #[serde(default, deserialize_with="parse_to_optional_string")]
    pub borderRadiusBottomRight: Option<String>,
    #[serde(default, deserialize_with="parse_to_optional_string")]
    pub borderRadiusTopLeft: Option<String>,
    #[serde(default, deserialize_with="parse_to_optional_string")]
    pub borderRadiusTopRight: Option<String>,
    #[serde(default, deserialize_with="parse_to_optional_string")]
    pub sizing: Option<String>,
    #[serde(default, deserialize_with="parse_to_optional_string")]
    pub height: Option<String>,
    #[serde(default, deserialize_with="parse_to_optional_string")]
    pub width: Option<String>,

    #[serde(default, deserialize_with="parse_to_optional_string")]
    pub fontFamily: Option<String>,
    #[serde(default, deserialize_with="parse_to_optional_string")]
    pub fontWeight: Option<String>,
    #[serde(default, deserialize_with="parse_to_optional_string")]
    pub lineHeight: Option<String>,
    #[serde(default, deserialize_with="parse_to_optional_string")]
    pub fontSize: Option<String>,
    #[serde(default, deserialize_with="parse_to_optional_string")]
    pub letterSpacing: Option<String>,
    #[serde(default, deserialize_with="parse_to_optional_string")]
    pub paragraphSpacing: Option<String>,
    #[serde(default, deserialize_with="parse_to_optional_string")]
    pub paragraphIndent: Option<String>,
    #[serde(default, deserialize_with="parse_to_optional_string")]
    pub textCase: Option<String>,
    #[serde(default, deserialize_with="parse_to_optional_string")]
    pub textDecoration: Option<String>,

    #[serde(default, deserialize_with="parse_to_optional_string")]
    pub blur: Option<String>,
    #[serde(default, deserialize_with="parse_to_optional_string")]
    pub color: Option<String>,
    #[serde(default, deserialize_with="parse_to_optional_string")]
    pub string: Option<String>,
    #[serde(default, deserialize_with="parse_to_optional_string")]
    pub float: Option<String>,
    #[serde(default, deserialize_with="parse_to_optional_string")]
    pub boolen: Option<String>,
    #[serde(default, deserialize_with="parse_to_optional_string")]
    pub spread: Option<String>,
    #[serde(default, deserialize_with="parse_to_optional_string")]
    #[serde(alias = "type")]
    pub t_type: Option<String>,
    #[serde(default, deserialize_with="parse_to_optional_string")]
    pub x: Option<String>,
    #[serde(default, deserialize_with="parse_to_optional_string")]
    pub y:Option<String>,
}

pub fn parse_to_optional_string<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrNull {
        ToStr(serde_json::Value),
        Null,
    }

    let des = StringOrNull::deserialize(deserializer)?;
  
    match des {
        StringOrNull::ToStr(values) => {
            match values {
                serde_json::Value::Null => {
                    Ok(None)
                },
                serde_json::Value::Bool(x) => {
                    Ok(Some(x.to_string()))
                },
                serde_json::Value::Number(x) => {
                    Ok(Some(x.to_string()))
                },
                serde_json::Value::String(x) => {
                    Ok(Some(x))
                },
                serde_json::Value::Array(x) => {
                    Ok(None)
                },
                serde_json::Value::Object(x) => {
                    Ok(None)
                },
            }
        },
        StringOrNull::Null => {
            Ok(None)
        },
    }
}