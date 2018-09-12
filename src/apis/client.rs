use std::rc::Rc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient<C: hyper::client::Connect> {
  configuration: Rc<Configuration<C>>,
  balance_api: Box<::apis::BalanceApi>,
  reverse_api: Box<::apis::ReverseApi>,
  search_api: Box<::apis::SearchApi>,
}

impl<C: hyper::client::Connect> APIClient<C> {
  pub fn new(configuration: Configuration<C>) -> APIClient<C> {
    let rc = Rc::new(configuration);

    APIClient {
      configuration: rc.clone(),
      balance_api: Box::new(::apis::BalanceApiClient::new(rc.clone())),
      reverse_api: Box::new(::apis::ReverseApiClient::new(rc.clone())),
      search_api: Box::new(::apis::SearchApiClient::new(rc.clone())),
    }
  }

  pub fn balance_api(&self) -> &::apis::BalanceApi{
    self.balance_api.as_ref()
  }

  pub fn reverse_api(&self) -> &::apis::ReverseApi{
    self.reverse_api.as_ref()
  }

  pub fn search_api(&self) -> &::apis::SearchApi{
    self.search_api.as_ref()
  }


}
