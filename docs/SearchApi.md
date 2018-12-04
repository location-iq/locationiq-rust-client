# \SearchApi

All URIs are relative to *https://eu1.locationiq.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**search**](SearchApi.md#search) | **Get** /search.php | Forward Geocoding


# **search**
> Vec<::models::Location> search(ctx, q, format, normalizecity, optional)
Forward Geocoding

The Search API allows converting addresses, such as a street address, into geographic coordinates (latitude and longitude). These coordinates can serve various use-cases, from placing markers on a map to helping algorithms determine nearby bus stops. This process is also known as Forward Geocoding.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **q** | **String**| Address to geocode | 
  **format** | **String**| Format to geocode. Only JSON supported for SDKs | 
  **normalizecity** | **i32**| For responses with no city value in the address section, the next available element in this order - city_district, locality, town, borough, municipality, village, hamlet, quarter, neighbourhood - from the address section will be normalized to city. Defaults to 1 for SDKs. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **q** | **String**| Address to geocode | 
 **format** | **String**| Format to geocode. Only JSON supported for SDKs | 
 **normalizecity** | **i32**| For responses with no city value in the address section, the next available element in this order - city_district, locality, town, borough, municipality, village, hamlet, quarter, neighbourhood - from the address section will be normalized to city. Defaults to 1 for SDKs. | 
 **addressdetails** | **i32**| Include a breakdown of the address into elements. Defaults to 0. | 
 **viewbox** | **String**| The preferred area to find search results.  To restrict results to those within the viewbox, use along with the bounded option. Tuple of 4 floats. Any two corner points of the box - &#x60;max_lon,max_lat,min_lon,min_lat&#x60; or &#x60;min_lon,min_lat,max_lon,max_lat&#x60; - are accepted in any order as long as they span a real box.  | 
 **bounded** | **i32**| Restrict the results to only items contained with the viewbox | 
 **limit** | **i32**| Limit the number of returned results. Default is 10. | [default to 10]
 **accept_language** | **String**| Preferred language order for showing search results, overrides the value specified in the Accept-Language HTTP header. Defaults to en. To use native language for the response when available, use accept-language&#x3D;native | 
 **countrycodes** | **String**| Limit search to a list of countries. | 
 **namedetails** | **i32**| Include a list of alternative names in the results. These may include language variants, references, operator and brand. | 
 **dedupe** | **i32**| Sometimes you have several objects in OSM identifying the same place or object in reality. The simplest case is a street being split in many different OSM ways due to different characteristics. Nominatim will attempt to detect such duplicates and only return one match; this is controlled by the dedupe parameter which defaults to 1. Since the limit is, for reasons of efficiency, enforced before and not after de-duplicating, it is possible that de-duplicating leaves you with less results than requested. | 
 **extratags** | **i32**| Include additional information in the result if available, e.g. wikipedia link, opening hours. | 
 **statecode** | **i32**| Adds state or province code when available to the statecode key inside the address element. Currently supported for addresses in the USA, Canada and Australia. Defaults to 0 | 

### Return type

[**Vec<::models::Location>**](location.md)

### Authorization

[key](../README.md#key)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

