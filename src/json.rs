use crate::models::{ PasswordsFile, Application, Credential };

use std::fs::{ self, File };
use std::io::{ Write, Error, ErrorKind };

pub fn deserialize_json(path: &str) -> Result<PasswordsFile, Error> {
    let file = fs::read_to_string(path)?;
    let json_data: PasswordsFile = serde_json::from_str(&file)?;
    Ok(json_data)
}

pub fn serialize_json(passwords_data: &PasswordsFile) {
    let json_data = serde_json::to_string_pretty(&passwords_data).unwrap();
    let mut file = File::create("passwords.json").unwrap();
    file.write_all(json_data.as_bytes()).expect("Failed to write to file");
}

pub fn add_password(
    password_data: &mut PasswordsFile,
    application: &str,
    username: Option<String>,
    encrypted_password: Vec<u8>,
    ) -> Result<(), Error> {

    let passwords = &mut password_data.passwords;

    for application_iter in passwords {
        if application_iter.application == application {
            application_iter.credentials.push(Credential {
                username,
                password: encrypted_password,
            });
            return Ok(());
        }
    }

    password_data.passwords.push(
        Application {
            application: application.to_string(),
            credentials: vec![
                Credential {
                    username,
                    password: encrypted_password,
                }
            ]
        }
        );

    Ok(())
}

pub fn find_password(
    password_data: &PasswordsFile,
    application: &str,
    username: Option<String>,
    index: u8
    ) -> Result<Vec<u8>, Error> {

    let passwords = &password_data.passwords;

    for application_iter in passwords {
        if application_iter.application == application {
            let mut count: u8 = 0;

            for credential in &application_iter.credentials {
                if credential.username == username || username.is_none() {
                    if count == index { return Ok(credential.password.clone()) };
                    count += 1;
                }
            }
        }
    }

    Err(Error::new(ErrorKind::Other, "Password not found"))
}

