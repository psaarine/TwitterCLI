use crate::utils::serializer::Serializer;
use serde_json::from_str;
use serde::Deserialize;

pub fn parse_query<'de, T>(data: &str ) -> Result<T, &'static str> 
    where T : Deserialize<'de>
{

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

    let mut serializer = Serializer::new();
    for param_couple in query_strings.split("&") {

        let resp = parse_param_couple(param_couple);
        match resp {
            Ok(tuple) => {

                serializer.add(tuple.0, tuple.1);
            }
            Err(e) => {

                panic!("parsing query failed! {}", e);
            }
        }

    }

    let string_data  = serializer.get_json().unwrap();
    //let serialization_result = serde_json::from_str::<T>(&string_data);

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

#[cfg(test)]
mod parser_tests {

    use super::parse_param_couple;

    #[test]
    pub fn test_parse_param_couple(){

        let test_string = "mikko=pekka";
        let test_string_2 = "jari=matti";
        let test_string_3 = "key=value";

        let (test_key_1, test_key_2) = parse_param_couple(test_string).unwrap();
        assert!(test_key_1 == "mikko");
    }
}

#[cfg(test)]
mod parseQueryTests {

    /*

    Tests function parse query and its subfunction parse_param_couple

    */

    use super::*;

    #[test]
    pub fn test_parse_query(){

        // These are some real url:s I found

        let dev_art = "https://www.deviantart.com/search?q=tree";
        let twitter = "https://twitter.com/search?q=Russian&src=trend_click&vertical=trends";
        let youtube = "https://www.youtube.com/watch?v=3dcli9i_pvA";

        let dev_art_data = parse_query::<DeviantArtParams>(dev_art).unwrap();
        assert!(dev_art_data.q == "tree");
    }

    #[derive(Serialize, Deserialize)]
    struct DeviantArtParams {
        pub q: String
    }

    struct TwitterParams {
        q: String,
        src: String,
        vertical: String
    }
}