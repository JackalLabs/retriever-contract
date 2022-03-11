# RNS Name Service (RNS)

## Deploying the contract

Assuming you have a recent version of rust and cargo (v1.55.0+) installed
(via [rustup](https://rustup.rs/)),

### Storing
```
junod tx wasm store ibc_name_service.wasm  --from test --chain-id=uni-2 --gas auto
```

### Instantiating
```
junod tx wasm instantiate 472 '{"blocks_per_year": 5048093, "meta_url": "https://nameserviceimage.jackaldao.com", "denom": "ujunox"}' --amount 50000ujunox --label "JACKAL Name Service" --from test --chain-id uni-2 --gas 1000000 --gas-prices 0.075ujunox -y
```

### Queries
#### Contract State
```
junod query wasm contract-state smart juno1qr76sfjnr40xulzlymm2ehx5wxwv8pyqnlah5smqtnfrq446guvs42xc6l '{"contract_info": {}}'
```
#### Token Info
```
junod query wasm contract-state smart juno1qr76sfjnr40xulzlymm2ehx5wxwv8pyqnlah5smqtnfrq446guvs42xc6l '{"nft_info": {"token_id": "jackal"}}'
```
#### Resolves only the JUNO address
```
junod query wasm contract-state smart juno1qr76sfjnr40xulzlymm2ehx5wxwv8pyqnlah5smqtnfrq446guvs42xc6l '{"resolve_name": {"name": "jackal"}}'
```
#### Resolves entire metadata
```
junod query wasm contract-state smart juno1qr76sfjnr40xulzlymm2ehx5wxwv8pyqnlah5smqtnfrq446guvs42xc6l '{"resolve_attributes": {"name": "jackal"}}'
```

### Executions
#### Register Name
```
junod tx wasm execute juno1qr76sfjnr40xulzlymm2ehx5wxwv8pyqnlah5smqtnfrq446guvs42xc6l'{"register_name": {"name": "jackal", "years": 2}}' --from test --chain-id uni-2 --gas 1000000 --gas-prices 0.075ujunox --amount 312500ujunox
```
