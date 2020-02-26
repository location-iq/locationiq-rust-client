# LocationIq\MatchingApi

All URIs are relative to *https://eu1.locationiq.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**matching**](MatchingApi.md#matching) | **Get** /matching/driving/{coordinates} | Matching Service



## matching

> crate::models::DirectionsMatching matching(coordinates, generate_hints, approaches, exclude, bearings, radiuses, steps, annotations, geometries, overview, timestamps, gaps, tidy, waypoints)
Matching Service

Matching API matches or snaps given GPS points to the road network in the most plausible way.  Please note the request might result multiple sub-traces.  Large jumps in the timestamps (> 60s) or improbable transitions lead to trace splits if a complete matching could not be found. The algorithm might not be able to match all points. Outliers are removed if they can not be matched successfully.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**coordinates** | **String** | String of format {longitude},{latitude};{longitude},{latitude}[;{longitude},{latitude} ...] or polyline({polyline}) or polyline6({polyline6}). polyline follows Google's polyline format with precision 5 | [required] |
**generate_hints** | Option<**String**> | Adds a Hint to the response which can be used in subsequent requests, see hints parameter. Input Value - true (default), false Format - Base64 String |  |
**approaches** | Option<**String**> | Keep waypoints on curb side. Input Value - {approach};{approach}[;{approach} ...] Format - curb or unrestricted (default) |  |
**exclude** | Option<**String**> | Additive list of classes to avoid, order does not matter. input Value - {class}[,{class}] Format - A class name determined by the profile or none. |  |
**bearings** | Option<**String**> | Limits the search to segments with given bearing in degrees towards true north in clockwise direction. List of positive integer pairs separated by semi-colon and bearings array should be equal to length of coordinate array. Input Value :- {bearing};{bearing}[;{bearing} ...] Bearing follows the following format : bearing {value},{range} integer 0 .. 360,integer 0 .. 180 |  |
**radiuses** | Option<**String**> | Limits the search to given radius in meters Radiuses array length should be same as coordinates array, eaach value separated by semi-colon. Input Value - {radius};{radius}[;{radius} ...] Radius has following format :- double >= 0 or unlimited (default) |  |
**steps** | Option<**String**> | Returned route steps for each route leg [ true, false (default) ] |  |
**annotations** | Option<**String**> | Returns additional metadata for each coordinate along the route geometry.  [ true, false (default), nodes, distance, duration, datasources, weight, speed ] |  |[default to "false"]
**geometries** | Option<**String**> | Returned route geometry format (influences overview and per step) [ polyline (default), polyline6, geojson ] |  |[default to "polyline"]
**overview** | Option<**String**> | Add overview geometry either full, simplified according to highest zoom level it could be display on, or not at all. [ simplified (default), full, false ] |  |[default to "simplified"]
**timestamps** | Option<**String**> | Timestamps for the input locations in seconds since UNIX epoch. Timestamps need to be monotonically increasing. [ {timestamp};{timestamp}[;{timestamp} ...]  integer seconds since UNIX epoch |  |
**gaps** | Option<**String**> | Allows the input track splitting based on huge timestamp gaps between points. [ split (default), ignore ] |  |[default to "split"]
**tidy** | Option<**String**> | Allows the input track modification to obtain better matching quality for noisy tracks. [ true, false (default) ] |  |[default to "false"]
**waypoints** | Option<**String**> | Treats input coordinates indicated by given indices as waypoints in returned Match object. Default is to treat all input coordinates as waypoints. [ {index};{index};{index}... ] |  |

### Return type

[**crate::models::DirectionsMatching**](directions-matching.md)

### Authorization

[key](../README.md#key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

