/* 
 * LocationIQ
 *
 * LocationIQ provides flexible enterprise-grade location based solutions. We work with developers, startups and enterprises worldwide serving billions of requests everyday. This page provides an overview of the technical aspects of our API and will help you get started.
 *
 * OpenAPI spec version: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use std::rc::Rc;
use std::borrow::Borrow;

use hyper;
use serde_json;
use futures::Future;

use super::{Error, configuration};
use super::request as __internal_request;

pub struct SearchApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> SearchApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> SearchApiClient<C> {
        SearchApiClient {
            configuration: configuration,
        }
    }
}

pub trait SearchApi {
    fn search(&self, q: &str, format: &str, normalizecity: i32, addressdetails: i32, viewbox: &str, bounded: i32, limit: i32, accept_language: &str, countrycodes: &str, namedetails: i32, dedupe: i32, extratags: i32) -> Box<Future<Item = Vec<::models::Location>, Error = Error<serde_json::Value>>>;
}


impl<C: hyper::client::Connect>SearchApi for SearchApiClient<C> {
    fn search(&self, q: &str, format: &str, normalizecity: i32, addressdetails: i32, viewbox: &str, bounded: i32, limit: i32, accept_language: &str, countrycodes: &str, namedetails: i32, dedupe: i32, extratags: i32) -> Box<Future<Item = Vec<::models::Location>, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Get, "/search.php".to_string())
            .with_auth(__internal_request::Auth::ApiKey(__internal_request::ApiKey{
                in_header: false,
                in_query: true,
                param_name: "key".to_owned(),
            }))
            .with_query_param("q".to_string(), q.to_string())
            .with_query_param("format".to_string(), format.to_string())
            .with_query_param("normalizecity".to_string(), normalizecity.to_string())
            .with_query_param("addressdetails".to_string(), addressdetails.to_string())
            .with_query_param("viewbox".to_string(), viewbox.to_string())
            .with_query_param("bounded".to_string(), bounded.to_string())
            .with_query_param("limit".to_string(), limit.to_string())
            .with_query_param("accept-language".to_string(), accept_language.to_string())
            .with_query_param("countrycodes".to_string(), countrycodes.to_string())
            .with_query_param("namedetails".to_string(), namedetails.to_string())
            .with_query_param("dedupe".to_string(), dedupe.to_string())
            .with_query_param("extratags".to_string(), extratags.to_string())
            .execute(self.configuration.borrow())
    }

}