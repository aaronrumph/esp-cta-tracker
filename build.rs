// [[file:org/cargo.org::*Initialization and setup][Initialization and setup:2]]
use std::{env, fs, path::PathBuf};

fn main() {
    embuild::espidf::sysenv::output();

    // This function loads the 'wifi.env' file in the project root
    load_project_wifi_env();
    // This function loads my API keys from ~/.api_keys.env
    load_home_api_keys_env();
}

/// Loads 'WIFI_SSID' and 'WIFI_PASSWORD' env variables from 'wifi.env'
/// file in the project root directory.
fn load_project_wifi_env() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let wifi_env = manifest_dir.join("wifi.env");

    println!("cargo:rerun-if-changed={}", wifi_env.display());

    load_env_file(&wifi_env, &["WIFI_SSID", "WIFI_PASSWORD"]);
}

/// Loads 'CTA_BUS_API_KEY' and 'CTA_TRAIN_API_KEY' env variables from
/// '~/.api_keys.env'.
fn load_home_api_keys_env() {
    /* NOTE: I choose to keep my API keys in ~/.api_keys.env.
    If you keep them somewhere else you should adjust the
    following code to look in the correct paths */
    let home = env::var("HOME").expect("HOME is not set");
    let api_env = PathBuf::from(home).join(".api_keys.env");

    println!("cargo:rerun-if-changed={}", api_env.display());
    println!("cargo:rerun-if-env-changed=HOME");

    load_env_file(&api_env, &["CTA_BUS_API_KEY", "CTA_TRAIN_API_KEY"]);
}

/// Simple parser for **.env** files that loads any variables
/// into rustc-env variables.
fn load_env_file(path: &PathBuf, allowed_keys: &[&str]) {
    let contents = fs::read_to_string(path).unwrap_or_else(|e| panic!("Couldn't read {}: {e}", path.display()));

    for line in contents.lines() {
        let line = line.trim();

        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let Some((key, value)) = line.split_once('=') else {
            panic!("invalid line in {}: {line}", path.display());
        };

        let key = key.trim();
        let value = value.trim().trim_matches('"').trim_matches('\'');

        if allowed_keys.contains(&key) {
            println!("cargo:rustc-env={key}={value}");
        }
    }
}
// Initialization and setup:2 ends here
