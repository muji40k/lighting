
pub mod default;

pub mod fetch {
    use provider;
    use domain::light::{
        Light,
        ProviderID
    };

    pub type Result<T> = std::result::Result<T, Error>;

    pub trait FetchManager {
        fn fetch_all(self: &Self) -> Result<Vec<Light>>;
        fn fetch_provider(self: &Self, provider: &str) -> Result<Vec<Light>>;
        fn fetch(self: &Self, id: &ProviderID) -> Result<Light>;
    }

    pub trait SyncManager {
        fn sync(self: &Self, light: &Light) -> Result<()>;
    }

    #[derive(Debug)]
    pub enum Error {
        NotFound(String),
        Provider(provider::Error),
    }

    impl std::error::Error for Error {
        fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
            match self {
                Error::Provider(err) => Some(err),
                _ => None,
            }
        }
    }

    impl std::fmt::Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Error::NotFound(provider) => {
                    write!(f, "No provider named \"{provider}\" was found")
                },
                Error::Provider(err) => {
                    err.fmt(f)
                },
            }
        }
    }
}

pub mod local {
    pub use local_registry::Error;
    pub use local_registry::Result;
    use domain::light::Light;

    pub trait LocalStateManager {
        fn list_dumps(self: &Self) -> Result<Vec<Light>>;
        fn list_defaults(self: &Self) -> Result<Vec<Light>>;

        fn save(self: &mut Self, light: &Light) -> Result<()>;
        fn load(self: &Self, name: &str) -> Result<Light>;

        fn set_default(self: &mut Self, light: &Light) -> Result<()>;
        fn get_default(self: &Self, name: &str) -> Result<Light>;

        fn remove(self: &mut Self, name: &str) -> Result<()>;
        fn rename(self: &mut Self, old: &str, new: &str) -> Result<()>;
    }
}

