use open::that;
use std::net::
{ TcpListener, TcpStream };

pub fn standard_login_flow(){


    open_login_page("randomcallback");
}

fn handle_tcp_response(stream: TcpStream){


}

fn open_login_page(callback: &str) {

    let uri = format!("https://api.twitter.com/oauth/request_token?oauth_callback={}", callback);
    open::that(uri);
}