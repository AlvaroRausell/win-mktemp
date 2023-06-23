use std::path::Path;

use clap::Parser;
use mktemp::cli::mktemp_api::{mktemp_dir, mktemp_file};
use mktemp::service_configuration::parse_config::{parse_config_data, Properties};
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

    println!("{}", new_dir);
}
