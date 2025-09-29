# EventParams


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**PriceUp** | [**EventParamsOneOfPriceUp**](EventParamsOneOfPriceUp.md) |  | [default to undefined]
**PriceDown** | [**EventParamsOneOfPriceUp**](EventParamsOneOfPriceUp.md) |  | [default to undefined]
**TokenSell** | [**EventParamsOneOf2TokenSell**](EventParamsOneOf2TokenSell.md) |  | [default to undefined]
**TokenBuy** | [**EventParamsOneOf2TokenSell**](EventParamsOneOf2TokenSell.md) |  | [default to undefined]
**WalletSell** | [**EventParamsOneOf4WalletSell**](EventParamsOneOf4WalletSell.md) |  | [default to undefined]
**WalletBuy** | [**EventParamsOneOf4WalletSell**](EventParamsOneOf4WalletSell.md) |  | [default to undefined]

## Example

```typescript
import { EventParams } from './api';

const instance: EventParams = {
    PriceUp,
    PriceDown,
    TokenSell,
    TokenBuy,
    WalletSell,
    WalletBuy,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
