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
            "constant.builtin" => self.default,
            "constant.macro" => self.default,
            // Modules
            "module" => self.default,
            "module.builtin" => self.default,
            "label" => self.default,
            // Literals
            "string" => self.default,
            "string.documentation" => self.default,
            "string.regexp" => self.default,
            "string.escape" => self.default,
            "string.special" => self.default,
            "string.special.symbol" => self.default,
            "string.special.url" => self.default,
            "string.special.path" => self.default,
            "character" => self.default,
            "character.special" => self.default,
            "boolean" => self.default,
            "number" => self.default,
            "number.float" => self.default,
            // Types
            "type" => self.default,
            "type.builtin" => self.default,
            "type.definition" => self.default,
            "type.qualifier" => self.default,
            "attribute" => self.default,
            "property" => self.default,
            // Functions
            "function" => self.default,
            "function.builtin" => self.default,
            "function.call" => self.default,
            "function.macro" => self.default,
            "function.method" => self.default,
            "function.method.call" => self.default,
            "constructor" => self.default,
            "operator" => self.default,
            // Keywords
            "keyword" => self.default,
            "keyword.coroutine" => self.default,
            "keyword.function" => self.default,
            "keyword.operator" => self.default,
            "keyword.import" => self.default,
            "keyword.storage" => self.default,
            "keyword.repeat" => self.default,
            "keyword.return" => self.default,
            "keyword.debug" => self.default,
            "keyword.exception" => self.default,
            "keyword.conditional" => self.default,
            "keyword.conditional.ternary" => self.default,
            "keyword.directive" => self.default,
            "keyword.directive.define" => self.default,
            // Punctuation
            "punctuation.delimiter" => self.default,
            "punctuation.bracke" => self.default,
            "punctuation.special" => self.default,
            // Comments
            "comment" => self.default,
            "comment.documentation" => self.default,
            "comment.error" => self.default,
            "comment.warning" => self.default,
            "comment.todo" => self.default,
            "comment.note" => self.default,
            _ => self.default,
        }
    }
}
