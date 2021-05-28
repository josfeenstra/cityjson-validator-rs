// lib.rs
// AUTHOR:  Jos Feenstra
// PURPOSE: The core logic of validating a city json. 
// NOTE:    for now, Initialize a `Validator` using both the full schema & full instance.
// TODO: in the future, initialize `Validator` using only the schema, which is then able to validate mulitple different cityjson's, potentially. 
//          - I did not make that initialy, because converting a string to a serde json object is expensive, so it makes sense to do that once during initialization 
//          

extern crate serde_json;
extern crate jsonschema;


use wasm_bindgen::prelude::*; // TODO : #[optional]

use serde_json::{Value as Json};
use jsonschema::{JSONSchema, paths::JSONPointer};
use std::collections::HashMap;

// TODO : #[optional wasm_bindgen]
#[wasm_bindgen] // TODO optional
pub struct CityJsonValidator {
    schema: Json,
}

// wasm public 
#[wasm_bindgen] // TODO optional
impl CityJsonValidator {
    pub fn validate_from_str(&self, instance_string: &str) -> bool {
        let json = &CityJsonValidator::str_to_json(instance_string);
        return self.validate(json);
    }
}

// public
impl CityJsonValidator {

    pub fn new(schema: Json) -> Self {
        Self {schema}
    }

    pub fn from_strings(schema_string: &str) -> Self {
        println!("converting jsons...");
        let schema = CityJsonValidator::str_to_json(schema_string);
        return Self::new(schema);
    }

    pub fn validate(&self, instance: &Json) -> bool {
        println!("validating...");
        // first, check the schema, and immediately abort if the json instance fails to comply
        if self.validate_schema(&instance).is_err() {
            println!("[BAD] schema not valid!");
            return false;
        } else {
            println!("[GOOD] schema valid");
        }

        // validate more advanced properties
        if !self.validate_no_duplicate_vertices(&instance) {
            println!("[BAD] duplicate vertices!");
            return false;
        } else {
            println!("[GOOD] no duplicate vertices");
        }

        // TODO : add more validators here!
        self.validate_some_other_property(&instance);

        // done
        return true;
    }

    // helper function to create a serde json
    pub fn str_to_json(json_string: &str) -> Json {
        // TODO this is already quite fast, but ideally, you would do something with a read buffer
        // no qlueue how to do that in rust yet
        return serde_json::from_str(json_string).expect("couldnt convert string to json");
    }
}

// private 
impl CityJsonValidator {

    // validate json schema using the 'jsonschema' crate
    fn validate_schema(&self, instance: &Json) -> Result<(), &str> {

        let schema = JSONSchema::compile(&self.schema).unwrap();
        let result = schema.validate(instance);

        if result.is_err() {

            // I dont get this construction...
            if let Err(errors) = result {
                for error in errors {

                    println!("Schema Error ");

                    // store the json location where this occured
                    let ptr: JSONPointer = error.instance_path.clone();
                    print!(" L Location:        : ");
                    let vec = ptr.into_vec();
                    print!("Root"); 
                    for part in vec.iter() {   
                        print!(" -> {}", part);
                    }
                    println!("");

                    // store the type of error. This is a nice rust enum, we could do some nice things with that, 
                    // like more user-friendly error messages & suggestions on how to fix them
                    let _kind = &error.kind;

                    // store the error itself 
                    println!(" L Validation error : {}", error);
                }
            }
            return Err("Failure");
        }
        return Ok(());
    }

    // validate some other property
    fn validate_no_duplicate_vertices(&self, instance: &Json) -> bool {
        let mut valid = true;
        let verts = instance
            .get("vertices")
            .expect("no vertices")
            .as_array()
            .expect("not an array");

        // use all vertices as keys in a hashmap
        let mut uniques = HashMap::new();
        for i in 0..verts.len() {
            let vert = verts[i].as_array().unwrap();
            
            let arr = [
                vert[0].as_i64().unwrap(),
                vert[1].as_i64().unwrap(),
                vert[2].as_i64().unwrap(),
            ];
            
            if !uniques.contains_key(&arr) {
                uniques.insert(arr, i);
            } else {
                // duplicate found!
                let other = uniques.get(&arr).unwrap();
                valid = false;

                // feedback
                println!("Duplicate Vertex Error ");
                println!("  L indices : vertices[{}] == vertices[{}]", other,  i);
                println!("  L vertex  : [{}, {}, {}]", arr[0], arr[1], arr[2]);
            }
        }
        return valid;
    }

    fn validate_some_other_property(&self, instance: &Json) -> bool {
        return true;
    }
}

// optional / not needed
impl CityJsonValidator {

    // a 'cheat sheet', to figure out how to iterate jsons within rust. 
    // Be prepared to unwrap :)
    fn _print_some_verts(&self, instance: &Json) {

        let t = instance
            .get("type")
            .expect("no type")
            .as_str()
            .unwrap();

        println!("type: {}", t);

        let verts = instance
            .get("vertices")
            .expect("no vertices")
            .as_array()
            .expect("not an array");
            
        println!("length of verts: {}", verts.len());
        
        for i in 0..10 {
            let vert = verts[i].as_array().unwrap();
            print!("vert: ");
            for j in 0..3 {
                let val = vert[j].as_i64().unwrap();
                print!("{} ", val);
            }
            println!("");
        }
    }
}