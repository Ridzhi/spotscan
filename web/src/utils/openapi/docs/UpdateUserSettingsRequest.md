# UpdateUserSettingsRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**app_theme** | [**AppTheme**](AppTheme.md) |  | [optional] [default to undefined]
**enabled** | **boolean** |  | [optional] [default to undefined]
**slots** | [**{ [key: string]: WindowSettings; }**](WindowSettings.md) |  | [optional] [default to undefined]
**window_default_duration** | **string** |  | [optional] [default to undefined]
**window_default_ends** | **string** |  | [optional] [default to undefined]
**window_default_starts** | **string** |  | [optional] [default to undefined]

## Example

```typescript
import { UpdateUserSettingsRequest } from './api';

const instance: UpdateUserSettingsRequest = {
    app_theme,
    enabled,
    slots,
    window_default_duration,
    window_default_ends,
    window_default_starts,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
