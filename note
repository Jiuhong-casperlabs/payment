===use mypurse rather than main purse for payment=====
casper-client put-deploy --chain-name casper-test \
--node-address http://94.130.10.55:7777 \
--secret-key "/home/jh/keys/test1/secret_key.pem" \
--session-path "/home/jh/rust/test22/contract/target/wasm32-unknown-unknown/release/contract1.wasm" \
--payment-path "/home/jh/rust/test22/contract/target/wasm32-unknown-unknown/release/contract.wasm" \
--payment-arg "purse_name:String='mypurse'" \
--payment-arg "amount:U512='5000000000'" 


casper-client get-deploy --node-address http://94.130.10.55:7777 bba1201a5182e5b955b2c25a79fe3aaaa8f8c1dc2208390a556f1a407c89258c

====transfer CSPR from main purse to mypurse====
casper-client put-deploy --chain-name casper-test \
--node-address http://94.130.10.55:7777 \
--secret-key "/home/jh/keys/test1/secret_key.pem" \
--session-path "/home/jh/rust/test22/contract/target/wasm32-unknown-unknown/release/contract2.wasm" \
--session-arg "amount:U512='10000000000'" \
--payment-amount 5000000000