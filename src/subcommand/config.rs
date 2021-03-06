extern crate rpassword;

use std::io::{self, Write};
use crate::config::{Credential, Kindle, Config, ConfigFile};

pub fn config(file: &ConfigFile) -> io::Result<()> {
    let kindle_name_default = "default";

    // TODO: バリデーションを追加
    // TODO: 既に設定ファイルが存在する時は更新の挙動にする
    let user_gmail_address = get_user_input("user_gmail_address: ")?;
    let google_application_password = get_user_input_password("google_application_password: ")?;
    let kindle_name = get_user_input_default(
        format!("kindle_name [{}]: ", kindle_name_default).as_str(),
        "default"
    )?;
    let kindle_mail_address = get_user_input("kindle_mail_address: ")?;

    let kindle = Kindle {
        name: kindle_name,
        mail_address: kindle_mail_address,
        default: true
    };
    let credential = Credential {
        user_gmail_address,
        google_application_password,
    };

    let config = Config::new(credential, vec![kindle]);
    config.save(&file)
}

fn get_user_input_default(prompt: &str, default: &str) -> io::Result<String> {
    let input = get_user_input(prompt)?;

    match input.as_str() {
        "" => Ok(default.to_string()),
        input => Ok(input.to_string())
    }
}

fn get_user_input_password(prompt: &str) -> io::Result<String> {
    let pass = rpassword::prompt_password_stdout(prompt)?;
    Ok(pass.trim().to_string())
}

fn get_user_input(prompt: &str) -> io::Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?;
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => (),
        Err(err) => return Err(err),
    };

    Ok(input.trim().to_string())
}
