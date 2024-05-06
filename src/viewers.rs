use crate::models::PasswordsFile;

pub fn show_usernames(passwords_data: &PasswordsFile, application: String) {
    for app in &passwords_data.passwords {
        if app.application == application {

            let mut usernames_vec: Vec<_> = app.credentials.iter().map(|c| {
                c.username.clone().unwrap_or("None".to_string())
            }).collect();

            usernames_vec.sort_unstable_by(|a, b| a.cmp(b));
            usernames_vec.iter().for_each(|s| println!("{}", s));

            return;
        }
    }

    panic!("No application was provided")
}

pub fn show_apps(passwords_data: &PasswordsFile) {
    let mut apps: Vec<&String> = Vec::new();
    for app in &passwords_data.passwords {
        apps.push(&app.application);
    }

    apps.sort_unstable_by(|a, b| a.cmp(b));
    apps.iter().for_each(|s| println!("{}", s));
}

