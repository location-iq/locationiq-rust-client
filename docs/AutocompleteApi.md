# LocationIq\AutocompleteApi

All URIs are relative to *https://eu1.locationiq.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**autocomplete**](AutocompleteApi.md#autocomplete) | **Get** /autocomplete.php | 



## autocomplete

> Vec<serde_json::Value> autocomplete(q, normalizecity, limit, viewbox, bounded, countrycodes, accept_language, tag)


The Autocomplete API is a variant of the Search API that returns place predictions in response to an HTTP request.  The request specifies a textual search string and optional geographic bounds.  The service can be used to provide autocomplete functionality for text-based geographic searches, by returning places such as businesses, addresses and points of interest as a user types. The Autocomplete API can match on full words as well as substrings. Applications can therefore send queries as the user types, to provide on-the-fly place predictions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**q** | **String** | Address to geocode | [required] |
**normalizecity** | **i32** | For responses with no city value in the address section, the next available element in this order - city_district, locality, town, borough, municipality, village, hamlet, quarter, neighbourhood - from the address section will be normalized to city. Defaults to 1 for SDKs. | [required] |
**limit** | Option<**i32**> | Limit the number of returned results. Default is 10. |  |[default to 10]
**viewbox** | Option<**String**> | The preferred area to find search results.  To restrict results to those within the viewbox, use along with the bounded option. Tuple of 4 floats. Any two corner points of the box - `max_lon,max_lat,min_lon,min_lat` or `min_lon,min_lat,max_lon,max_lat` - are accepted in any order as long as they span a real box.  |  |
**bounded** | Option<**i32**> | Restrict the results to only items contained with the viewbox |  |
**countrycodes** | Option<**String**> | Limit search to a list of countries. |  |
**accept_language** | Option<**String**> | Preferred language order for showing search results, overrides the value specified in the Accept-Language HTTP header. Defaults to en. To use native language for the response when available, use accept-language=native |  |
**tag** | Option<**String**> | Restricts the autocomplete search results to elements of specific OSM class and type.  Example - To restrict results to only class place and type city: tag=place:city, To restrict the results to all of OSM class place: tag=place |  |

### Return type

[**Vec<serde_json::Value>**](serde_json::Value.md)

### Authorization

[key](../README.md#key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

