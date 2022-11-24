use yaml_rust::Yaml;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub struct Error {
    kind: ErrorKind,
    inner_error: Option<Box<Error>>,
}

impl Error {
    pub fn from_yaml_parser(type_name: &'static str, node: Option<Yaml>, inner_error: Option<Box<Error>>) -> Self {
        Self {
            kind: ErrorKind::YamlParserError { type_name, node },
            inner_error,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            ErrorKind::YamlParserError { type_name, node } => {
                write!(f, "An error occurred while parsing {}.", type_name)
            }
            _ => write!(f, "An error occurred."),
        }
    }
}

impl std::error::Error for Error {}

#[derive(Debug, Clone)]
pub enum ErrorKind {
    YamlParserError {
        type_name: &'static str,
        node: Option<Yaml>,
    },
}
