use open::that;
use std::net::
{ TcpListener, TcpStream };
use crate::configuration::fetch::get_configuration;
use crate::configuration::configuration_structs::AppConfiguration;
use std::io::Read;
use std::str;

pub fn standard_login_flow(){

    let configuration: AppConfiguration;

    // Get configuration
    match get_configuration::<AppConfiguration>() {
        Ok(elem) => {
            configuration = elem;
        }
        Err(msg) => {
            println!("Error getting configuration for login: {}", msg);
            return;
        }
    }

    // create tcp listener for capturing the redirect from twitter login

    open_login_page(&configuration.callback_url);
}

fn handle_tcp_response(stream: TcpStream){


}

fn create_tcp_listener(port: &str){

    /*

    Creates a tcplistener to a specified port to capture redirectionUrl:s

     */

     let listener = TcpListener::bind(port).unwrap();

     for stream in listener.incoming() {
         let stream = stream.unwrap();

         handle_stream(stream);
     }
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

fn open_login_page(callback: &str) {

    let uri = format!("https://api.twitter.com/oauth/request_token?oauth_callback={}", callback);
    open::that(uri);
}