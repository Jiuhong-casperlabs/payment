## prepare =>  the user has to store the purse to be paying under a contract

```
casper-client put-deploy --chain-name casper-test \
--node-address http://94.130.10.55:7777 \
--secret-key "/home/jh/keys/test2/secret_key.pem" \
--session-path "/home/jh/mywork/payment/contract/target/wasm32-unknown-unknown/release/prepare.wasm" \
--session-arg "amount:U512='10000000000'" \
--payment-amount 15000000000
```

casper-client get-deploy --node-address http://94.130.10.55:7777 edb5ad6cbb66338fa713dafa222740d7bdf11896ce1650f80b499ca2f6c32d44

## get purse stored under contract to pay for the current transaction

```
casper-client put-deploy --chain-name casper-test \
--node-address http://94.130.10.55:7777 \
--secret-key "/home/jh/keys/test1/secret_key.pem" \
--session-path "/home/jh/mywork/payment/contract/target/wasm32-unknown-unknown/release/contract1.wasm" \
--payment-path "/home/jh/mywork/payment/contract/target/wasm32-unknown-unknown/release/contract.wasm" \
--payment-arg "pursecontract:string='contract-edb3c80c101011cfea2e3556526d9982ad20207cd0c762f2648e9d0fd706d43a'" \
--payment-arg "amount:U512='5000000000'" 
```


casper-client get-deploy --node-address http://94.130.10.55:7777 20c1e95c438c11b422689f6eef467189a0197aa8fe3ca9611876d8ff18f3dcbe