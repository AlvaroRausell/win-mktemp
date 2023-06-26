
use std::{thread, time::Duration, sync::{Arc, atomic::{AtomicBool, Ordering}}};

use chrono::Utc;

use crate::service_configuration::records::{Records};

use super::parse_config::{parse_config_data, lifespan_to_millis};
pub struct Service {
    records: Records,
}

impl Service{
    pub fn run(&mut self) { 
        let terminating = Arc::new(AtomicBool::new(false));
        let t = terminating.clone();
        ctrlc::set_handler( move || {
            
            println!("Received SIGINT event... Terminating");
            t.store(true, Ordering::SeqCst);
        });
        loop {
            if terminating.load(Ordering::SeqCst) {
                break
            }
            match self.records.get_next_deletion_date() {
                None => (),
                Some(date) => {
                    if Utc::now() >= date{
                        self.records.pop_file();
                        println!("File deleted")
                    }
                }
            }
           
            println!("Awaiting 5 seconds");
            thread::sleep(Duration::from_secs(5))
        }
        drop(&self.records)
    }
    pub fn new(records_file_path: String) -> Self {
        let properties = parse_config_data();
        let records = Records::new(records_file_path, lifespan_to_millis(properties.lifespan_amount, properties.lifespan_unit));
        Service { records: records}
    }
}