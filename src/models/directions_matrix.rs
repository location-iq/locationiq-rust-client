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
pub struct DirectionsMatrix {
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "distances", skip_serializing_if = "Option::is_none")]
    pub distances: Option<Vec<f32>>,
    #[serde(rename = "fallback_speed_cells", skip_serializing_if = "Option::is_none")]
    pub fallback_speed_cells: Option<Vec<f32>>,
    #[serde(rename = "sources", skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<crate::models::DirectionsMatrixSources>>,
    #[serde(rename = "destinations", skip_serializing_if = "Option::is_none")]
    pub destinations: Option<Vec<crate::models::DirectionsMatrixSources>>,
}

impl DirectionsMatrix {
    pub fn new() -> DirectionsMatrix {
        DirectionsMatrix {
            code: None,
            distances: None,
            fallback_speed_cells: None,
            sources: None,
            destinations: None,
        }
    }
}


