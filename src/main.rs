mod models; use models::{ PasswordsFile, Application, Credential };
mod encryption; use encryption::{ encrypt, decrypt };
mod json; use json::{ serialize_json, deserialize_json, add_password, find_password };
mod viewers; use viewers::{ show_apps, show_usernames };

use std::env;
use clap::{command, Arg, ArgMatches};

fn main() {
    let matches = command!()
        .arg(
            Arg::new("encrypt")
            .short('e')
            .takes_value(false)
            )
        .arg(
            Arg::new("decrypt")
            .short('d')
            .takes_value(false)
            )
        .arg(
            Arg::new("viewer")
            .short('v')
            .takes_value(false)
            )
        .arg(
            Arg::new("delete")
            .short('D')
            .takes_value(false)
            )
        .arg(
            Arg::new("application")
            .short('a')
            .long("app")
            .takes_value(true)
            )
        .arg(
            Arg::new("username")
            .short('u')
            .long("username")
            .takes_value(true)
            )
        .arg(
            Arg::new("password")
            .short('p')
            .long("pasword")
            .takes_value(true)
            )
        .arg(
            Arg::new("index")
            .short('i')
            .long("index")
            .takes_value(true)
            )
        .arg(
            Arg::new("option")
            )
        .get_matches();

    let mut passwords_data: PasswordsFile = deserialize_json("passwords.json").unwrap();

    if matches.is_present("encrypt") {
        let (application, username) = get_args(&matches);
        let password: String = matches.get_one::<String>("password").ok_or("No password provided").unwrap().to_string();

        let encrypted_password: Vec<u8> = encrypt(&password);
        add_password(&mut passwords_data, &application, username, encrypted_password).unwrap();
        serialize_json(&passwords_data);
        println!("encrypted the password");

    } else if matches.is_present("decrypt") {
        let (application, username) = get_args(&matches);
        let index: u8 = match matches.get_one::<String>("index") {
            Some(index) => index.parse::<u8>().unwrap_or(0),
            None => 0
        };

        let encrypted_password: Vec<u8> = find_password(&passwords_data, &application, username, index).unwrap();
        let decrypted_password: String = decrypt(encrypted_password);
        println!("Decrypted password: {}", decrypted_password);

    } else if matches.is_present("viewer") {
        let view_option: String = matches.get_one::<String>("option").unwrap_or(&"apps".to_string()).to_string();
        match view_option.as_str() {
            "apps" => {
                show_apps(&passwords_data);
            },
            "usernames" => {
                let application: String = matches.get_one::<String>("application").ok_or("No application provided").unwrap().to_string();
                show_usernames(&passwords_data, application);
            }
            _ => panic!("Unsupported view option")
        }
    } else if matches.is_present("delete") {
        let (application, username) = get_args(&matches);
        let index: u8 = match matches.get_one::<String>("index") {
            Some(index) => index.parse::<u8>().unwrap_or(0),
            None => 0
        };

        delete_password(&mut passwords_data, &application, &username, index);
    }
}

fn delete_password(
    passwords_data: &mut PasswordsFile,
    application: &String,
    username: &Option<String>,
    index: u8,
    ) {

    for i in 0..passwords_data.passwords.len() {
        let application_iter: Application = passwords_data.passwords[i].clone();

        if &application_iter.application == application {

            for j in 0..application_iter.credentials.len() {
                let credential: Credential = application_iter.credentials[j].clone();

                if &credential.username == username {
                    passwords_data.passwords[i].credentials.remove(j);

                    if application_iter.credentials.len() == 0 {
                        passwords_data.passwords.swap_remove(i);
                    }

                    return;
                }
            }

        }
    }
}

    fn get_args(matches: &ArgMatches) -> (String, Option<String>) {
        let username: Option<String> = matches.get_one::<String>("username").cloned();

        let application: String = match matches.get_one::<String>("application") {
            Some(application) => application.to_string(),
            None => panic!("No application was provided")
        };

        (application, username)
    }

