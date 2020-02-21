# \DirectionsApi

All URIs are relative to *https://eu1.locationiq.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**directions**](DirectionsApi.md#directions) | **Get** /directions/driving/{coordinates} | Directions Service



## directions

> crate::models::DirectionsDirections directions(coordinates, bearings, radiuses, generate_hints, approaches, exclude, alternatives, steps, annotations, geometries, overview, continue_straight)
Directions Service

Finds the fastest route between coordinates in the supplied order.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**coordinates** | **String** | String of format {longitude},{latitude};{longitude},{latitude}[;{longitude},{latitude} ...] or polyline({polyline}) or polyline6({polyline6}). polyline follows Google's polyline format with precision 5 | [required] |
**bearings** | Option<**String**> | Limits the search to segments with given bearing in degrees towards true north in clockwise direction. List of positive integer pairs separated by semi-colon and bearings array should be equal to length of coordinate array. Input Value :- {bearing};{bearing}[;{bearing} ...] Bearing follows the following format : bearing {value},{range} integer 0 .. 360,integer 0 .. 180 |  |
**radiuses** | Option<**String**> | Limits the search to given radius in meters Radiuses array length should be same as coordinates array, eaach value separated by semi-colon. Input Value - {radius};{radius}[;{radius} ...] Radius has following format :- double >= 0 or unlimited (default) |  |
**generate_hints** | Option<**String**> | Adds a Hint to the response which can be used in subsequent requests, see hints parameter. Input Value - true (default), false Format - Base64 String |  |
**approaches** | Option<**String**> | Keep waypoints on curb side. Input Value - {approach};{approach}[;{approach} ...] Format - curb or unrestricted (default) |  |
**exclude** | Option<**String**> | Additive list of classes to avoid, order does not matter. input Value - {class}[,{class}] Format - A class name determined by the profile or none. |  |
**alternatives** | Option<**f32**> | Search for alternative routes. Passing a number alternatives=n searches for up to n alternative routes. [ true, false (default), or Number ] |  |
**steps** | Option<**String**> | Returned route steps for each route leg [ true, false (default) ] |  |
**annotations** | Option<**String**> | Returns additional metadata for each coordinate along the route geometry.  [ true, false (default), nodes, distance, duration, datasources, weight, speed ] |  |[default to "false"]
**geometries** | Option<**String**> | Returned route geometry format (influences overview and per step) [ polyline (default), polyline6, geojson ] |  |[default to "polyline"]
**overview** | Option<**String**> | Add overview geometry either full, simplified according to highest zoom level it could be display on, or not at all. [ simplified (default), full, false ] |  |[default to "simplified"]
**continue_straight** | Option<**String**> | Forces the route to keep going straight at waypoints constraining uturns there even if it would be faster. Default value depends on the profile [ default (default), true, false ] |  |[default to "default"]

### Return type

[**crate::models::DirectionsDirections**](directions-directions.md)

### Authorization

[key](../README.md#key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

