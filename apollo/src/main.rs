use hermes::{Token, Hermes, Credentials, reqwest::Url, serde_json};
use serde::{Serialize, Deserialize};
use std::error::Error;
use std::env;
use std::path;
use std::fs;
use dialoguer::{theme::ColorfulTheme, Password, Input, Confirm};

#[derive(Serialize, Deserialize, Default, Debug)]
struct Config {
    token: Option<Token>,
    endpoint: String,
}

impl Config {
    pub fn new<S: Into<String>>(s: S) -> Self {
        Self { endpoint: s.into(), ..Default::default() }
    }

    pub fn has_token(&self) -> bool {
        self.token.is_some()
    }

    pub fn set_token<'a, T: Into<&'a Token>>(&mut self, t: T) {
        self.token = Some(t.into().clone())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config_path = env::var("ARTEMIS_CONFIG").unwrap_or_else(|_| ".apollo".to_string());
    let config_path = path::Path::new(&config_path);
    let theme = ColorfulTheme::default();

    let mut config: Config = if !config_path.exists() { Config::new(Input::<String>::with_theme(&theme).with_prompt("api enpoint").interact_text()?) }
    else { serde_json::from_str(&fs::read_to_string(config_path)?)? };

    let mut hermes = Hermes::new(Url::parse(&config.endpoint)?).with_agent(concat!("apollo-cli/", env!("CARGO_PKG_VERSION")));

    if !config.has_token() {
        let username: String = Input::with_theme(&theme).with_prompt("username").interact_text()?;
        let password: String = Password::with_theme(&theme).with_prompt("password").interact()?;
        let remember_me: bool = Confirm::with_theme(&theme).with_prompt("remember me").interact()?;

        hermes.login(&Credentials { username, password, remember_me }).await?;
        config.set_token(&hermes.token);

        println!("Saving {config_path:?}");
        fs::write(config_path, serde_json::to_string_pretty(&config)?)?;
    } else {
        hermes.set_token(config.token.as_ref().unwrap());
    }



    println!("Projects: {}", hermes.get_for_dashboard().await?.len());

    Ok(())
}
