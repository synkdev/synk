#[derive(Clone, Copy, Debug)]
pub struct Style {
    pub color: &'static str,
    pub background: &'static str,
    pub weight: &'static str,
}

pub struct Theme {
    pub character: Style,
    pub comment: Style,
    pub comment_doc: Style,
    pub constant: Style,
    pub function_call: Style,
    pub invalid: Style,
    pub keyword: Style,
    pub keyword_light: Style,
    pub link: Style,
    pub macro_call: Style,
    pub punctuation: Style,
    pub string: Style,
    pub type_def: Style,
    pub variant: Style,
    pub default: Style,
}

impl Theme {
    pub fn default() -> Self {
        Theme {
            default: Style {
                color: "rgb(205, 214, 244)",
                background: "rgb(30, 29, 45)",
                weight: "normal",
            },
            character: Style {
                color: "rgb(203, 166, 247)",
                background: "rgb(30, 29, 45)",
                weight: "normal",
            },
            comment: Style {
                color: "rgb(108, 112, 134)",
                background: "rgb(30, 29, 45)",
                weight: "normal",
            },
            comment_doc: Style {
                color: "rgb(108, 112, 134)",
                background: "rgb(30, 29, 45)",
                weight: "normal",
            },
            constant: Style {
                color: "rgb(250, 179, 135)",
                background: "rgb(30, 29, 45)",
                weight: "normal",
            },
            function_call: Style {
                color: "rgb(137, 180, 250)",
                background: "rgb(30, 29, 45)",
                weight: "normal",
            },
            invalid: Style {
                color: "rgb(243, 139, 168)",
                background: "rgb(30, 29, 45)",
                weight: "normal",
            },
            keyword: Style {
                color: "rgb(203, 166, 247)",
                background: "rgb(30, 29, 45)",
                weight: "bold",
            },
            keyword_light: Style {
                color: "rgb(180, 190, 254)",
                background: "rgb(30, 29, 45)",
                weight: "normal",
            },
            link: Style {
                color: "rgb(242, 205, 205)",
                background: "rgb(30, 29, 45)",
                weight: "normal",
            },
            macro_call: Style {
                color: "rgb(203, 166, 247)",
                background: "rgb(30, 29, 45)",
                weight: "normal",
            },
            punctuation: Style {
                color: "rgb(148, 226, 213)",
                background: "rgb(30, 29, 45)",
                weight: "normal",
            },
            string: Style {
                color: "rgb(166, 227, 161)",
                background: "rgb(30, 29, 45)",
                weight: "normal",
            },
            type_def: Style {
                color: "rgb(249, 226, 175)",
                background: "rgb(30, 29, 45)",
                weight: "italic",
            },
            variant: Style {
                color: "rgb(148, 226, 213)",
                background: "rgb(30, 29, 45)",
                weight: "italic",
            },
        }
    }
    pub fn get_char_style(&self, scope: String) -> Style {
        match &scope as &str {
            // Variables
            "variable" => self.default,
            "variable.builtin" => self.default,
            "variable.parameter" => self.default,
            "variable.member" => self.default,
            // Constants
            "constant" => self.default,
            "" => self.default,
            "" => self.default,
            "" => self.default,
            "" => self.default,
            "" => self.default,
            "" => self.default,
            "" => self.default,
            "" => self.default,
            "" => self.default,
            "" => self.default,
            "" => self.default,
            "" => self.default,
            "" => self.default,
            "" => self.default,
            "" => self.default,
            "" => self.default,
            "" => self.default,
            "" => self.default,
            "" => self.default,
            "" => self.default,
            "" => self.default,
            "" => self.default,
            "" => self.default,
            _ => self.default,
        }
    }
}
