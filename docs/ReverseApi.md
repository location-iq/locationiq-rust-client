# \ReverseApi

All URIs are relative to *https://eu1.locationiq.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**reverse**](ReverseApi.md#reverse) | **Get** /reverse.php | Reverse Geocoding



## reverse

> crate::models::Location reverse(lat, lon, format, normalizecity, addressdetails, accept_language, namedetails, extratags, statecode, showdistance, postaladdress)
Reverse Geocoding

Reverse geocoding is the process of converting a coordinate or location (latitude, longitude) to a readable address or place name. This permits the identification of nearby street addresses, places, and/or area subdivisions such as a neighborhood, county, state, or country.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lat** | **f32** | Latitude of the location to generate an address for. | [required] |
**lon** | **f32** | Longitude of the location to generate an address for. | [required] |
**format** | **String** | Format to geocode. Only JSON supported for SDKs | [required] |
**normalizecity** | **i32** | Normalizes village to city level data to city | [required] |
**addressdetails** | Option<**i32**> | Include a breakdown of the address into elements. Defaults to 1. |  |[default to 1]
**accept_language** | Option<**String**> | Preferred language order for showing search results, overrides the value specified in the Accept-Language HTTP header. Defaults to en. To use native language for the response when available, use accept-language=native |  |
**namedetails** | Option<**i32**> | Include a list of alternative names in the results. These may include language variants, references, operator and brand. |  |
**extratags** | Option<**i32**> | Include additional information in the result if available, e.g. wikipedia link, opening hours. |  |
**statecode** | Option<**i32**> | Adds state or province code when available to the statecode key inside the address element. Currently supported for addresses in the USA, Canada and Australia. Defaults to 0 |  |
**showdistance** | Option<**i32**> | Returns the straight line distance (meters) between the input location and the result's location. Value is set in the distance key of the response. Defaults to 0 [0,1] |  |
**postaladdress** | Option<**i32**> | Returns address inside the postaladdress key, that is specifically formatted for each country. Currently supported for addresses in Germany. Defaults to 0 [0,1] |  |

### Return type

[**crate::models::Location**](location.md)

### Authorization

[key](../README.md#key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

