use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PasswordsFile {
    pub passwords: Vec<Application>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Application {
    pub application: String,
    pub credentials: Vec<Credential>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Credential {
    pub username: Option<String>,
    pub password: Vec<u8>,
}
