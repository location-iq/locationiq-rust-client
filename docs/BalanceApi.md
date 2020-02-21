# \BalanceApi

All URIs are relative to *https://eu1.locationiq.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**balance**](BalanceApi.md#balance) | **Get** /balance.php | 



## balance

> crate::models::Balance balance()


The Balance API provides a count of request credits left in the user's account for the day. Balance is reset at midnight UTC everyday (00:00 UTC).

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Balance**](balance.md)

### Authorization

[key](../README.md#key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

