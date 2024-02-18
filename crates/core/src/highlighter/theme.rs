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
    pub default: Style,
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

impl Default for SyntaxTheme {
    fn default() -> Self {
        SyntaxTheme {
            identifiers: Default::default(),
            literals: Default::default(),
            types: Default::default(),
            functions: Default::default(),
            keywords: Default::default(),
            punctuation: Default::default(),
            comments: Default::default(),
            default: Style::normal(MOCHA.text),
        }
    }
}

impl SyntaxTheme {
    pub fn get_char_style(&self, scope: String) -> Style {
        match &scope as &str {
            // Variables
            "variable" => self.identifiers.variables.default,
            "variable.builtin" => self.identifiers.variables.builtin,
            "variable.parameter" => self.identifiers.variables.parameter,
            "variable.member" => self.identifiers.variables.member,
            // Constants
            "constant" => self.identifiers.constants.default,
            "constant.builtin" => self.identifiers.constants.builtin,
            "constant.macro" => self.identifiers.constants.macro_,
            // Modules
            "module" => self.identifiers.modules.default,
            "module.builtin" => self.identifiers.modules.builtin,
            "label" => self.identifiers.label,
            // Literals
            "string" => self.literals.strings.default,
            "string.documentation" => self.literals.strings.docs,
            "string.regexp" => self.literals.strings.regexp,
            "string.escape" => self.literals.strings.escape,
            "string.special" => self.literals.strings.special,
            "string.special.symbol" => self.literals.strings.special_symbol,
            "string.special.url" => self.literals.strings.special_url,
            "string.special.path" => self.literals.strings.special_path,
            "character" => self.literals.characters.default,
            "character.special" => self.literals.characters.special,
            "boolean" => self.literals.boolean,
            "number" => self.literals.number,
            "number.float" => self.literals.float,
            // Types
            "type" => self.types.default,
            "type.builtin" => self.types.builtin,
            "type.definition" => self.types.definition,
            "type.qualifier" => self.types.qualifier,
            "attribute" => self.types.attribute,
            "property" => self.types.property,
            // Functions
            "function" => self.functions.default,
            "function.builtin" => self.functions.builtin,
            "function.call" => self.functions.call,
            "function.macro" => self.functions.macro_,
            "function.method" => self.functions.method,
            "function.method.call" => self.functions.method_call,
            "constructor" => self.functions.constructor,
            "operator" => self.functions.operator,
            // Keywords
            "keyword" => self.keywords.default,
            "keyword.coroutine" => self.keywords.coroutine,
            "keyword.function" => self.keywords.function,
            "keyword.operator" => self.keywords.operator,
            "keyword.import" => self.keywords.import,
            "keyword.storage" => self.keywords.storage,
            "keyword.repeat" => self.keywords.repeat,
            "keyword.return" => self.keywords.return_,
            "keyword.debug" => self.keywords.debug,
            "keyword.exception" => self.keywords.exception,
            "keyword.conditional" => self.keywords.conditional,
            "keyword.conditional.ternary" => self.keywords.conditional_ternary,
            "keyword.directive" => self.keywords.directive,
            "keyword.directive.define" => self.keywords.directive_define,
            // Punctuation
            "punctuation.delimiter" => self.punctuation.delimeter,
            "punctuation.bracket" => self.punctuation.bracket,
            "punctuation.special" => self.punctuation.special,
            // Comments
            "comment" => self.comments.default,
            "comment.documentation" => self.comments.docs,
            "comment.error" => self.comments.error,
            "comment.warning" => self.comments.warning,
            "comment.todo" => self.comments.todo,
            "comment.note" => self.comments.note,
            _ => self.default,
        }
    }
}
