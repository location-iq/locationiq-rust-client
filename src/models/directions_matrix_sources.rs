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
pub struct DirectionsMatrixSources {
    #[serde(rename = "distance", skip_serializing_if = "Option::is_none")]
    pub distance: Option<f32>,
    #[serde(rename = "location", skip_serializing_if = "Option::is_none")]
    pub location: Option<Vec<f32>>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl DirectionsMatrixSources {
    pub fn new() -> DirectionsMatrixSources {
        DirectionsMatrixSources {
            distance: None,
            location: None,
            name: None,
        }
    }
}

