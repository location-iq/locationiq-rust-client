# \ReverseApi

All URIs are relative to *https://eu1.locationiq.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**reverse**](ReverseApi.md#reverse) | **Get** /reverse.php | Reverse Geocoding


# **reverse**
> ::models::Location reverse(ctx, lat, lon, format, normalizecity, optional)
Reverse Geocoding

Reverse geocoding is the process of converting a coordinate or location (latitude, longitude) to a readable address or place name. This permits the identification of nearby street addresses, places, and/or area subdivisions such as a neighborhood, county, state, or country.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **lat** | **f32**| Latitude of the location to generate an address for. | 
  **lon** | **f32**| Longitude of the location to generate an address for. | 
  **format** | **String**| Format to geocode. Only JSON supported for SDKs | 
  **normalizecity** | **i32**| Normalizes village to city level data to city | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **lat** | **f32**| Latitude of the location to generate an address for. | 
 **lon** | **f32**| Longitude of the location to generate an address for. | 
 **format** | **String**| Format to geocode. Only JSON supported for SDKs | 
 **normalizecity** | **i32**| Normalizes village to city level data to city | 
 **addressdetails** | **i32**| Include a breakdown of the address into elements. Defaults to 1. | [default to 1]
 **accept_language** | **String**| Preferred language order for showing search results, overrides the value specified in the Accept-Language HTTP header. Defaults to en. To use native language for the response when available, use accept-language&#x3D;native | 
 **namedetails** | **i32**| Include a list of alternative names in the results. These may include language variants, references, operator and brand. | 
 **extratags** | **i32**| Include additional information in the result if available, e.g. wikipedia link, opening hours. | 
 **statecode** | **i32**| Adds state or province code when available to the statecode key inside the address element. Currently supported for addresses in the USA, Canada and Australia. Defaults to 0 | 

### Return type

[**::models::Location**](location.md)

### Authorization

[key](../README.md#key)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

