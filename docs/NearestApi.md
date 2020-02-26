# LocationIq\NearestApi

All URIs are relative to *https://eu1.locationiq.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**nearest**](NearestApi.md#nearest) | **Get** /nearest/driving/{coordinates} | Nearest Service



## nearest

> crate::models::DirectionsNearest nearest(coordinates, generate_hints, exclude, bearings, radiuses, approaches, number)
Nearest Service

Snaps a coordinate to the street network and returns the nearest n matches. Where coordinates only supports a single {longitude},{latitude} entry.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**coordinates** | **String** | String of format {longitude},{latitude};{longitude},{latitude}[;{longitude},{latitude} ...] or polyline({polyline}) or polyline6({polyline6}). polyline follows Google's polyline format with precision 5 | [required] |
**generate_hints** | Option<**String**> | Adds a Hint to the response which can be used in subsequent requests, see hints parameter. Input Value - true (default), false Format - Base64 String |  |
**exclude** | Option<**String**> | Additive list of classes to avoid, order does not matter. input Value - {class}[,{class}] Format - A class name determined by the profile or none. |  |
**bearings** | Option<**String**> | Limits the search to segments with given bearing in degrees towards true north in clockwise direction. List of positive integer pairs separated by semi-colon and bearings array should be equal to length of coordinate array. Input Value :- {bearing};{bearing}[;{bearing} ...] Bearing follows the following format : bearing {value},{range} integer 0 .. 360,integer 0 .. 180 |  |
**radiuses** | Option<**String**> | Limits the search to given radius in meters Radiuses array length should be same as coordinates array, eaach value separated by semi-colon. Input Value - {radius};{radius}[;{radius} ...] Radius has following format :- double >= 0 or unlimited (default) |  |
**approaches** | Option<**String**> | Keep waypoints on curb side. Input Value - {approach};{approach}[;{approach} ...] Format - curb or unrestricted (default) |  |
**number** | Option<**i32**> | Number of nearest segments that should be returned. [ integer >= 1 (default 1) ] |  |

### Return type

[**crate::models::DirectionsNearest**](directions-nearest.md)

### Authorization

[key](../README.md#key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

