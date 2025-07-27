use crate::config::DadagenConfig;
use anyhow::{Context, Result};
use csv::{Error, ReaderBuilder};
use lazy_static::lazy_static;
use rand::thread_rng;
use rand::Rng;
use serde_yaml;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::hash::Hash;
use std::io::BufReader;
use std::io::Read;
use std::path::PathBuf;
use std::sync::RwLock;
use xdg::BaseDirectories;

// Define the ListManager struct
pub struct ListManager {
    // Use RwLock for thread safety
    list_data: RwLock<HashMap<String, Vec<String>>>,
    index: RwLock<HashMap<IndexKey, Vec<usize>>>,
}

// Define the global static instance of ListManager
lazy_static! {
    pub static ref GLOBAL_LIST_MANAGER: ListManager = ListManager::new();
}

impl ListManager {
    // Create a new instance of ListManager
    pub fn new() -> Self {
        Self {
            list_data: RwLock::new(HashMap::new()),
            index: RwLock::new(HashMap::new()),
        }
    }

    // Accessor method to retrieve list_data
    pub fn list_data(&self) -> std::sync::RwLockReadGuard<HashMap<String, Vec<String>>> {
        self.list_data
            .read()
            .expect("Failed to acquire read lock for list_data")
    }

    /// list data either has a discriminator or not
    /// The discriminator is used to group the data into sub-lists
    /// For example first-names can be discriminated by gender (M,F)
    pub fn import_data_with_discriminator(
        &self,
        list_name: &str,
        values: Vec<(String,String)>, // rows of Value + Discriminator
    ) {
        if !self.list_data.write().unwrap().contains_key(list_name) {
            self.list_data
                .write()
                .unwrap()
                .insert(list_name.to_string(), Vec::new());
        }

        // now index the data via the discriminator
        for (value, discriminator) in values.iter() {
            let index_key = IndexKey {
                list_name: list_name.to_string(),
                discriminator: discriminator.clone(),
            };

            let i = self.list_data.read().unwrap().get(list_name).unwrap().len();
            self.list_data
                .write()
                .unwrap()
                .get_mut(list_name)
                .unwrap()
                .push(value.clone());

            // Ensure the index has a vector for this key
            if !self.index.read().unwrap().contains_key(&index_key) {
                self.index
                    .write()
                    .unwrap()
                    .insert(index_key.clone(), Vec::new());
            }

            self.index
                .write()
                .unwrap()
                .get_mut(&index_key)
                .unwrap()
                .push(i);
        }
    }

    // Import row data
    pub fn import_data(&self, list_name: &str, rows: Vec<Vec<String>>, has_discriminator: bool) {
        if !self.list_data.write().unwrap().contains_key(list_name) {
            self.list_data
                .write()
                .unwrap()
                .insert(list_name.to_string(), Vec::new());
        }

        if has_discriminator {
            let mut index_key = IndexKey {
                list_name: list_name.to_string(),
                discriminator: ",,^^,,".to_string(),
            };
            for (_i, r) in rows.iter().enumerate() {
                let i = self
                    .list_data
                    .write()
                    .unwrap()
                    .get(list_name)
                    .unwrap()
                    .len();
                self.list_data
                    .write()
                    .unwrap()
                    .get_mut(list_name)
                    .unwrap()
                    .push(r[0].clone());

                let mut new_index_key = index_key.clone();
                if new_index_key.discriminator != r[1] {
                    new_index_key = IndexKey {
                        list_name: list_name.to_string(),
                        discriminator: r[1].clone(),
                    };
                    self.index
                        .write()
                        .unwrap()
                        .insert(new_index_key.clone(), Vec::new());
                }

                self.index
                    .write()
                    .unwrap()
                    .get_mut(&new_index_key)
                    .unwrap()
                    .push(i);

                index_key = new_index_key;
            }
        } else {
            for r in rows {
                self.list_data
                    .write()
                    .unwrap()
                    .get_mut(list_name)
                    .unwrap()
                    .push(r[0].clone());
            }
        }
    }

    // Import data from a CSV file
    pub fn import_file(&self, list_name: &str, file_path: &str) -> Result<(), Error> {
        if !self.list_data.write().unwrap().contains_key(list_name) {
            self.list_data
                .write()
                .unwrap()
                .insert(list_name.to_string(), Vec::new());
        }

        let mut rdr = ReaderBuilder::new()
            .has_headers(false)
            .from_path(file_path)?;
        for result in rdr.records() {
            let record = result?;
            if record.len() > 1 {
                self.import_data(
                    list_name,
                    vec![record.iter().map(|s| s.to_string()).collect()],
                    true,
                );
            } else {
                self.import_data(
                    list_name,
                    vec![record.iter().map(|s| s.to_string()).collect()],
                    false,
                );
            }
        }
        Ok(())
    }

    // Return a random value
    pub fn get_random_value(&self, list_name: &str, discriminator: Option<String>) -> String {
        let mut rng = thread_rng();
        let list_data = self.list_data.read().unwrap();
        let index = self.index.read().unwrap();

        match discriminator {
            Some(discrim) => {
                if let Some(index_key) = index.get(&IndexKey {
                    list_name: list_name.to_string(),
                    discriminator: discrim.clone(),
                }) {
                    let i = rng.gen_range(0..index_key.len());
                    list_data.get(list_name).unwrap()[index_key[i]].clone()
                } else {
                    "".to_string()
                }
            }
            None => {
                if let Some(data) = list_data.get(list_name) {
                    let i = rng.gen_range(0..data.len());
                    data[i].clone()
                } else {
                    "".to_string()
                }
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct IndexKey {
    pub list_name: String,
    pub discriminator: String,
}

pub struct DadagenConfigSupport;

impl DadagenConfigSupport {
    // Determine the config path based on the build mode
    fn config_path() -> Result<PathBuf> {
        let config_file = if cfg!(debug_assertions) {
            // Create the path to the config file
            let mut path =
                env::current_exe().context("Failed to get the current executable's path")?;
            path.pop(); // Remove the binary name from the path
            path.push("config"); // Add "config" folder

            path
        } else {
            let xdg_dirs = BaseDirectories::new().context("XDG not configured ?")?;
            let config_path = xdg_dirs.place_config_file("dadagen.config.yml")?;
            config_path
        };

        Ok(config_file)
    }

    pub fn import_config_list_data(
        list_key_name: &str,
        _file_path: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if GLOBAL_LIST_MANAGER.list_data().contains_key(list_key_name) {
            println!("Not importing {} as it is already imported", list_key_name);
            return Ok(());
        }

        let config_path = DadagenConfigSupport::config_path()?;

        let mut file = File::open(&config_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let list_config: DadagenConfig = serde_yaml::from_str(&contents)?;

        if let Some(list_data) = list_config.lists.get(list_key_name) {
            if let Some(filename) = &list_data.filename {
                // You can handle the file-based import here
                // The `filename` variable contains the path to the file
                println!("Importing list data from file: {}", filename);
            }

            if let Some(values) = &list_data.values {
                let mut tuple_values: Vec<(String, String)> = Vec::new();
                for (discriminator, value_list) in values {
                    for value in value_list {
                        tuple_values.push((value.clone(), discriminator.clone()));
                    }
                }
                GLOBAL_LIST_MANAGER.import_data_with_discriminator(
                    list_key_name,
                    tuple_values,
                );
            }
        }

        Ok(())
    }

    pub fn import_file(list_key_name: &str, file_path: &str) -> Result<(), std::io::Error> {
        if GLOBAL_LIST_MANAGER.list_data().contains_key(list_key_name) {
            println!("Not importing {} as it is already imported", list_key_name);
            return Ok(());
        }

        let file = File::open(file_path)?;
        let mut reader = BufReader::new(file);
        let mut contents = String::new();
        reader.read_to_string(&mut contents)?;

        let rows: Vec<Vec<String>> = contents
            .lines()
            .map(|line| line.split(',').map(String::from).collect())
            .collect();

        GLOBAL_LIST_MANAGER.import_data(list_key_name, rows, true);
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_import_file() {
        // Clear any existing data to ensure test isolation
        GLOBAL_LIST_MANAGER.list_data.write().unwrap().clear();
        
        let file_contents = "foo\nbar\nbaz\n";
        let mut file = NamedTempFile::new().unwrap();
        write!(file, "{}", file_contents).unwrap();
        let file_path = file.path().to_str().unwrap();

        GLOBAL_LIST_MANAGER.import_file("test_list", file_path).unwrap();

        let list_data = GLOBAL_LIST_MANAGER.list_data.read().unwrap();
        assert_eq!(list_data.get("test_list"), Some(&vec!["foo".to_string(), "bar".to_string(), "baz".to_string()]));
    }

    #[test]
    fn test_import_data() {
        // Clear any existing data to ensure test isolation
        GLOBAL_LIST_MANAGER.list_data.write().unwrap().clear();
        
        GLOBAL_LIST_MANAGER.import_data("test_list", vec![vec!["foo".to_string()], vec!["bar".to_string()], vec!["baz".to_string()]], false);

        let list_data = GLOBAL_LIST_MANAGER.list_data.read().unwrap();
        assert_eq!(list_data.get("test_list"), Some(&vec!["foo".to_string(), "bar".to_string(), "baz".to_string()]));
    }

    // #[test]
    // fn test_get_list() {
    //     let list_manager = ListManager::new();
    //     GLOBAL_LIST_MANAGER.import_data("test_list", vec![vec!["foo".to_string()], vec!["bar".to_string()], vec!["baz".to_string()]], true);

    //     let list = GLOBAL_LIST_MANAGER.get_list("test_list").unwrap();

    //     assert_eq!(list, vec!["foo", "bar", "baz"]);
    // }

    // #[test]
    // fn test_get_list_index() {
    //     let list_manager = ListManager::new();
    //     GLOBAL_LIST_MANAGER.import_data("test_list", vec![vec!["foo".to_string()], vec!["bar".to_string()], vec!["baz".to_string()]], true);

    //     let index = GLOBAL_LIST_MANAGER.get_list_index("test_list", "bar").unwrap();

    //     assert_eq!(index, 1);
    // }

    // #[test]
    // fn test_get_list_index_not_found() {
    //     let list_manager = ListManager::new();
    //     GLOBAL_LIST_MANAGER.import_data("test_list", vec![vec!["foo".to_string()], vec!["bar".to_string()], vec!["baz".to_string()]], true);

    //     let index = GLOBAL_LIST_MANAGER.get_list_index("test_list", "qux");

    //     assert_eq!(index, None);
    // }
}