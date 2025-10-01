use crate::PandoraError;

/// Extension trait to add context to Results
pub trait ErrorContext<T> {
    fn context<C>(self, context: C) -> Result<T, PandoraError>
    where
        C: Into<String>;

    fn with_context<C, F>(self, f: F) -> Result<T, PandoraError>
    where
        C: Into<String>,
        F: FnOnce() -> C;
}

impl<T, E> ErrorContext<T> for Result<T, E>
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn context<C>(self, context: C) -> Result<T, PandoraError>
    where
        C: Into<String>,
    {
        self.map_err(|e| PandoraError::config_with_source(context.into(), e))
    }

    fn with_context<C, F>(self, f: F) -> Result<T, PandoraError>
    where
        C: Into<String>,
        F: FnOnce() -> C,
    {
        self.map_err(|e| PandoraError::config_with_source(f().into(), e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error as _;

    #[test]
    fn test_error_context() {
        let result: Result<(), std::io::Error> =
            Err(std::io::Error::new(std::io::ErrorKind::NotFound, "file not found"));

        let with_context = result.context("Failed to read config file");
        assert!(with_context.is_err());

        let err = with_context.unwrap_err();
        assert!(err.to_string().contains("Failed to read config file"));
        assert!(err.source().is_some());
    }

    #[test]
    fn test_lazy_context() {
        let filename = "config.json";
        let result: Result<(), std::io::Error> =
            Err(std::io::Error::new(std::io::ErrorKind::NotFound, "not found"));

        let with_context = result.with_context(|| format!("Failed to read {}", filename));
        assert!(with_context.is_err());
    }
}


