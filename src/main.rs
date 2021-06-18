use cjval;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    schema_path: std::path::PathBuf,
    #[structopt(parse(from_os_str))]
    json_path: std::path::PathBuf,
}

fn main() {
    let args = Cli::from_args();

    let json = std::fs::read_to_string(&args.json_path).expect("couldn't read json");
    let schema = std::fs::read_to_string(&args.schema_path).expect("coudn't read schema");

    let res = cjval::CityJsonValidator::new_from_string(schema.as_str());
    let validator = match res {
        Ok(val) => val,
        Err(_) => return,
    };

    validator.validate_from_str(json.as_str());
    return;
}
