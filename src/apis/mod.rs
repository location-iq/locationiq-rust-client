use hyper;
use serde;
use serde_json;

#[derive(Debug)]
pub enum Error<T> {
    UriError(hyper::error::UriError),
    Hyper(hyper::Error),
    Serde(serde_json::Error),
    ApiError(ApiError<T>),
}

#[derive(Debug)]
pub struct ApiError<T> {
    pub code: hyper::StatusCode,
    pub content: Option<T>,
}

impl<'de, T> From<(hyper::StatusCode, &'de [u8])> for Error<T> 
    where T: serde::Deserialize<'de> {
    fn from(e: (hyper::StatusCode, &'de [u8])) -> Self {
        if e.1.len() == 0 {
            return Error::ApiError(ApiError{
                code: e.0,
                content: None,
            });
        }
        match serde_json::from_slice::<T>(e.1) {
            Ok(t) => Error::ApiError(ApiError{
                code: e.0,
                content: Some(t),
            }),
            Err(e) => {
                Error::from(e)
            }
        }
    }
}

impl<T> From<hyper::Error> for Error<T> {
    fn from(e: hyper::Error) -> Self {
        return Error::Hyper(e)
    }
}

impl<T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        return Error::Serde(e)
    }
}

mod request;

mod autocomplete_api;
pub use self::autocomplete_api::{ AutocompleteApi, AutocompleteApiClient };
mod balance_api;
pub use self::balance_api::{ BalanceApi, BalanceApiClient };
mod directions_api;
pub use self::directions_api::{ DirectionsApi, DirectionsApiClient };
mod matching_api;
pub use self::matching_api::{ MatchingApi, MatchingApiClient };
mod matrix_api;
pub use self::matrix_api::{ MatrixApi, MatrixApiClient };
mod nearest_api;
pub use self::nearest_api::{ NearestApi, NearestApiClient };
mod reverse_api;
pub use self::reverse_api::{ ReverseApi, ReverseApiClient };
mod search_api;
pub use self::search_api::{ SearchApi, SearchApiClient };

pub mod configuration;
pub mod client;
