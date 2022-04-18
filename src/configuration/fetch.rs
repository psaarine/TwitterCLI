use std::env::current_dir;
use std::fs::read_dir;
use serde::de::DeserializeOwned;
use serde_json::from_reader;
use std::fs::File;

pub fn get_configuration<T>() -> Result<T, &'static str> 
    where T : DeserializeOwned
{

    let configuration_json = read_dir(current_dir()
        .unwrap())
        .unwrap()
        .find(|x| { 
            let res = x.as_ref().unwrap().file_name().into_string().unwrap();
            return res == "configuration.json";
            
        });
    match configuration_json {
        Some(val) => {

            let name = val.unwrap();
            let file = File::open(name.path()).unwrap();
            let res: Result<T, serde_json::Error> = serde_json::from_reader(file);
            match res {
                Ok(conf) => {
                    return Ok(conf);
                }
                Err(_) => {
                    return Err("Could not serialize configuration into class");
                }
            }
        }
        None => {
            return Err("Could not find configuration file");
        }
    }
}