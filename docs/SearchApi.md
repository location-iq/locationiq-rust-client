# \SearchApi

All URIs are relative to *https://eu1.locationiq.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**search**](SearchApi.md#search) | **Get** /search.php | Forward Geocoding



## search

> Vec<crate::models::Location> search(q, format, normalizecity, addressdetails, viewbox, bounded, limit, accept_language, countrycodes, namedetails, dedupe, extratags, statecode, matchquality, postaladdress)
Forward Geocoding

The Search API allows converting addresses, such as a street address, into geographic coordinates (latitude and longitude). These coordinates can serve various use-cases, from placing markers on a map to helping algorithms determine nearby bus stops. This process is also known as Forward Geocoding.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**q** | **String** | Address to geocode | [required] |
**format** | **String** | Format to geocode. Only JSON supported for SDKs | [required] |
**normalizecity** | **i32** | For responses with no city value in the address section, the next available element in this order - city_district, locality, town, borough, municipality, village, hamlet, quarter, neighbourhood - from the address section will be normalized to city. Defaults to 1 for SDKs. | [required] |
**addressdetails** | Option<**i32**> | Include a breakdown of the address into elements. Defaults to 0. |  |
**viewbox** | Option<**String**> | The preferred area to find search results.  To restrict results to those within the viewbox, use along with the bounded option. Tuple of 4 floats. Any two corner points of the box - `max_lon,max_lat,min_lon,min_lat` or `min_lon,min_lat,max_lon,max_lat` - are accepted in any order as long as they span a real box.  |  |
**bounded** | Option<**i32**> | Restrict the results to only items contained with the viewbox |  |
**limit** | Option<**i32**> | Limit the number of returned results. Default is 10. |  |[default to 10]
**accept_language** | Option<**String**> | Preferred language order for showing search results, overrides the value specified in the Accept-Language HTTP header. Defaults to en. To use native language for the response when available, use accept-language=native |  |
**countrycodes** | Option<**String**> | Limit search to a list of countries. |  |
**namedetails** | Option<**i32**> | Include a list of alternative names in the results. These may include language variants, references, operator and brand. |  |
**dedupe** | Option<**i32**> | Sometimes you have several objects in OSM identifying the same place or object in reality. The simplest case is a street being split in many different OSM ways due to different characteristics. Nominatim will attempt to detect such duplicates and only return one match; this is controlled by the dedupe parameter which defaults to 1. Since the limit is, for reasons of efficiency, enforced before and not after de-duplicating, it is possible that de-duplicating leaves you with less results than requested. |  |
**extratags** | Option<**i32**> | Include additional information in the result if available, e.g. wikipedia link, opening hours. |  |
**statecode** | Option<**i32**> | Adds state or province code when available to the statecode key inside the address element. Currently supported for addresses in the USA, Canada and Australia. Defaults to 0 |  |
**matchquality** | Option<**i32**> | Returns additional information about quality of the result in a matchquality object. Read more Defaults to 0 [0,1] |  |
**postaladdress** | Option<**i32**> | Returns address inside the postaladdress key, that is specifically formatted for each country. Currently supported for addresses in Germany. Defaults to 0 [0,1] |  |

### Return type

[**Vec<crate::models::Location>**](location.md)

### Authorization

[key](../README.md#key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

