use std::backtrace::Backtrace;
use std::error::Error as StdError;
use std::fmt::{self, Debug, Display};

use crate::Error;

/// Provides the `context` method for `Result`.
pub trait Context<T, E> {
    /// Wrap the error value with additional context.
    fn context<C>(self, context: C) -> Result<T, Error>
    where
        C: Display + Send + Sync + 'static;

    /// Wrap the error value with additional context lazily.
    fn with_context<C, F>(self, f: F) -> Result<T, Error>
    where
        C: Display + Send + Sync + 'static,
        F: FnOnce() -> C;
}

impl<T, E> Context<T, E> for Result<T, E>
where
    E: StdError + Send + Sync + 'static,
{
    fn context<C>(self, context: C) -> Result<T, Error>
    where
        C: Display + Send + Sync + 'static,
    {
        self.map_err(|error| Error::from(ContextError { error, context }))
    }

    fn with_context<C, F>(self, f: F) -> Result<T, Error>
    where
        C: Display + Send + Sync + 'static,
        F: FnOnce() -> C,
    {
        self.map_err(|error| {
            Error::from(ContextError {
                context: f(),
                error,
            })
        })
    }
}

impl<T> Context<T, Error> for Result<T, Error> {
    fn context<C>(self, context: C) -> Result<T, Error>
    where
        C: Display + Send + Sync + 'static,
    {
        self.map_err(|error| Error::from(ContextError { error, context }))
    }

    fn with_context<C, F>(self, f: F) -> Result<T, Error>
    where
        C: Display + Send + Sync + 'static,
        F: FnOnce() -> C,
    {
        self.map_err(|error| {
            Error::from(ContextError {
                context: f(),
                error,
            })
        })
    }
}

struct ContextError<E, C> {
    error: E,
    context: C,
}

impl<E, C> Debug for ContextError<E, C>
where
    E: Debug,
    C: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}\n\n{}", self.error, self.context)
    }
}

impl<E, C> Display for ContextError<E, C>
where
    C: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.context, f)
    }
}

impl<E, C> StdError for ContextError<E, C>
where
    E: StdError + 'static,
    C: Display,
{
    fn backtrace(&self) -> Option<&Backtrace> {
        self.error.backtrace()
    }

    fn cause(&self) -> Option<&dyn StdError> {
        Some(&self.error)
    }

    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        Some(&self.error)
    }
}

impl<C> StdError for ContextError<Error, C>
where
    C: Display,
{
    fn backtrace(&self) -> Option<&Backtrace> {
        Some(self.error.backtrace())
    }

    fn cause(&self) -> Option<&dyn StdError> {
        Some(&*self.error)
    }

    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        Some(&*self.error)
    }
}
