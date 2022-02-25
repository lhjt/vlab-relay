use dialoguer::{theme::ColorfulTheme, Confirm, Input, Password};

#[derive(Debug, Clone)]
pub(crate) struct Configuration {
    pub(crate) host:   String,
    pub(crate) secure: bool,
    pub(crate) token:  String,
}

impl Configuration {
    pub(crate) fn get_url(&self) -> String {
        if self.secure {
            format!("wss://{}", self.host)
        } else {
            format!("ws://{}", self.host)
        }
    }
}

pub(crate) fn get_config() -> Configuration {
    // collect values
    let host = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Please your relay's hostname")
        .default("vlab-relay.example.com".into())
        .interact_text()
        .unwrap();
    let secure = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Use a secure connection (wss://)?")
        .default(true)
        .interact()
        .unwrap();
    let token = Password::with_theme(&ColorfulTheme::default())
        .with_prompt("Please enter your token")
        .interact()
        .unwrap();

    println!();

    Configuration {
        host,
        secure,
        token,
    }
}
