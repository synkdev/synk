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
    pub fn get_color_from_scope(&self, scope: String) -> Style {
        let starts = |pattern| scope.starts_with(pattern);

        match () {
            _ if scope.is_empty() => self.default,
            _ if starts("error") => self.invalid,
            _ if starts("attribute") => self.macro_call,
            _ if starts("comment.block") => self.comment_doc,
            _ if starts("comment") => self.comment,
            _ if starts("constructor") => self.variant,
            _ if starts("constant.character") => self.character,
            _ if starts("constant.numeric") => self.function_call,
            _ if starts("constant") => self.constant,
            _ if starts("string") => self.string,
            _ if starts("function.macro") => self.macro_call,
            _ if starts("function") => self.function_call,
            _ if starts("keyword.control.import") => self.keyword,
            _ if starts("keyword") => self.keyword,
            _ if starts("punctuation") => self.punctuation,
            _ if starts("property") => self.function_call,
            _ if starts("punctuation.bracket") => self.punctuation,
            _ if starts("punctuation.delimiter") => self.punctuation,
            _ if starts("punctuation.special") => self.punctuation,
            _ if starts("punctuation") => self.default,
            _ if starts("special") => self.punctuation,
            _ if starts("table.name") => self.keyword_light,
            _ if starts("type.variant") => self.variant,
            _ if starts("type") => self.type_def,
            _ if starts("tag") => self.function_call,
            _ if starts("default.title") => self.keyword,
            _ if starts("default.emphasis") => self.keyword_light,
            _ if starts("default.strong") => self.keyword,
            _ if starts("default.literal") => self.string,
            _ if starts("default.uri") => self.punctuation,

            _ => self.default,
        }
    }
}
