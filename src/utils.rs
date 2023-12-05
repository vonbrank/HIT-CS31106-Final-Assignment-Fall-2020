use crossterm::style::Stylize;

#[derive(Clone, Copy)]
pub enum AlignType {
    Left,
    Center,
    Right,
}

pub fn align_string(content: &String, width: u32, align_type: AlignType) -> String {
    match align_type {
        AlignType::Left => {
            if content.len() >= width as usize {
                (&content[0..(width as usize)]).to_string().clone()
            } else {
                format!("{}{}", content, " ".repeat(width as usize - content.len()))
            }
        }
        AlignType::Center => {
            if content.len() >= width as usize {
                let length_diff = content.len() - width as usize;
                let left_slice_length = length_diff / 2;
                // let right_slice_length = length_diff - left_slice_length;
                (&content[left_slice_length..(left_slice_length + width as usize)])
                    .to_string()
                    .clone()
            } else {
                let length_diff = width as usize - content.len();
                let left_space_length = length_diff / 2;
                let right_space_length = length_diff - left_space_length;

                format!(
                    "{}{}{}",
                    " ".repeat(left_space_length),
                    content,
                    " ".repeat(right_space_length)
                )
            }
        }
        AlignType::Right => {
            if content.len() >= width as usize {
                let content_reverse = content.clone().reverse().to_string();
                (&content_reverse[0..(width as usize)])
                    .reverse()
                    .to_string()
            } else {
                format!("{}{}", " ".repeat(width as usize - content.len()), content)
            }
        }
    }
}

pub fn align_key_value(key: &String, value: &String, width: u32, align_type: AlignType) -> String {
    let key_width = width / 2;
    let value_width = width - key_width;

    format!(
        "{}{}",
        align_string(key, key_width, align_type),
        align_string(value, value_width, align_type)
    )
}
