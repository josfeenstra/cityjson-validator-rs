// lib.rs
// AUTHOR:  Jos Feenstra
// PURPOSE: The core logic of validating a city json. 
// NOTE:    for now, Initialize a `Validator` using both the full schema & full instance.
// TODO: in the future, initialize `Validator` using only the schema, which is then able to validate mulitple different cityjson's, potentially. 
//          - I did not make that initialy, because converting a string to a serde json object is expensive, so it makes sense to do that once during initialization 
//  
// TODO     add some logging mechanism, to not only print the errors in the console, but also give those error messages to a potential web environment...        

extern crate serde_json;
extern crate jsonschema;
extern crate web_sys;

use wasm_bindgen::prelude::*; // TODO : #[optional]

use serde_json::{Value as Json};
use jsonschema::{JSONSchema, paths::JSONPointer};
use std::{collections::HashMap, str::FromStr};


// logging to the javascript / web console
macro_rules! log {
    ( $( $t:tt )* ) => {    
       web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

/// print & log.
macro_rules! plog {
    ( $( $t:tt )* ) => {
        if cfg!(target_arch = "wasm32") {
            log!($( $t )*);
        } else {
            println!($( $t )* );
        }
    }
}


// TODO : #[optional wasm_bindgen]
#[wasm_bindgen] // TODO optional
pub struct CityJsonValidator {
    schema: Json,
}

// wasm public 
#[wasm_bindgen] // TODO optional
impl CityJsonValidator {

    pub fn new_from_string(schema_string: &str) -> Self {
        println!("converting jsons...");
        let schema = CityJsonValidator::str_to_json(schema_string);
        return Self::new(schema);
    }

    pub fn validate_from_str(&self, instance_string: &str) -> bool {
        let json = &CityJsonValidator::str_to_json(instance_string);
        return self.validate(json);
    }

    
    /// TODO: this would spit out the gathered errors in an ideal world. 
    ///
    /// [JF]: Haven't done it yet, since rust's borrow checker doesnt like it if I gather errors during non-mutable function calls. 
    pub fn get_errors() -> String {
        let string = String::from_str("henkiepenkie").unwrap();
        return string;
    }
}

// public
impl CityJsonValidator {

    pub fn new(schema: Json) -> Self {
        // let errors: Vec<String> = Vec::new();
        Self {schema}
    }

    pub fn validate(&self, instance: &Json) -> bool {
        
        plog!("validating...");
        
        // first, check the schema, and immediately abort if the json instance fails to comply
        if self.validate_schema(&instance).is_err() {
            plog!("\n[BAD] schema not valid!");
            return false;
        } else {
            plog!("\n[GOOD] schema valid");
        }

        if !self.validate_no_duplicate_vertices(&instance) {
            plog!("\n[BAD] duplicate vertices!");
            return false;
        } else {
            plog!("\n[GOOD] no duplicate vertices");
        }

        if !self.validate_parent_child(&instance) {
            plog!("\n[BAD] errors in hierarchy!");
            return false;
        } else {
            plog!("\n[GOOD] perfect hierarchy");
        }

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
    fn validate_schema(&self, instance: &Json) -> Result<(), ()> {

        let schema = JSONSchema::compile(&self.schema).unwrap();
        let result = schema.validate(instance);

        if result.is_err() {

            // I dont get this construction...
            if let Err(errors) = result {
                for error in errors {

                    plog!("");
                    plog!("\nSchema Error");

                    // store the json location where this occured
                    let strings: Vec<String> = Vec::new();
                    let ptr: JSONPointer = error.instance_path.clone();
                    plog!(" L Location:        : ");
                    let vec = ptr.into_vec();
                    plog!("Root"); 
                    for part in vec.iter() {   
                        plog!(" -> {}", part);
                    }
                    // plog!("");

                    

                    // store the type of error. This is a nice rust enum, we could do some nice things with that, 
                    // like more user-friendly error messages & suggestions on how to fix them
                    let _kind = &error.kind;

                    // store the error itself 
                    plog!(" L Validation error : {}", error);
                }
            }
            return Err(());
        }
        return Ok(());
    }


    fn validate_no_duplicate_vertices(&self, instance: &Json) -> bool {
        let mut valid = true;
        let verts = instance
            .get("vertices").expect("no vertices")
            .as_array().expect("not an array");

        // use all vertices as keys in a hashmap
        let mut uniques = HashMap::new();
        for i in 0..verts.len() {
            let vert = verts[i].as_array().unwrap();
          
            let arr = [
                vert[0].to_string(),
                vert[1].to_string(),
                vert[2].to_string()
            ];
            
            if !uniques.contains_key(&arr) {
                uniques.insert(arr, i);
            } else {
                // duplicate found!
                let other = uniques.get(&arr).unwrap();
                valid = false;

                // feedback
                plog!("");
                plog!("\nDuplicate Vertex Error");
                plog!("  L indices : vertices[{}] == vertices[{}]", other,  i);
                plog!("  L vertex  : [{}, {}, {}]", arr[0], arr[1], arr[2]);
            }
        }
        return valid;
    }

    // fn concat_vecs(vecs: Vec<String>) -> String {
    //     let vals = vecs.iter().map(|x| &x[..]);
    //     let result = vals.collect();
    // }

    fn validate_parent_child(&self, instance: &Json) -> bool {
        
        // NOTE: I use unwrap a lot, assuming that the instance has been schema-checked before this step...
        // TODO: is there a way to schema-parse the Json after validation? so that all the unwrapping is not needed anymore? 
        //       seen some nice stuff here : https://docs.serde.rs/serde_json/

        let mut valid = true;
        let city_objects = instance
            .get("CityObjects").unwrap()
            .as_object().unwrap();
        
        for key in city_objects.keys() {
            let object = city_objects
                .get(key).unwrap()
                .as_object().unwrap();
            
            // check is parents exist
            if object.contains_key("parents") {
                let parents = object.get("parents").unwrap().as_array().unwrap();
                for p_raw in parents {
                    let p_key = p_raw.as_str().unwrap();
                    
                    if !city_objects.contains_key(p_key) {
                        valid = false;
                        plog!("");
                        plog!("\nInvalid Parent Error");
                        plog!("  L object : CityObjects[{}]", key);
                        plog!("  L its parent ({}) does not exist in CityObjects.", p_key);
                        break;
                    } 
                }
            }

            // check is children exist 
            if object.contains_key("children") {
                let children = object.get("children").unwrap().as_array().unwrap();
                for c in children {
                    let c_key = c.as_str().unwrap();
                    
                    if !city_objects.contains_key(c_key) {
                        valid = false;
                        plog!("");
                        plog!("\nInvalid Child Error");
                        plog!("  L object : CityObjects[{}]", key);
                        plog!("  L its child ({}) does not exist in CityObjects.", c_key);
                        break;
                    } 
                }
            }
        }

        // quit if parent or child errors 
        if !valid {
            return valid;
        }

        // now that we know all parents & children are valid,
        //  we can do the many-to-many doublly linked list check.
        // (otherwise, we would have to double check if children / parents exist...)
        for key in city_objects.keys() {
            let object = city_objects
                .get(key).unwrap()
                .as_object().unwrap();

            if object.contains_key("parents") {
                let parents = object.get("parents").unwrap().as_array().unwrap();
                for c in parents {
                    let p_key = c.as_str().unwrap();
                    
                    let parent = city_objects.get(p_key).unwrap().as_object().unwrap();
                    if !parent.contains_key("children") {
                        valid = false;
                        plog!("");
                        plog!("\nInvalid Parent Logic Error");
                        plog!("  L object : CityObjects[{}]", key);
                        plog!("  L its parent ({}) does not have 'object' as child.", &p_key);
                        plog!("  L it has no childen at all in fact...");
                        break;
                    }
                    
                    // make parents array usable first...
                    let pcs_raw = parent.get("children").unwrap().as_array().unwrap();
                    let mut parent_children = vec![""; pcs_raw.len()];
                    for i in 0..pcs_raw.len() {
                        parent_children[i] = pcs_raw[i].as_str().unwrap();
                    }

                    // now check if it contains
                    if !parent_children.contains(&&key[..]) {
                        valid = false;
                        plog!("");
                        plog!("\nInvalid Parent Logic Error");
                        plog!("  L object : CityObjects[{}]", key);
                        plog!("  L its parent ({}) does not have 'object' as its child.", &p_key);
                        plog!("  L instead it has: {:?}", parent_children); 
                    }
                }
            }

            // check if chilren point back to this object
            if object.contains_key("children") {
                let children = object.get("children").unwrap().as_array().unwrap();
                for c in children {
                    let c_key = c.as_str().unwrap();
                    
                    let child = city_objects.get(c_key).unwrap().as_object().unwrap();
                    if !child.contains_key("parents") {
                        valid = false;
                        plog!("");
                        plog!("\nInvalid Child Logic Error");
                        plog!("  L object : CityObjects[{}]", key);
                        plog!("  L its child ({}) does not have 'object' as parent.", &c_key);
                        plog!("  L it has no parents at all in fact...");
                        break;
                    }
                    
                    // make parents array usable first...
                    let cps_raw = child.get("parents").unwrap().as_array().unwrap();
                    let mut child_parents = vec![""; cps_raw.len()];
                    for i in 0..cps_raw.len() {
                        child_parents[i] = cps_raw[i].as_str().unwrap();
                    }

                    // now check if it contains
                    if !child_parents.contains(&&key[..]) {
                        valid = false;
                        plog!("");
                        plog!("\nInvalid Child Logic Error");
                        plog!("  L object : CityObjects[{}]", key);
                        plog!("  L its child ({}) does not have 'object' as its parent.", &c_key);
                        plog!("  L instead it has: {:?}", child_parents); 
                    }
                }
            }
        } 
        
        return valid;
    }
}

// sketchbook, delete when done
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

    fn _test_syntax(&self, instance: &Json) -> bool {
        
        // aha... this is how you do the less verbose thing...
        let obs = &instance["CityObjects"];

        return true;
    }
}