
pub fn between<'a>(source: &'a str, start: &'a str, end: &'a str) -> &'a str {

    if let Some(start_position) = source.find(start) {
        let start_position = start_position + start.len();
        let source = &source[start_position..];
        let end_position = source.find(end).unwrap_or_default();
        return &source[..end_position];
    }
    ""
}

pub fn between_all<'a>(mut values: Vec<String>, source: &'a str, start: &'a str, end: &'a str) -> Vec<String> {
    
    let value = between(source, start, end);

    if value.is_empty() {
        return values;
    }

    values.push(value.to_string());
    let search_for = format!("{}{}{}", start, value, end);
    let search_updated = source.replace(search_for.as_str(), "");
    return between_all(values, search_updated.as_str(), start, end)
}

pub fn remove_white_spaces(value: &mut String) { 
    value.retain(|c| !c.is_whitespace());
}
