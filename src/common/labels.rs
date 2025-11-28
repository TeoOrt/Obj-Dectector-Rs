use anyhow::Result;
use std::{collections::btree_map::BTreeMap, fs::File, path::Path};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Labels {
    pub names: BTreeMap<i16, String>,
}

pub fn get_labels<P: AsRef<Path>>(path: P) -> Result<Labels> {
    let file = File::open(path)?;
    let config: Labels = serde_yaml::from_reader(file)?;
    println!("{:?}", config);
    Ok(config)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_labels() {
        let path = "test.yaml";

        let mut example_labels = BTreeMap::new();
        example_labels.entry(0).or_insert(String::from("person"));
        let label = Labels {
            names: example_labels,
        };
        // Create or open the file for writing
        let file = File::create(path).expect("Failed to create config file");

        // Serialize the Config struct to the file in YAML format
        serde_yaml::to_writer(file, &label).expect("Failed to write out to file");
        let res = get_labels(path);
        assert!(res.is_ok());
        let success = res.unwrap();
        let person = success.names.get(&0).unwrap();
        assert!(person == "person")
    }
}
