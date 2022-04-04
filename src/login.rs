use std::net::
{ TcpListener, TcpStream };
use std::io::prelude::*;
use std::str;

pub fn login() {

    /*

    Opens a tcp connection that listens to a specific port.
    Then directs user to a twitter login page that redirects to that particular tcp port.
    Parses Id from the identification.

     */

     create_tcp_listener();
     //open_browser(&generate_login_url());

}

fn open_browser(url: &str) {

    /*

    Opens a browser in a specified url.

     */
}

fn create_tcp_listener(){

    /*

    Creates a tcplistener to a specified port to capture redirectionUrl:s

     */

     let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

     for stream in listener.incoming() {
         let stream = stream.unwrap();

         handle_stream(stream);
     }
}

fn generate_login_url() -> String {

    return String::from("asd");
}

fn handle_stream(mut stream: TcpStream) {

    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let id = parse_received_url(str::from_utf8(&buffer).unwrap().to_owned());
    println!("{}", id);
}

fn parse_received_url(data: String) -> String {

    let mut parts = data.split_whitespace();
    
    let part1 = parts.next().unwrap();
    let part2 = parts.next().unwrap();
    let part3 = parts.next().unwrap();

    return format!("Part 1: {}, part 2: {}, part 3: {}", part1, part2, part3);
}

fn parse_twitter_return_url(url: &str) {

    let query_part = url.split("?").last().unwrap_or("Query string was maliformed");
    
}

struct AuthResult {

    oauth_token: String,
    oauth_verifier: String

}

#[cfg(test)]
mod tests {
    
    use super::*;
    #[test]
    pub fn test_return_string_parser() {

        // tests that method can parse valid information from return string
    }
}
