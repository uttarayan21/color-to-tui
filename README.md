# Color -> TUI

[![build](https://img.shields.io/drone/build/uttarayan/color-to-tui?server=https%3A%2F%2Fdrone.uttarayan.me)][color-to-tui]
[![build](https://github.com/uttarayan21/color-to-tui/actions/workflows/build.yaml/badge.svg)][mirror]  

Parse HEX colors to [tui-rs](https://github.com/fdehau/tui-rs)'s [Rgb](https://docs.rs/tui/0.16.0/tui/style/enum.Color.html) colors.

## Example

<span style="color: #C3F111">`#C3F111`</span> -> `Color::Rgb(195,241,17)`

<span style="color: #CFB">`#CFB`</span> -> `Color::Rgb(204,255,187)`

<span style="color: #AFAF00">`142`</span> -> `Color::Indexed(142)`  

## Usage

```rust
#[derive(Serialize, Deserialize, PartialEq)]
sruct ColorStruct {
    #[serde(with = "color_to_tui"]
    color: tui::style::Color,
    #[serde(with = "color_to_tui::optional"]
    optional_color: Option<tui::style::Color>,
}

let color_text =  r###"{ "color" : "#12FC1C", "optional_color" : "123" }"###
let t: ColorStruct = serde_json::from_str::<ColorStruct>(color_text).unwrap();

let c = ColorStruct {
    color: Color::Rgb(18, 252, 28),
    optional_color: Option<Color::Indexed(123)>,
};

assert_eq!(t, c);
```

[color-to-tui]: https://git.uttarayan.me/uttarayan/color-to-tui
[mirror]: https://github.com/uttarayan21/color-to-tui
