use serde::{Deserialize, Deserializer, Serializer};
use tui::style::Color;

pub fn serialize<S: Serializer>(color: &Option<Color>, serializer: S) -> Result<S::Ok, S::Error> {
    use serde::ser::Error;
    if color.is_none() {
        return Err(Error::custom("color is none"));
    }
    let color = color.unwrap();
    serializer.serialize_str(&match color {
        Color::Reset => "Reset".to_string(),
        Color::Red => "Red".to_string(),
        Color::Green => "Green".to_string(),
        Color::Black => "Black".to_string(),
        Color::Yellow => "Yellow".to_string(),
        Color::Blue => "Blue".to_string(),
        Color::Magenta => "Magenta".to_string(),
        Color::Cyan => "Cyan".to_string(),
        Color::Gray => "Gray".to_string(),
        Color::White => "White".to_string(),

        Color::DarkGray => "DarkGray".to_string(),
        Color::LightBlue => "LightBlue".to_string(),
        Color::LightCyan => "LightCyan".to_string(),
        Color::LightGreen => "LightGreen".to_string(),
        Color::LightMagenta => "LightMagenta".to_string(),
        Color::LightRed => "LightRed".to_string(),
        Color::LightYellow => "LightYellow".to_string(),
        Color::Indexed(index) => format!("#{:03}", index),
        Color::Rgb(r, g, b) => format!("#{:02X}{:02X}{:02X}", r, g, b),
    })
}

pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Option<Color>, D::Error> {
    use serde::de::{Error, Unexpected};

    let mut color_string = String::deserialize(deserializer)?;

    Ok(Some(match color_string.to_lowercase().as_str() {
        "reset" => Color::Reset,
        "red" => Color::Red,
        "green" => Color::Green,
        "black" => Color::Black,
        "yellow" => Color::Yellow,
        "blue" => Color::Blue,
        "magenta" => Color::Magenta,
        "cyan" => Color::Cyan,
        "gray" => Color::Gray,
        "white" => Color::White,

        "darkgray" => Color::DarkGray,
        "lightblue" => Color::LightBlue,
        "lightcyan" => Color::LightCyan,
        "lightgreen" => Color::LightGreen,
        "lightmagenta" => Color::LightMagenta,
        "lightred" => Color::LightRed,
        "lightyellow" => Color::LightYellow,
        _ => match color_string.len() {
            4 => {
                if !color_string.starts_with('#') {
                    return Err(Error::invalid_value(
                        Unexpected::Char(color_string.chars().next().unwrap()),
                        &"# at the start",
                    ));
                }
                color_string.remove(0);

                let index = color_string.parse::<u8>();
                if let Ok(index) = index {
                    Color::Indexed(index)
                } else {
                    return Err(Error::invalid_type(
                        Unexpected::Bytes(color_string.as_bytes()),
                        &"u8 index color",
                    ));
                }
            }
            7 => {
                if !color_string.starts_with('#') {
                    return Err(Error::invalid_value(
                        Unexpected::Char(color_string.chars().next().unwrap()),
                        &"# at the start",
                    ));
                }
                let color_string = color_string.trim_start_matches('#');

                let r = u8::from_str_radix(&color_string[0..2], 16);
                let g = u8::from_str_radix(&color_string[2..4], 16);
                let b = u8::from_str_radix(&color_string[4..6], 16);

                match (r, g, b) {
                    (Ok(r), Ok(g), Ok(b)) => Color::Rgb(r, g, b),
                    (_, _, _) => {
                        return Err(Error::invalid_value(
                            Unexpected::Bytes(color_string.as_bytes()),
                            &"hex color string",
                        ));
                    }
                }
            }
            _ => {
                return Err(serde::de::Error::invalid_length(
                    color_string.len(),
                    &"color string with length 4 or 7",
                ))
            }
        },
    }))
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn ser() {
//         use super::serialize;
//         use serde::Serialize;
//         use tui::style::Color;

//         #[derive(Serialize)]
//         struct Test {
//             #[serde(serialize_with = "serialize")]
//             pub c: Color,
//         }

//         let color: Color = Color::Rgb(1, 5, 77);
//         let t = Test { c: color };
//         println!("{:?}", serde_json::to_string(&t));

//         let color: Color = Color::Indexed(128);
//         let t = Test { c: color };
//         println!("{:?}", serde_json::to_string(&t));
//     }
//     #[test]
//     fn deser() {
//         use super::deserialize;
//         use serde::Deserialize;
//         use tui::style::Color;
//         #[derive(Debug, Deserialize)]
//         struct Test {
//             #[serde(deserialize_with = "deserialize")]
//             pub c: Color,
//         }
//         let color_text = r###"{ "c": "#12fc1c" }"###;
//         let t = serde_json::from_str::<Test>(color_text);
//         println!("{:?}", t);

//         let color_text = r###"{ "c": "#123" }"###;
//         let t = serde_json::from_str::<Test>(color_text);
//         println!("{:?}", t);
//     }
// }
