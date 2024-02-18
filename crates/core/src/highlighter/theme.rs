use crate::default_themes::catppuccin::Catppuccin;

const MOCHA: Catppuccin = Catppuccin::mocha();

#[derive(Clone, Copy, Debug)]
pub struct Style {
    pub color: &'static str,
    pub background: &'static str,
    pub weight: &'static str,
}

pub struct SyntaxTheme {
    pub identifiers: Identifiers,
    pub literals: Literals,
    pub types: Types,
    pub functions: Functions,
    pub keywords: Keywords,
    pub punctuation: Punctuation,
    pub comments: Comments,
}

pub struct Identifiers {
    pub variables: Variables,
    pub constants: Constants,
    pub modules: Modules,
    pub label: Style,
}

pub struct Literals {
    pub strings: Strings,
    pub characters: Characters,
    pub boolean: Style,
    pub number: Style,
    pub float: Style,
}

pub struct Variables {
    pub default: Style,
    pub builtin: Style,
    pub parameter: Style,
    pub member: Style,
}

pub struct Constants {
    pub default: Style,
    pub builtin: Style,
    pub macro_: Style,
}

pub struct Modules {
    pub default: Style,
    pub builtin: Style,
}

pub struct Strings {
    pub default: Style,
    pub docs: Style,
    pub regexp: Style,
    pub escape: Style,
    pub special: Style,
    pub special_symbol: Style,
    pub special_url: Style,
    pub special_path: Style,
}

pub struct Characters {
    pub default: Style,
    pub special: Style,
}

pub struct Types {
    pub default: Style,
    pub builtin: Style,
    pub definition: Style,
    pub qualifier: Style,
    pub attribute: Style,
    pub property: Style,
}

pub struct Functions {
    pub default: Style,
    pub builtin: Style,
    pub call: Style,
    pub macro_: Style,
    pub method: Style,
    pub method_call: Style,
    pub constructor: Style,
    pub operator: Style,
}

pub struct Keywords {
    pub default: Style,
    pub coroutine: Style,
    pub function: Style,
    pub operator: Style,
    pub import: Style,
    pub storage: Style,
    pub repeat: Style,
    pub return_: Style,
    pub debug: Style,
    pub exception: Style,
    pub conditional: Style,
    pub conditional_ternary: Style,
    pub directive: Style,
    pub directive_define: Style,
}

pub struct Punctuation {
    pub delimeter: Style,
    pub bracket: Style,
    pub special: Style,
}

pub struct Comments {
    pub default: Style,
    pub docs: Style,
    pub error: Style,
    pub warning: Style,
    pub todo: Style,
    pub note: Style,
}

impl Style {
    pub fn normal(color: &'static str) -> Self {
        Style {
            color,
            background: "rgb(30, 29, 45)",
            weight: "normal",
        }
    }
    pub fn inverted(background: &'static str) -> Self {
        Style {
            background,
            color: "rgb(30, 29, 45)",
            weight: "normal",
        }
    }
    pub fn bold(color: &'static str) -> Self {
        Style {
            color,
            background: "rgb(30, 29, 45)",
            weight: "bold",
        }
    }
}

impl Default for Identifiers {
    fn default() -> Self {
        Identifiers {
            variables: Variables {
                default: Style::normal(MOCHA.text),
                builtin: Style::normal(MOCHA.red),
                parameter: Style::normal(MOCHA.maroon),
                member: Style::normal(MOCHA.lavender),
            },
            constants: Constants {
                default: Style::normal(MOCHA.overlay0),
                builtin: Style::normal(MOCHA.peach),
                macro_: Style::normal(MOCHA.mauve),
            },
            modules: Modules {
                default: Style::normal(MOCHA.lavender),
                builtin: Style::normal(MOCHA.lavender),
            },
            label: Style::normal(MOCHA.sapphire),
        }
    }
}

impl Default for Literals {
    fn default() -> Self {
        Literals {
            strings: Strings {
                default: Style::normal(MOCHA.green),
                docs: Style::normal(MOCHA.green),
                regexp: Style::normal(MOCHA.peach),
                escape: Style::normal(MOCHA.pink),
                special: Style::normal(MOCHA.pink),
                special_symbol: Style::normal(MOCHA.flamingo),
                special_url: Style::normal(MOCHA.rosewater),
                special_path: Style::normal(MOCHA.rosewater),
            },
            characters: Characters {
                default: Style::normal(MOCHA.teal),
                special: Style::normal(MOCHA.pink),
            },
            boolean: Style::normal(MOCHA.peach),
            number: Style::normal(MOCHA.peach),
            float: Style::normal(MOCHA.peach),
        }
    }
}

impl Default for Types {
    fn default() -> Self {
        Types {
            default: Style::normal(MOCHA.yellow),
            builtin: Style::normal(MOCHA.yellow),
            definition: Style::normal(MOCHA.yellow),
            qualifier: Style::normal(MOCHA.mauve),
            attribute: Style::normal(MOCHA.teal),
            property: Style::normal(MOCHA.lavender),
        }
    }
}

impl Default for Functions {
    fn default() -> Self {
        Functions {
            default: Style::normal(MOCHA.blue),
            builtin: Style::normal(MOCHA.peach),
            call: Style::normal(MOCHA.blue),
            macro_: Style::normal(MOCHA.teal),
            method: Style::normal(MOCHA.blue),
            method_call: Style::normal(MOCHA.blue),
            constructor: Style::normal(MOCHA.sapphire),
            operator: Style::normal(MOCHA.sky),
        }
    }
}

impl Default for Keywords {
    fn default() -> Self {
        Keywords {
            default: Style::normal(MOCHA.mauve),
            coroutine: Style::normal(MOCHA.mauve),
            function: Style::normal(MOCHA.mauve),
            operator: Style::normal(MOCHA.mauve),
            import: Style::normal(MOCHA.mauve),
            storage: Style::normal(MOCHA.yellow),
            repeat: Style::normal(MOCHA.mauve),
            return_: Style::normal(MOCHA.mauve),
            debug: Style::normal(MOCHA.mauve),
            exception: Style::normal(MOCHA.mauve),
            conditional: Style::normal(MOCHA.mauve),
            conditional_ternary: Style::normal(MOCHA.mauve),
            directive: Style::normal(MOCHA.pink),
            directive_define: Style::normal(MOCHA.pink),
        }
    }
}

impl Default for Punctuation {
    fn default() -> Self {
        Punctuation {
            delimeter: Style::normal(MOCHA.overlay2),
            bracket: Style::normal(MOCHA.overlay2),
            special: Style::normal(MOCHA.pink),
        }
    }
}

impl Default for Comments {
    fn default() -> Self {
        Comments {
            default: Style::normal(MOCHA.overlay0),
            docs: Style::normal(MOCHA.overlay0),
            error: Style::inverted(MOCHA.red),
            warning: Style::inverted(MOCHA.yellow),
            todo: Style::inverted(MOCHA.blue),
            note: Style::inverted(MOCHA.flamingo),
        }
    }
}

impl SyntaxTheme {
    pub fn default() -> Self {
        SyntaxTheme {
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
