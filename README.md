# README

This is payment logic used by put-deploy on casper blockchain.

You can specify payment-path and payment-arg rather than payment-amount for put-deploy.

```
--payment-path "/home/jh/mywork/payment/contract/target/wasm32-unknown-unknown/release/contract.wasm" \
--payment-arg "pursecontract:string='contract-edb3c80c101011cfea2e3556526d9982ad20207cd0c762f2648e9d0fd706d43a'" \
--payment-arg "amount:U512='5000000000'" 
```

Entire command see [note](./note.md)