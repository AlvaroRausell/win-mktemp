use std::{path::Path, fs::File, fs::read_to_string,  io::Write, thread::yield_now};
use serde_derive::{Deserialize, Serialize};
use chrono::{DateTime, Utc, Duration};

#[derive(Deserialize, Serialize, Debug, Clone)]
struct FileProperties {
    path: String,
    date_created: String,
    delete_by: String
}
type Files =  Vec<FileProperties>;
#[derive(Deserialize, Serialize, Debug)]
pub struct Records {
    files: Files,
    path: String,
    file_lifespan: u64
}
impl Drop for Records {
    fn drop(&mut self) {
        self.save();
    }
}
impl Records {
    fn save(&self) {
        std::fs::write(self.path.as_str(), serde_json::to_string_pretty(self).unwrap()).unwrap();
    }
    fn millis_to_date(from: DateTime<Utc>, to: DateTime<Utc>)-> i64 {
        (to-from).num_milliseconds()
    }
    fn delete_file_from_records(&mut self, path: String) {
        self.files = self.files.clone().into_iter().filter(|file| file.path != path).collect();
    }
    // pub fn delete_
    fn get_next_file_to_delete(&self) -> Option<FileProperties>{
        self.files.clone().into_iter().min_by_key(|file|-> DateTime<Utc> {DateTime::parse_from_rfc2822(file.delete_by.as_str()).unwrap().into()}) 
    }
    pub fn get_next_deletion_date(&self) -> Option<DateTime<Utc>> {
        match self.get_next_file_to_delete() {
            None => None,
            Some(file) => Some(DateTime::parse_from_rfc2822(file.delete_by.as_str()).unwrap().into())

        }
    }
    pub fn pop_file(& mut self) {
        match self.get_next_file_to_delete(){
            None => (),
            Some(file) => {
                self.delete_file_from_records((&file.path).to_string());
                if Path::new(&(file.path).to_string().as_str()).is_dir() {
                    std::fs::remove_dir_all(file.path).unwrap();
                } else {
                    std::fs::remove_file(file.path).unwrap();
                }
            }

        }
    }
    fn get_millis_to_next_deletion_date(&self)-> Option<i64> {
        match self.get_next_deletion_date(){
            None => None,
            Some(file) => Some(Self::millis_to_date(Utc::now(), file))

        }
    }
    pub fn record_file_creation(&mut self, path: String) {
        let now: DateTime<Utc> =  Utc::now();
        let day_to_delete = now + Duration::milliseconds(self.file_lifespan as i64);
        self.files.push(FileProperties { path: path.to_string(), date_created: Utc::now().to_rfc2822(), delete_by: day_to_delete.to_string() })
    }
    pub fn new (records_file_path: String, lifespan: u64) -> Self {
        match Self::check_or_create_records_file(&records_file_path) {
            Ok(()) => Records {
                files: Files::new(),
                path: records_file_path,
                file_lifespan: lifespan
            },
            Err(()) => panic!("Error creating records file")
        }

    }
    fn check_or_create_records_file(records_file_path: &String)-> Result<(),()> {
        if records_file_path == "" {
            return Err(());
        }
        let path = Path::new(records_file_path.as_str());
        if path.exists() {
            return Ok(());
        }
        match File::create(path) {
            Ok(_) => Ok(()),
            Err(e) => {
                println!("{}", e);
                Err(())
            }
        }
    }
}


mod tests {
    use std::path::PathBuf;

    use serial_test::serial;

    use super::*;
    fn test_records_file(test_file: &str) -> String {
        let mut test_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_dir.push("test");
        test_dir.push("records");
        test_dir.push(test_file);
        String::from(test_dir.to_str().unwrap())
    }

    #[test]
    #[serial] // Needed as we cannot be messing with env vars in parallel!
    fn test_creates_file_on_new() {
        Records::new(test_records_file("records.json"), 10);
        assert!(Path::new(test_records_file("records.json").as_str()).exists())
    }

    #[test]
    #[serial] // Needed as we cannot be messing with env vars in parallel!
    fn test_updates_file_records() {
        let mut records = Records::new(test_records_file("records.json"), 1);
        records.record_file_creation(String::from("Hai"));
        assert!(records.files.len() == 1 && records.files[0].path == String::from("Hai"))
    }
    #[test]
    #[serial]
    fn test_saves_file_on_exit() {
        let path = &test_records_file("records_saves.json");
        let mut records = Records::new(path.to_string(), 1);
        records.record_file_creation(String::from("Hai"));
        drop(records);
        let contents = read_to_string(path).unwrap();
        assert_ne!(contents.len(), 0)
    }
}

