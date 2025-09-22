use anyhow::Result;
use std::{collections::btree_map::BTreeMap, fs::File, path::Path};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Labels {
    pub names: BTreeMap<i16, String>,
}

pub fn get_labels<P: AsRef<Path>>(path: P) -> Result<Labels> {
    let file = File::open(path)?;
    let config: Labels = serde_yaml::from_reader(file)?;
    println!("{:?}", config);
    Ok(config)
}

#[test]
fn test_labels() {
    let path = "Pylearn/data/coco.yaml";
    let res = get_labels(path);
    assert!(res.is_ok());
    let success = res.unwrap();
    let person = success.names.get(&0).unwrap();
    assert!(person == "person")
}
