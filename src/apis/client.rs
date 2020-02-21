use std::rc::Rc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient {
    autocomplete_api: Box<dyn crate::apis::AutocompleteApi>,
    balance_api: Box<dyn crate::apis::BalanceApi>,
    directions_api: Box<dyn crate::apis::DirectionsApi>,
    matching_api: Box<dyn crate::apis::MatchingApi>,
    matrix_api: Box<dyn crate::apis::MatrixApi>,
    nearest_api: Box<dyn crate::apis::NearestApi>,
    reverse_api: Box<dyn crate::apis::ReverseApi>,
    search_api: Box<dyn crate::apis::SearchApi>,
}

impl APIClient {
    pub fn new<C: hyper::client::Connect>(configuration: Configuration<C>) -> APIClient {
        let rc = Rc::new(configuration);

        APIClient {
            autocomplete_api: Box::new(crate::apis::AutocompleteApiClient::new(rc.clone())),
            balance_api: Box::new(crate::apis::BalanceApiClient::new(rc.clone())),
            directions_api: Box::new(crate::apis::DirectionsApiClient::new(rc.clone())),
            matching_api: Box::new(crate::apis::MatchingApiClient::new(rc.clone())),
            matrix_api: Box::new(crate::apis::MatrixApiClient::new(rc.clone())),
            nearest_api: Box::new(crate::apis::NearestApiClient::new(rc.clone())),
            reverse_api: Box::new(crate::apis::ReverseApiClient::new(rc.clone())),
            search_api: Box::new(crate::apis::SearchApiClient::new(rc.clone())),
        }
    }

    pub fn autocomplete_api(&self) -> &dyn crate::apis::AutocompleteApi{
        self.autocomplete_api.as_ref()
    }

    pub fn balance_api(&self) -> &dyn crate::apis::BalanceApi{
        self.balance_api.as_ref()
    }

    pub fn directions_api(&self) -> &dyn crate::apis::DirectionsApi{
        self.directions_api.as_ref()
    }

    pub fn matching_api(&self) -> &dyn crate::apis::MatchingApi{
        self.matching_api.as_ref()
    }

    pub fn matrix_api(&self) -> &dyn crate::apis::MatrixApi{
        self.matrix_api.as_ref()
    }

    pub fn nearest_api(&self) -> &dyn crate::apis::NearestApi{
        self.nearest_api.as_ref()
    }

    pub fn reverse_api(&self) -> &dyn crate::apis::ReverseApi{
        self.reverse_api.as_ref()
    }

    pub fn search_api(&self) -> &dyn crate::apis::SearchApi{
        self.search_api.as_ref()
    }

}
