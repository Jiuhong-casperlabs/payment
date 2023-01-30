## README
---
This is payment logic used by put-deploy on casper blockchain.

You can specify payment-path and payment-arg rather than payment-amount for put-deploy.

```
--payment-path "/home/jh/rust/test22/contract/target/wasm32-unknown-unknown/release/contract.wasm" \
--payment-arg "purse_name:String='mypurse'" \
--payment-arg "amount:U512='5000000000'" 
```


Entire command see [note](./note.md)