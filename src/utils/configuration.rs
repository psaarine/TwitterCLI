use std::env::current_dir;
use std::fs::read_dir;
use serde::de::DeserializeOwned;
use serde_json::from_reader;
use std::fs::File;

pub fn get_configuration<T>() -> T 
    where T : DeserializeOwned
{

    let configuration_json = read_dir(current_dir()
        .unwrap())
        .unwrap()
        .find(|x| { 
            let res = x.as_ref().unwrap().file_name().into_string().unwrap();
            return res == "configuration.json";
            
        })
        .unwrap();
    
    let name = configuration_json.unwrap();
    let file = File::open(name.path()).unwrap();
    return serde_json::from_reader(file).unwrap();
}