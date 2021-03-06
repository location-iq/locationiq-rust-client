/*
 * LocationIQ
 *
 * LocationIQ provides flexible enterprise-grade location based solutions. We work with developers, startups and enterprises worldwide serving billions of requests everyday. This page provides an overview of the technical aspects of our API and will help you get started.
 *
 * The version of the OpenAPI document: 1.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DirectionsNearest {
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "waypoints", skip_serializing_if = "Option::is_none")]
    pub waypoints: Option<Vec<crate::models::DirectionsNearestWaypoints>>,
}

impl DirectionsNearest {
    pub fn new() -> DirectionsNearest {
        DirectionsNearest {
            code: None,
            waypoints: None,
        }
    }
}


