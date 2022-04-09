use std::collections::HashMap;
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

#[cfg(test)]
mod SerializerTests {

    use super::*;
    #[test]
    pub fn test_valid_json() {

        let mut serializer = Serializer::new();
        serializer.add(String::from("asd"), String::from("lol"));
        serializer.add(String::from("kek"), String::from("resp"));

        let result_string = serializer.get_json().unwrap();
        assert!(result_string.contains("asd"));
        assert!(result_string.contains("lol"));
        assert!(result_string.contains("kek"));
        assert!(result_string.contains("resp"));
    }
}