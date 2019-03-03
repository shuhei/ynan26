use failure::{Backtrace, Context, Fail};
use std::convert::From;
use std::fmt::{self, Display};
use std::result;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug, Fail, PartialEq)]
pub enum ErrorKind {
    #[fail(display = "failed to read env variable")]
    ReadEnvVar,

    #[fail(display = "failed to get transactions from YNAB")]
    YnabGetTransactions,

    #[fail(display = "failed to parse transactions from YNAB")]
    YnabParseTransactions,

    #[fail(display = "failed to authenticate against N26")]
    N26Authenticate,
}

#[derive(Debug)]
pub struct Error {
    inner: Context<ErrorKind>,
}

impl Error {
    pub fn kind(&self) -> &ErrorKind {
        &*self.inner.get_context()
    }
}

impl Fail for Error {
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        let inner = Context::new(kind);
        Error { inner }
    }
}

impl From<Context<ErrorKind>> for Error {
    fn from(inner: Context<ErrorKind>) -> Error {
        Error { inner }
    }
}
