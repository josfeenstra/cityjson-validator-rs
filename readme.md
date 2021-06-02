# cityjson-validator-rust

A cli tool which validates a cityjson file according to a schema, and certain other properties



# properties the validator tests against

- [X] The cityjson must match the 1.0.2 [cityjson-schema](https://www.cityjson.org/specs/overview/) 
- [X] Vertices cannot be duplicate 
- [X] Proper building hierarchies
   - [X] 1st level city objects cannot have parents -> schema should take care of this 
      - [X] if they have children, they must exist
   - [X] 2nd level city objects must have at least 1 parent -> schema should take care of this
      - [X] this parent must exist
   - [X] parent-child relationships must be mutual (parent pointing to child, child pointing to parent)
- [ ] no duplicate keys in general
   - [ ] names (hashes) cannot be duplicate



# Install 

```
git clone https://github.com/josfeenstra/cj-val-rs
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
wasm-pack build -t web --out-dir ../cityjson-validator/docs/bin/
```

Then run it like we do [here](https://github.com/josfeenstra/cj-val).


Credits
-------

- Written as a GEO5010 research project for the Msc Geomatics @ Tu Delft 


