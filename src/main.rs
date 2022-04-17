mod login;
mod utils;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate open;

use login::*;

fn main() {

    standard_login::standard_login_flow();
}
