use std::path::Path;

use clap::Parser;
use win_mktemp::cli::mktemp_api::{mktemp_dir, mktemp_file};
use win_mktemp::service_configuration::parse_config::{parse_config_data, Properties, lifespan_to_millis};
use win_mktemp::service_configuration::records::Records;
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct CliArgs {
    #[arg(short, default_value_t = false)]
    dir: bool,
}
fn main() {
    let args = CliArgs::parse();
    let properties = parse_config_data();
    let new_dir = match args.dir {
        true => mktemp_dir(Path::new(properties.tmp_path.as_str())),
        false => mktemp_file(Path::new(properties.tmp_path.as_str())),
    };
    let properties = parse_config_data();
    Records::new(properties.records_file_path, lifespan_to_millis(properties.lifespan_amount, properties.lifespan_unit)).record_file_creation((&new_dir).to_string());


    println!("{}", new_dir);
}
