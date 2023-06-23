use configparser::ini::Ini;
use std::{env, fs::File, io::Read, path::Path};

#[derive(Debug, PartialEq)]
pub struct Properties {
    pub lifespan_amount: u64,
    pub lifespan_unit: String,
    pub tmp_path: String,
}
impl Default for Properties {
    fn default() -> Self {
        Properties {
            lifespan_amount: 1,
            lifespan_unit: String::from("day"),
            tmp_path: String::from("/tmp"),
        }
    }
}
pub fn parse_config_data() -> Properties {
    let data = get_config_data();
    return match data {
        None => Properties::default(),
        Some(contents) => {
            let mut ini = Ini::new();
            ini.read(contents).unwrap();
            Properties {
                lifespan_amount: ini
                    .getuint("service", "lifespan_amount")
                    .unwrap_or_default()
                    .unwrap_or_default(),
                lifespan_unit: ini
                    .get("service", "lifespan_unit")
                    .unwrap_or_else(|| Properties::default().lifespan_unit),
                tmp_path: ini
                    .get("properties", "tmp_path")
                    .unwrap_or_else(|| Properties::default().tmp_path),
            }
        }
    };
}

fn get_config_data() -> Option<String> {
    // Check config dir
    let mut contents = String::new();

    let cfg_path = env::current_exe()
        .unwrap()
        .join("config")
        .join("config.ini");
    if cfg_path.exists() {
        File::open(cfg_path)
            .unwrap()
            .read_to_string(&mut contents)
            .unwrap();
        return Some(contents);
    }
    // Check env vars
    let config_env_dir = env::var("MKTEMP_CONFIG_DIR");
    return match config_env_dir {
        Ok(path_str) => {
            let path = Path::new(path_str.as_str());
            if path.exists() {
                File::open(path)
                    .unwrap()
                    .read_to_string(&mut contents)
                    .unwrap();
                return Some(contents);
            }
            None
        }
        Err(e) => None,
    };
}

#[cfg(test)]

mod tests {
    use std::path::PathBuf;

    use serial_test::serial;

    use super::*;
    fn test_dir() -> String {
        let mut test_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_dir.push("test");
        test_dir.push("config");
        test_dir.push("config.ini");
        String::from(test_dir.to_str().unwrap())
    }

    #[test]
    #[serial] // Needed as we cannot be messing with env vars in parallel!
    fn test_get_config_should_return_string() {
        env::set_var("MKTEMP_CONFIG_DIR", test_dir());
        assert!(get_config_data().is_some() && get_config_data() != Some(String::from("")))
    }

    #[test]
    #[serial] // Needed as we cannot be messing with env vars in parallel!
    fn test_get_config_should_return_none() {
        env::remove_var("MKTEMP_CONFIG_DIR");
        assert_eq!(get_config_data(), None)
    }
    #[test]
    #[serial] // Needed as we cannot be messing with env vars in parallel!
    fn parse_config_data_returns_default() {
        assert_eq!(parse_config_data(), Properties::default())
    }
    #[test]
    #[serial] // Needed as we cannot be messing with env vars in parallel!

    fn parse_config_data_returns_provided() {
        env::set_var("MKTEMP_CONFIG_DIR", test_dir());
        assert_eq!(
            parse_config_data(),
            Properties {
                lifespan_amount: 2,
                lifespan_unit: String::from("min"),
                tmp_path: Properties::default().tmp_path
            }
        )
    }
}
