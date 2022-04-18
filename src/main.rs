mod login;
mod utils;
mod configuration;
#[macro_use] extern crate serde;
use serde::{Serialize, Deserialize};
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate open;

use login::*;
use serde::de::DeserializeOwned;
use configuration::fetch::get_configuration;
use std::env;

fn main() {

    //standard_login::standard_login_flow();
    let conf: AppConfiguration = get_configuration().unwrap();
    println!("conf is {}", conf.redirect_url);
}

#[derive(Deserialize)]
pub struct AppConfiguration {

    pub redirect_url: String
}

fn handle_args(args: Vec<String>){

    if args.len() == 0 {
        println!("No arguments detected.");
        display_help();
        return;
    }

    match args[1].as_str() {
        "login" => {
            standard_login::standard_login_flow();
        },
        _ => {
            println!("Found no function for argument");
            display_help();
        }
    }




}

fn display_help(){

    println!("");
    println!("Possible commands are: ");
    println!("TODO lisää uudet komennot tänne.");
}