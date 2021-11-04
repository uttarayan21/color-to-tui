# Color -> Tui

[![build](https://img.shields.io/drone/build/uttarayan/color-parser-tui?server=https%3A%2F%2Fdrone.uttarayan.me)][color-parser-tui]
[![build](https://github.com/uttarayan21/color-parser-tui/actions/workflows/build.yaml/badge.svg)][mirror]  


Parse hex colors to tui rgb colors  

<span style="color: #c3f111">`#c3f111`</span> -> `Color::Rgb(195,241,17)`

Note that the indexed colors are **NOT HEX**  
<span style="color: #afaf00">`#142`</span> -> `Color::Indexed(142)`  

Example

```rust
#[derive(Serialize, Deserialize, PartialEq)]
sruct ColorStruct {
    #[serde(with = "color-parser-tui"]
    color: tui::style::Color,
    #[serde(with = "color-parser-tui::optional"]
    optional_color: Option<tui::style::Color>,
}

let color_text =  r###"{ "color" : "#12FC1C", "optional_color" : "#123" }"###
let t: ColorStruct = serde_json::from_str::<ColorStruct>(color_text).unwrap();

let c = ColorStruct {
    color: Color::Rgb(18, 252, 28),
    optional_color: Option<Color::Indexed(123)>,
};

assert_eq!(t, c);

```


[color-parser-tui]: https://git.uttarayan.me/uttarayan/color-parser-tui
[mirror]: https://github.com/uttarayan21/color-parser-tui
