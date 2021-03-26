mod config;
mod api_key;
mod utils;

#[macro_use] extern crate rocket;
#[macro_use] extern crate lazy_static;

use std::collections::HashMap;
use std::path::Path;
use std::str::FromStr;

use rocket::{State, Data};
use rocket::data::ByteUnit;
use crate::config::{UploadConfig, UploadState};
use crate::api_key::User;
use crate::utils::generate_filename;
use std::sync::atomic::{AtomicU64, Ordering};
use rocket::tokio::sync::RwLock;
use rocket_contrib::serve::StaticFiles;
use tokio::fs::File;

#[post("/upload/<ext>", data = "<data>")]
async fn upload(user: User, ext: String, data: Data, state: State<'_, UploadState>) -> String {
    let lower = state.lower.load(Ordering::SeqCst);
    let upper = state.upper.load(Ordering::SeqCst);
    let out_dir = state.output.read().await;
    let out_path = Path::new(out_dir.as_str());

    let file = generate_filename(lower, upper, &ext, out_path, 0);

    let filename = user.descriptor
        .clone()
        .replace("{name}", file.as_str())
        .replace("{ext}", ext.as_str());

    println!("{} by {}", filename, user.name);

    let filepath = Path::new(out_dir.as_str()).join(filename.as_str());
    let file = File::create(filepath.clone()).await;

    match file {
        Ok(file) => {

            data
                .open(ByteUnit::Byte(user.upload_limit))
                .stream_to(file)
                .await
                .unwrap();

            filename
        }
        Err(e) => {
            e.to_string()
        }
    }
}

lazy_static! {
    pub static ref USERS: RwLock<HashMap<String, User>> = RwLock::new(HashMap::new());
}

#[rocket::main]
async fn main() {
    let config_string = std::fs::read_to_string(Path::new("Settings.toml")).expect("Failed to read config");
    let config: UploadConfig = toml::from_str(config_string.as_str()).expect("Failed to parse config");
    let limit: u64;

    let out = Path::new(&config.settings.output);

    if !out.exists() {
        std::fs::create_dir_all(out).expect("Failed to create output directory");
    }

    if let Some(upload_limit) = config.settings.upload_limit {
        limit = ByteUnit::from_str(upload_limit.as_str()).expect("Failed to parse Global Upload Limit").as_u64();
    } else {
        limit = ByteUnit::from_str("200 MiB").unwrap().as_u64();
    }

    let lower = u64::from_str_radix(config.settings.min_id.unwrap_or("aaaaaa".to_string()).as_str(), 36).expect("Failed to parse min_id");
    let upper = u64::from_str_radix(config.settings.max_id.unwrap_or("zzzzzzz".to_string()).as_str(), 36).expect("Failed to parse max_id");

    assert!(lower < upper, "Lower id bound has to be smaller than upper id bound");

    let mut users = USERS.write().await;

    for (name, user) in config.user {
        let mut user_limit = limit;
        if let Some(upload_limit) = user.upload_limit {
            user_limit = ByteUnit::from_str(upload_limit.as_str()).expect("Failed to parse user-specific upload limit").as_u64();
        }

        users.insert(user.token, User {
            name,
            descriptor: user.descriptor.unwrap_or("{name}.{ext}".to_string()),
            upload_limit: user_limit,
        });
    }

    drop(users);

    let state = UploadState {
        output: RwLock::new(config.settings.output.clone()),
        lower: AtomicU64::new(lower),
        upper: AtomicU64::new(upper),
    };

    rocket::ignite()
        .mount("/", routes![upload])
        .mount("/", StaticFiles::from(out))
        .manage(state)
        .launch()
        .await
        .expect("Failed to start webserver");
}
