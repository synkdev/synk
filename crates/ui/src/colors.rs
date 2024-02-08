use anyhow::Result;

#[derive(Clone)]
pub enum Color {
    Hex(&'static str),
    Rgba(u8, u8, u8, u8),
    Rgb(u8, u8, u8),
}

impl Color {
    pub fn default_hex() -> Self {
        Color::Hex("#1e1d2d")
    }

    pub fn default_rgb() -> Self {
        Color::Rgb(30, 29, 45)
    }

    pub fn default_rgba() -> Self {
        Color::Rgba(30, 29, 45, 100)
    }
    pub fn into(self) -> Result<String> {
        match self {
            Color::Hex(hex) => {
                let hex = hex.strip_prefix("#").unwrap_or(hex);

                if hex.len() != 6 && hex.len() != 3 && hex.len() != 8 {
                    return Err(anyhow::anyhow!("Invalid length for hex String. Must be 3, 6 or 8 characters long (excluding the # symbol if any)"));
                } else if hex.len() == 3 {
                    let red =
                        u8::from_str_radix(&format!("{}{}", &hex[0..1], &hex[0..1]), 16).unwrap();
                    let green =
                        u8::from_str_radix(&format!("{}{}", &hex[1..2], &hex[1..2]), 16).unwrap();
                    let blue =
                        u8::from_str_radix(&format!("{}{}", &hex[2..3], &hex[2..3]), 16).unwrap();

                    return Ok(format!("rgb({}, {}, {})", red, green, blue));
                } else if hex.len() == 6 {
                    let red = u8::from_str_radix(&hex[0..2], 16).unwrap();
                    let green = u8::from_str_radix(&hex[2..4], 16).unwrap();
                    let blue = u8::from_str_radix(&hex[4..6], 16).unwrap();

                    return Ok(format!("rgb({}, {}, {})", red, green, blue));
                } else if hex.len() == 8 {
                    let red = u8::from_str_radix(&hex[0..2], 16).unwrap();
                    let green = u8::from_str_radix(&hex[2..4], 16).unwrap();
                    let blue = u8::from_str_radix(&hex[4..6], 16).unwrap();
                    let alpha = u8::from_str_radix(&hex[6..8], 16).unwrap();

                    return Ok(format!("rgb({}, {}, {}, {})", red, green, blue, alpha));
                }

                Ok(format!("rgb({}, {}, {})", 0, 0, 0))
            }
            Color::Rgb(red, green, blue) => Ok(format!("rgb({}, {}, {})", red, green, blue)),
            Color::Rgba(red, green, blue, alpha) => {
                Ok(format!("rgb({}, {}, {}, {})", red, green, blue, alpha))
            }
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct Colors {
    pub background: String,
    pub foreground: String,
    pub line_numbers: LineNumberColors,
    pub sidebar: SidebarColors,
    pub separator: SeparatorColors,
    pub statusbar: StatusbarColors,
    pub editor: EditorColors,
    pub tab_bar: TabBarColors,
}

#[derive(Clone, PartialEq)]
pub struct LineNumberColors {
    pub line_numbers_bg: String,
    pub line_numbers_fg: String,
    pub line_numbers_edited_fg: String,
    pub line_numbers_deleted_fg: String,
    pub line_numbers_added_fg: String,
}

#[derive(Clone, PartialEq)]
pub struct SidebarColors {
    pub background: String,
    pub foreground: String,
    pub tab_bar_bg: String,
    pub tab_fg: String,
    pub tab_bg: String,
}

#[derive(Clone, PartialEq)]
pub struct SeparatorColors {
    pub default: String,
    pub active: String,
}

#[derive(Clone, PartialEq)]
pub struct StatusbarColors {
    pub background: String,
    pub foreground: String,
}

#[derive(Clone, PartialEq)]
pub struct EditorColors {
    pub background: String,
    pub foreground: String,
}

#[derive(Clone, PartialEq)]
pub struct TabBarColors {
    pub bar_bg: String,
    pub tab_bg: String,
    pub tab_fg: String,
}

impl Colors {
    pub fn new() -> Self {
        Colors {
            background: Color::Hex("#1e1d2d").into().unwrap(),
            foreground: Color::Hex("#cdd6f4").into().unwrap(),
            line_numbers: LineNumberColors {
                line_numbers_bg: Color::Hex("#1e1d2d").into().unwrap(),
                line_numbers_fg: Color::Hex("#6c7086").into().unwrap(),
                line_numbers_added_fg: Color::Hex("#a6e3a1").into().unwrap(),
                line_numbers_edited_fg: Color::Hex("#f9e2af").into().unwrap(),
                line_numbers_deleted_fg: Color::Hex("#f38ba8").into().unwrap(),
            },
            sidebar: SidebarColors {
                background: Color::Hex("#1e1d2d").into().unwrap(),
                foreground: Color::Hex("#cdd6f4").into().unwrap(),
                tab_fg: Color::Hex("#cdd6f4").into().unwrap(),
                tab_bar_bg: Color::Hex("#1e1d2d").into().unwrap(),
                tab_bg: Color::Hex("#1e1d2d").into().unwrap(),
            },
            separator: SeparatorColors {
                default: Color::Hex("#45475a").into().unwrap(),
                active: Color::Hex("#9399b2").into().unwrap(),
            },
            statusbar: StatusbarColors {
                background: Color::Hex("#1e1d2d").into().unwrap(),
                foreground: Color::Hex("#cdd6f4").into().unwrap(),
            },
            editor: EditorColors {
                background: Color::Hex("#1e1d2d").into().unwrap(),
                foreground: Color::Hex("#cdd6f4").into().unwrap(),
            },
            tab_bar: TabBarColors {
                bar_bg: Color::Hex("#1e1d2d").into().unwrap(),
                tab_bg: Color::Hex("#1e1d2d").into().unwrap(),
                tab_fg: Color::Hex("#cdd6f4").into().unwrap(),
            },
        }
    }
    pub fn into(&self) {}
}
