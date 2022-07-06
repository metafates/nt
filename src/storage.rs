use std::error::Error;
use std::io::{Read, Write};
use std::path::PathBuf;
use dirs::config_dir;
use json::object;


const CONFIG_FILE_NAME: &str = "records.json";

/// set new record
pub fn set_record(name: &str, value: &str) -> Result<(), Box<dyn Error>> {
    // check if file exists
    let storage_path = get_storage_path().ok_or("Could not get storage path")?;
    let storage_file = storage_path.join(CONFIG_FILE_NAME);

    if !storage_file.exists() {
        // create file
        std::fs::create_dir_all(&storage_file.parent().unwrap())?;

        let mut record = object! {};
        record.insert(name, value)?;


        std::fs::File::create(&storage_file)?
            .write(record.dump().as_bytes())?;
    } else {
        // update file
        let mut file = std::fs::File::open(&storage_file)?;
        let mut contents = String::new();

        file.read_to_string(&mut contents)?;

        let mut records = json::parse(&contents).unwrap();

        records.insert(name, value)?;

        std::fs::File::create(storage_file)?
            .write(records.dump().as_bytes())?;
    }

    Ok(())
}

/// Get record by name.
pub fn get_record(name: &str) -> Result<(String, Option<String>), Box<dyn Error>> {
   // check if file exists
    let storage_path = get_storage_path().ok_or("Could not get storage path")?;
    let storage_file = storage_path.join(CONFIG_FILE_NAME);

    if !storage_file.exists() {
        return Ok((String::new(), None));
    }

    let mut file = std::fs::File::open(&storage_file)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    let records = json::parse(&contents).unwrap();

    if records.has_key(name) {
        let value = records[name].to_string();
        Ok((name.to_string(), Some(value)))
    } else {
        Ok((String::new(), None))
    }
}

pub fn remove_record(name: &str) -> Result<(), Box<dyn Error>> {
    // check if file exists
    let storage_path = get_storage_path().ok_or("Could not get storage path")?;
    let storage_file = storage_path.join(CONFIG_FILE_NAME);

    if !storage_file.exists() {
        return Ok(());
    }

    // read file
    let mut file = std::fs::File::open(&storage_file)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    // parse file
    let mut records = json::parse(&contents).unwrap();

    // remove record
    records.remove(name);

    // write file
    std::fs::File::create(storage_file)?
        .write(records.dump().as_bytes())?;

    Ok(())
}

/// Get all records
pub fn get_records() -> Result<Vec<(String, String)>, Box<dyn Error>> {
    let storage_path = get_storage_path().ok_or("Could not get storage path")?;
    let storage_file = storage_path.join(CONFIG_FILE_NAME);

    if !storage_file.exists() {
        return Ok(Vec::new());
    }

    let mut file = std::fs::File::open(&storage_file)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    let records = json::parse(&contents).unwrap();

    let mut keys: Vec<(String, String)> = Vec::new();
    for key in records.entries() {
        keys.push((key.0.to_string(), key.1.to_string()));
    }

    Ok(keys)
}

/// Gets the path to the storage directory
fn get_storage_path() -> Option<PathBuf> {
    let config_path = config_dir()?;
    Some(config_path.join("nt"))
}

