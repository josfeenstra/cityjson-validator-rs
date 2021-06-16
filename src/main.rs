use std::{env, fs};
use cityjson_validator;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::from_three(&args).expect("couldn't parse");
    // println!("args: {:?}", config);
    println!("Preparing to test a cityjson...");
    run(config);
    println!("Done, have a nice day!");
}

fn run(config: Config) {
    let json = fs::read_to_string(config.json_path)
        .expect("couldn't read json");
    let schema = fs::read_to_string(config.schema_path)
        .expect("coudn't read schema");

    let res = cityjson_validator::CityJsonValidator::new_from_string(schema.as_str());
    let validator = match res {
        Ok(val) => val,
        Err(_) => return,
    };

    validator.validate_from_str(json.as_str());
    return;
}

/// [JF]: Just something the rust book explains... 
///
/// Personally, I think this is a bit to much for the scale that we are dealing with...
///
/// Still, nice to make to get used to rust error handling
#[derive(Debug)]
struct Config {
    json_path: String,
    schema_path: String,
}

impl Config {

    pub fn from_three(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Incorrect usage. Usage: validator.exe [absolute path to folder] [cityjson file] [schema]");
        }

        let json_path = args[2].clone();
        let schema_path = args[1].clone();
    
        Ok(Self::new(json_path, schema_path))
    }

    pub fn _from_four(args: &[String]) -> Result<Config, &str> {
        if args.len() < 4 {
            return Err("Incorrect usage. Usage: validator.exe [absolute path to folder] [schema] [cityjson file]");
        }
    
        let folder = &args[1];
        let schema_name = &args[2];
        let json_name = &args[3];

        let json_path = [folder.clone(), json_name.clone()].join("");
        let schema_path = [folder.clone(), schema_name.clone()].join("");
    
        Ok(Self::new(json_path, schema_path))
    }

    pub fn new(json_path: String, schema_path: String) -> Config {
        Config {
            schema_path,
            json_path,
        }
    }
}