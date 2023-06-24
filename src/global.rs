use crate::deserializer::AvailableFields;

pub const global: &str = "global";

pub const type_unknown: &str = "unknown";
pub const type_color: &str = "color";
pub const type_string: &str = "string";
pub const type_float: &str = "float";
pub const type_boolean: &str = "boolean";
pub const type_typography: &str = "typography";
pub const type_spacing: &str = "spacing";
pub const type_paragraph_spacing: &str = "paragraphSpacing";
pub const type_paragraph_ident: &str = "paragraphIndent";
pub const type_text_case: &str = "textCase";
pub const type_text_decoration: &str = "textDecoration";
pub const type_borderWidth: &str = "borderWidth";
pub const type_borderRadius: &str = "borderRadius";
pub const type_letterSpacing: &str = "letterSpacing";
pub const type_composition: &str = "composition";
pub const type_lineHeights: &str = "lineHeights";
pub const type_fontSizes: &str = "fontSizes";
pub const type_fontWeights: &str = "fontWeights";
pub const type_fontFamilies: &str = "fontFamilies";
pub const type_boxShadow: &str = "boxShadow";

pub const field_variable_name: &str = "variable_name";
pub const field_description: &str = "description";
pub const field_color: &str = type_color;

pub const field_value_color: &str = type_color;
pub const field_value_font_family: &str = "fontFamily";
pub const field_value_string: &str = type_string;
pub const field_value_float: &str = type_float;
pub const field_value_boolean: &str = type_boolean;
pub const field_value_font_size: &str = "fontSize";
pub const field_value_font_weight: &str = "fontWeight";
pub const field_value_spacing: &str = "spacing";
pub const field_value_letter_spacing: &str = "letterSpacing";
pub const field_value_paragraph_spacing: &str = type_paragraph_spacing;
pub const field_value_paragraph_indent: &str = type_paragraph_ident;
pub const field_value_text_case: &str = type_text_case;
pub const field_value_text_decoration: &str = type_text_decoration;
pub const field_value_line_height: &str = "lineHeight";
pub const field_value_horizontal_padding: &str = "horizontalPadding";
pub const field_value_vertical_padding: &str = "verticalPadding";
pub const field_value_item_spacing: &str = "itemSpacing";
pub const field_value_padding_bottom: &str = "paddingBottom";
pub const field_value_padding_top: &str = "paddingTop";
pub const field_value_padding_left: &str = "paddingLeft";
pub const field_value_padding_right: &str = "paddingRight";
pub const field_value_sizing: &str = "sizing";
pub const field_value_height: &str = "height";
pub const field_value_width: &str = "width";
pub const field_value_border_radius: &str = "borderRadius";
pub const field_value_border_width: &str = "borderWidth";
pub const field_value_border_radius_bottom_left: &str = "borderRadiusBottomLeft";
pub const field_value_border_radius_bottom_right: &str = "borderRadiusBottomRight";
pub const field_value_border_radius_top_left: &str = "borderRadiusTopLeft";
pub const field_value_border_radius_top_right: &str = "borderRadiusTopRight";
pub const field_value_blur: &str = "blur";
pub const field_value_spread: &str = "spread";
pub const field_value_type: &str = "type";
pub const field_value_x: &str = "x";
pub const field_value_y: &str = "y";

pub const color_rgb_r_v1: &str = "rgb_r_v1";
pub const color_rgb_g_v1: &str = "rgb_g_v1";
pub const color_rgb_b_v1: &str = "rgb_b_v1";
pub const color_rgb_a_v1: &str = "rgb_a_v1";
pub const color_rgb_r_v2: &str = "rgb_r_v2";
pub const color_rgb_g_v2: &str = "rgb_g_v2";
pub const color_rgb_b_v2: &str = "rgb_b_v2";
pub const color_rgb_a_v2: &str = "rgb_a_v2";
pub const color_hex: &str = "hex";

pub const optional_key: &str = "optional";
pub const optional_value: &str = "%value";

