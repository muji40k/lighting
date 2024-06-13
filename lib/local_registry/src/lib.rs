
use logic::light::Light;

pub type Result<T> = std::result::Result<T, Error>;

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
pub struct Error {
    pub registry: String,
    pub etype: ErrorType,
}

#[derive(Debug)]
pub enum ErrorType {
    NotFound(String),
    IncorrectLight(Light),
    Unnamed,
    Internal(Box<dyn std::error::Error>),
}

impl Error {
    pub fn not_found<T>(registry: &str, name: &str) -> Result<T> {
        Err(Self {
            registry: registry.to_string(),
            etype: ErrorType::NotFound(name.to_string()),
        })
    }

    pub fn incorrect_light<T>(registry: &str, light: &Light) -> Result<T> {
        Err(Self {
            registry: registry.to_string(),
            etype: ErrorType::IncorrectLight(light.clone()),
        })
    }

    pub fn internal<T>(registry: &str, err: Box<dyn std::error::Error>) -> Result<T> {
        Err(Self {
            registry: registry.to_string(),
            etype: ErrorType::Internal(err),
        })
    }

    pub fn unnamed<T>(registry: &str) -> Result<T> {
        Err(Self {
            registry: registry.to_string(),
            etype: ErrorType::Unnamed,
        })
    }
}

impl From<Error> for ErrorType {
    fn from(value: Error) -> Self {
        value.etype
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.etype {
            ErrorType::Internal(err) => Some(err.as_ref()),
            _ => None,
        }
    }
}

impl std::fmt::Display for ErrorType {
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

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Registry \"{}\": {}", self.registry, self.etype)
    }
}

