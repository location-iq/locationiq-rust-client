/*
 * LocationIQ
 *
 * LocationIQ provides flexible enterprise-grade location based solutions. We work with developers, startups and enterprises worldwide serving billions of requests everyday. This page provides an overview of the technical aspects of our API and will help you get started.
 *
 * The version of the OpenAPI document: 1.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use std::rc::Rc;
use std::borrow::Borrow;
#[allow(unused_imports)]
use std::option::Option;

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
            configuration,
        }
    }
}

pub trait ReverseApi {
    fn reverse(&self, lat: f32, lon: f32, format: &str, normalizecity: i32, addressdetails: Option<i32>, accept_language: Option<&str>, namedetails: Option<i32>, extratags: Option<i32>, statecode: Option<i32>, showdistance: Option<i32>, postaladdress: Option<i32>) -> Box<dyn Future<Item = crate::models::Location, Error = Error<serde_json::Value>>>;
}

impl<C: hyper::client::Connect>ReverseApi for ReverseApiClient<C> {
    fn reverse(&self, lat: f32, lon: f32, format: &str, normalizecity: i32, addressdetails: Option<i32>, accept_language: Option<&str>, namedetails: Option<i32>, extratags: Option<i32>, statecode: Option<i32>, showdistance: Option<i32>, postaladdress: Option<i32>) -> Box<dyn Future<Item = crate::models::Location, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/reverse.php".to_string())
            .with_auth(__internal_request::Auth::ApiKey(__internal_request::ApiKey{
                in_header: false,
                in_query: true,
                param_name: "key".to_owned(),
            }))
        ;
        req = req.with_query_param("lat".to_string(), lat.to_string());
        req = req.with_query_param("lon".to_string(), lon.to_string());
        req = req.with_query_param("format".to_string(), format.to_string());
        req = req.with_query_param("normalizecity".to_string(), normalizecity.to_string());
        if let Some(ref s) = addressdetails {
            req = req.with_query_param("addressdetails".to_string(), s.to_string());
        }
        if let Some(ref s) = accept_language {
            req = req.with_query_param("accept-language".to_string(), s.to_string());
        }
        if let Some(ref s) = namedetails {
            req = req.with_query_param("namedetails".to_string(), s.to_string());
        }
        if let Some(ref s) = extratags {
            req = req.with_query_param("extratags".to_string(), s.to_string());
        }
        if let Some(ref s) = statecode {
            req = req.with_query_param("statecode".to_string(), s.to_string());
        }
        if let Some(ref s) = showdistance {
            req = req.with_query_param("showdistance".to_string(), s.to_string());
        }
        if let Some(ref s) = postaladdress {
            req = req.with_query_param("postaladdress".to_string(), s.to_string());
        }

        req.execute(self.configuration.borrow())
    }

}
