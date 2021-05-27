// lib.rs
// AUTHOR:  Jos Feenstra
// PURPOSE: The core logic of validating a city json. 
// NOTE:    for now, Initialize a `Validator` using both the full schema & full instance.
//          in the future, initialize `Validator` using only the schema, which is then able to validate mulitple different cityjson's, potentially. 
//          - I did not make that initialy, because converting a string to a serde json object is expensive, so it makes sense to do that once during initialization 

extern crate serde_json;
extern crate jsonschema;

use serde_json::{Value as Json};
use jsonschema::{JSONSchema, paths::JSONPointer};

pub struct CityJsonValidator {
    instance: Json,
    schema: Json,
}

impl CityJsonValidator {
    pub fn from_strings(schema_string: &str, instance_string: &str, ) -> Self {
        
        let schema: Json = serde_json::from_str(schema_string).expect("couldnt convert schema to json");
        let instance: Json = serde_json::from_str(instance_string).expect("couldnt convert instance to json");

        Self::new(instance, schema)
    }

    pub fn validate(&self) -> bool {
        if self.validate_schema_jsonschema().is_err() {
            println!("NOT vaild according to schema!");
            return false;
        } else {
            println!("vaild according to schema");
        }
        if self.validate_property().is_err() {
            println!("NOT vaild according to property!");
            return false;
        } else {
            println!("valid according to property");
        }

        return true;
    }
}

impl CityJsonValidator {

    fn new(instance: Json, schema: Json) -> Self {
        Self {instance, schema}
    }

    // validate json using the 'jsonschema' crate
    fn validate_schema_jsonschema(&self) -> Result<(), &str> {

        let schema = JSONSchema::compile(&self.schema).unwrap();
        let result = schema.validate(&self.instance);

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
                    let kind = &error.kind;
                    
                    // store the error itself 
                    println!(" L Validation error : {}", error);
                }
            }

            return Err("Failure");
        }

        return Ok(());
    }

    // validate some other property
    fn validate_property(&self) -> Result<(), &str> {
        // TODO
        return Ok(());
    }
}