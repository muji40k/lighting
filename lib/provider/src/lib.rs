
use logic::light::Light;

pub type Result<T> = std::result::Result<T, Error>;

pub trait Provider {
    fn name(self: &Self) -> &str;
    fn list(self: &Self) -> Result<Vec<Light>>;
    fn get(self: &Self, id: &str) -> Result<Light>;
    fn sync(self: &Self, light: &Light) -> Result<()>;
}

#[derive(Debug)]
pub struct Error {
    pub provider: String,
    pub etype: ErrorType,
}

#[derive(Debug)]
pub enum ErrorType {
    NotFound(String),
    IncorrectLight(Light),
    IncorrectState(Light, String),
    ForeignLight(Light),
    Internal(Box<dyn std::error::Error>),
}

impl Error {
    pub fn not_found<T>(provider: &str, id: &str) -> Result<T> {
        Err(Self {
            provider: provider.to_string(),
            etype: ErrorType::NotFound(id.to_string()),
        })
    }

    pub fn incorrect_light<T>(provider: &str, light: &Light) -> Result<T> {
        Err(Self {
            provider: provider.to_string(),
            etype: ErrorType::IncorrectLight(light.clone()),
        })
    }

    pub fn foreign_light<T>(provider: &str, light: &Light) -> Result<T> {
        Err(Self {
            provider: provider.to_string(),
            etype: ErrorType::ForeignLight(light.clone()),
        })
    }

    pub fn incorrect_state<T>(provider: &str, light: &Light, msg: String) -> Result<T> {
        Err(Self {
            provider: provider.to_string(),
            etype: ErrorType::IncorrectState(light.clone(), msg),
        })
    }

    pub fn internal<T>(provider: &str, err: Box<dyn std::error::Error>) -> Result<T> {
        Err(Self {
            provider: provider.to_string(),
            etype: ErrorType::Internal(err),
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
            Self::NotFound(id) => {
                write!(f, "Can't find light with id \"{}\"", id)
            },
            Self::IncorrectLight(light) => {
                write!(f, "Incorrect light occured --- {:?}", light)
            },
            Self::ForeignLight(light) => {
                write!(f, "Attempted to proceed light of another provider --- \
                           \"{}\"",
                       light.provider.name)
            },
            Self::IncorrectState(light, msg) => {
                write!(f, "Got light with incorrect state: \"{}\"\n\
                           Light: {:?}", msg, light)
            },
            Self::Internal(err) => {
                write!(f, "Internal error occured\n{}", err)
            },
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Provider \"{}\": {}", self.provider, self.etype)
    }
}

