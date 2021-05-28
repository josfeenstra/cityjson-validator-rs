# cityjson-validator-rs 

A pure rust implementation of a (city) json validator.

# build 
```
cargo build
```
or
```
wasm-pack build -t web
```
or 
```
wasm-pack build -t web --out-dir ../cityjson-validator/dist/bin/
```

# run 
```
validator.exe [absolute path to schema] [absolute path to cityjson file] 
```
