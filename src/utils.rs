use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_json::to_string;

#[derive(Serialize, Deserialize)]
pub struct Serializer {

    #[serde(flatten)]
    items: HashMap<String, String>
}

impl Serializer {

    pub fn new() -> Self {
        return Self {
            items: HashMap::new()
        };
    }

    pub fn add(& mut self, key: impl ToString, value: impl ToString) {
        self.items.insert(key.to_string(), value.to_string());
    }

    pub fn get_json(& mut self) -> Result<String, &'static str> {

        match to_string(self){
            Ok(data) => {
                return Ok(data);
            }
            Err(_) => {
                return Err("failed to get json, data was malformet");
            }
        } 
    }
}

#[cfg(Test)]
mod SerializerTests {

    use super::*;
    #[Test]
    pub fn test_valid_json() {

        let json = r#"{"asd":"lol", "kek":"resp"}"#;

        let mut serializer = Serializer::new();
        serializer.add(String::from("asd"), String::from("lol"));
        serializer.add(String::from("kek"), String::from("resp"));
        assert!(json == serializer.get_json().unwrap());
    }
}

pub fn parse_query<T>(data: &str ) -> Result<T, &'static str> {

    // Takes in an url and tries to parse all query parameters into a struct given in generic type

    let query_strings: &str;

    if data.contains("?") {

        // string must have host part included. remove that
        match data.split("?").last() {
            None => {
                return Err("Input did not contain valid query parameters");
            }
            Some(last_part) => {
                query_strings = last_part;
            }
        }

    } else {
        query_strings = data;
    }

    for param_couple in query_strings.split("&") {

    }
    return Err("Todo");
}

fn parse_param_couple(couple: &str) -> Result<(&str, &str), &'static str> {

    // split key-value pair of query string to key and value.

    let mut iterator = couple.split("=");
    let key = iterator.next();
    let return_key: &str;
    let return_value: &str;

    match key {
        Some(value) => {
            return_key = value;
        }
        None => {
            return Err("key was invalid");
        }
    }

    let value = iterator.next();

    match value {
        Some(data) => {
            return_value = data;
        }
        None => {
            return Err("value was invalid");
        }
    }
    return Ok((return_key, return_value));
}

#[cfg(Test)]
mod ParseQueryTests {

    /*

    Tests function parse query and its subfunction parse_param_couple

    */

    #[Test]
    pub fn test_parse_param_couple(){

    }
}