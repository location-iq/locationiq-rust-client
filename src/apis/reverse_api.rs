/* 
 * LocationIQ
 *
 * LocationIQ provides flexible enterprise-grade location based solutions. We work with developers, startups and enterprises worldwide serving billions of requests everyday. This page provides an overview of the technical aspects of our API and will help you get started.
 *
 * OpenAPI spec version: 1.0.1
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

pub struct ReverseApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> ReverseApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> ReverseApiClient<C> {
        ReverseApiClient {
            configuration: configuration,
        }
    }
}

pub trait ReverseApi {
    fn reverse(&self, lat: f32, lon: f32, format: &str, normalizecity: i32, addressdetails: i32, accept_language: &str, namedetails: i32, extratags: i32) -> Box<Future<Item = ::models::Location, Error = Error<serde_json::Value>>>;
}


impl<C: hyper::client::Connect>ReverseApi for ReverseApiClient<C> {
    fn reverse(&self, lat: f32, lon: f32, format: &str, normalizecity: i32, addressdetails: i32, accept_language: &str, namedetails: i32, extratags: i32) -> Box<Future<Item = ::models::Location, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Get, "/reverse.php".to_string())
            .with_auth(__internal_request::Auth::ApiKey(__internal_request::ApiKey{
                in_header: false,
                in_query: true,
                param_name: "key".to_owned(),
            }))
            .with_query_param("lat".to_string(), lat.to_string())
            .with_query_param("lon".to_string(), lon.to_string())
            .with_query_param("format".to_string(), format.to_string())
            .with_query_param("normalizecity".to_string(), normalizecity.to_string())
            .with_query_param("addressdetails".to_string(), addressdetails.to_string())
            .with_query_param("accept-language".to_string(), accept_language.to_string())
            .with_query_param("namedetails".to_string(), namedetails.to_string())
            .with_query_param("extratags".to_string(), extratags.to_string())
            .execute(self.configuration.borrow())
    }

}
