use easy_color::{ColorMix, Hex, IntoHex, HSL, HSLA, RGB, RGBA};

pub fn between<'a>(source: &'a str, start: &'a str, end: &'a str) -> &'a str {
    if let Some(start_position) = source.find(start) {
        let start_position = start_position + start.len();
        let source = &source[start_position..];
        let end_position = source.find(end).unwrap_or_default();
        return &source[..end_position];
    }
    ""
}

pub fn between_all<'a>(
    mut values: Vec<String>,
    source: &'a str,
    start: &'a str,
    end: &'a str,
) -> Vec<String> {
    let value = between(source, start, end);

    if value.is_empty() {
        return values;
    }

    values.push(value.to_string());
    let search_for = format!("{}{}{}", start, value, end);
    let search_updated = source.replace(search_for.as_str(), "");
    return between_all(values, search_updated.as_str(), start, end);
}

pub fn remove_white_spaces(value: &mut String) {
    value.retain(|c| !c.is_whitespace());
}
// Convert any color to hex
pub fn color_to_hex(color: &str) -> Hex {
    let mut hex: Hex = "#ffffff".try_into().unwrap();
    let hexResult: Result<Hex, _> = color.try_into();
    match hexResult {
        Ok(color_value_hex) => {
            hex = color_value_hex;
        }
        Err(_) => {
            let rgbResult: Result<RGBA, _> = color.try_into();
            match rgbResult {
                Ok(color_value_rgb) => {
                    hex = color_value_rgb.to_hex();
                }
                Err(_) => {
                    let rgbResult: Result<HSLA, _> = color.try_into();
                    if let Ok(color_value_hsla) = rgbResult {
                        hex = color_value_hsla.to_hex();
                    }
                }
            }
        }
    }

    //println!("hex: {}", hex);
    hex
}
