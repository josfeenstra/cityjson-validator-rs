# cityjson-validator-rs 

A cli tool which validates a cityjson file according to a schema, and certain other properties


# properties the validator tests againts
- [X] json schema
- [X] Duplicate vertices
- [ ] Duplicate names (hashes)
- [ ] Proper building hierarchies
- [ ] ...

# Install 

```
git clone https://github.com/josfeenstra/cityjson-validator-rs
cd cityjson-validator-rs
cargo build
```


# Local usage 
```
target/debug/validator.exe [absolute path to schema] [absolute path to cityjson file] 
```

# Web usage

Download [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/).

Then run

```
wasm-pack build -t web
```
for a local `/pkg`, or run 
```
wasm-pack build -t web --out-dir ../cityjson-validator/dist/bin/
```

Then run it like we do [here](https://github.com/josfeenstra/cityjson-validator).


Credits
-------

- Written as a GEO5010 research project for the Msc Geomatics @ Tu Delft 


