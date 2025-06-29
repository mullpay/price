#[derive(Debug)]
pub enum Error {
    Reqwest(reqwest::Error),
    PricesNotAvailable,
}

pub type Result<T, E = Error> = core::result::Result<T, E>;

impl From<reqwest::Error> for Error {
    #[inline(always)]
    fn from(value: reqwest::Error) -> Self {
        Self::Reqwest(value)
    }
}
