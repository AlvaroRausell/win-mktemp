use win_mktemp::service_configuration::{service::Service, parse_config::parse_config_data};
 fn main() {
    Service::new(parse_config_data().records_file_path).run();
}
    
