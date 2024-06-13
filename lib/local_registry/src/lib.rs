
use logic::light::Light;

pub type Result<'a, T> = std::result::Result<T, Error<'a>>;

pub trait Registry {
    fn name(self: &Self) -> &str;
    fn list_defaults(self: &Self) -> Result<Vec<Light>>;
    fn list_dumps(self: &Self) -> Result<Vec<Light>>;
    fn load_default(self: &Self, name: &str) -> Result<Light>;
    fn load_dump(self: &Self, name: &str) -> Result<Light>;
    fn dump(self: &mut Self, light: &Light) -> Result<()>;
    fn default(self: &mut Self, light: &Light) -> Result<()>;
    fn remove_dump(self: &mut Self, name: &str) -> Result<()>;
    fn remove_default(self: &mut Self, name: &str) -> Result<()>;
    fn rename_dump(self: &mut Self, old: &str, new: &str) -> Result<()>;
    fn rename_default(self: &mut Self, old: &str, new: &str) -> Result<()>;
}

#[derive(Debug)]
pub struct Error<'a> {
    pub registry: &'a str,
    pub etype: ErrorType<'a>,
}

#[derive(Debug)]
pub enum ErrorType<'a> {
    NotFound(String),
    IncorrectLight(&'a Light),
    Unnamed,
    Internal(Box<dyn std::error::Error>),
}

impl<'a> Error<'a> {
    pub fn not_found<T>(registry: &'a str, name: &str) -> Result<'a, T> {
        Err(Self {
            registry,
            etype: ErrorType::NotFound(name.to_string()),
        })
    }

    pub fn incorrect_light<T>(registry: &'a str, light: &'a Light) -> Result<'a, T> {
        Err(Self {
            registry,
            etype: ErrorType::IncorrectLight(light),
        })
    }

    pub fn internal<T>(registry: &'a str, err: Box<dyn std::error::Error>) -> Result<'a, T> {
        Err(Self {
            registry,
            etype: ErrorType::Internal(err),
        })
    }

    pub fn unnamed<T>(registry: &'a str) -> Result<'a, T> {
        Err(Self {
            registry,
            etype: ErrorType::Unnamed,
        })
    }
}

impl<'a> From<Error<'a>> for ErrorType<'a> {
    fn from(value: Error<'a>) -> Self {
        value.etype
    }
}

impl<'a> std::error::Error for Error<'a> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.etype {
            ErrorType::Internal(err) => Some(err.as_ref()),
            _ => None,
        }
    }
}

impl<'a> std::fmt::Display for ErrorType<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound(name) => {
                write!(f, "Can't find light named \"{}\"", name)
            },
            Self::IncorrectLight(light) => {
                write!(f, "Incorrect light occured --- {:?}", light)
            },
            Self::Unnamed => {
                write!(f, "Local registry can't manage unnamed lights")
            }
            Self::Internal(err) => {
                write!(f, "Internal error occured\n{}", err)
            },
        }
    }
}

impl<'a> std::fmt::Display for Error<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Registry \"{}\": {}", self.registry, self.etype)
    }
}

