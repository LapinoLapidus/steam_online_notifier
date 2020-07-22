use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::thread;
use std::time::Duration;

use serde_json::Value;

fn main() {
    dotenv::dotenv().ok();

    // Read .env file into variables.
    let api_key = &std::env::var("STEAM_API_KEY").expect("Couldn't find STEAM_API_KEY.");
    let delay: u64 = std::env::var("DELAY")
        .expect("Couldn't find DELAY.")
        .parse()
        .unwrap();

    // Read steam IDs to check.
    let steam_ids: Vec<String> =
        serde_json::from_reader(BufReader::new(File::open("./steam_ids.json").unwrap())).unwrap();
    let steam_ids = steam_ids.join(",");

    // Create HTTP Client and a personastate cache.
    let mut status_cache: HashMap<String, u8> = HashMap::new();
    let client = reqwest::blocking::Client::new();

    // Main loop.
    loop {
        // Send HTTP request.
        let result = client.get(&format!("https://api.steampowered.com/ISteamUser/GetPlayerSummaries/v2/?key={}&format=json&steamids={}", api_key, steam_ids)).send();
        // A weird error can occur, so instead of unwrapping we handle it.
        if result.is_err() {
            continue;
        }
        let result = result.unwrap();
        let text = result.text().unwrap();
        // Read response to JSON.
        let json: Result<Value, _> = serde_json::from_str(&text);
        if json.is_err() {
            continue;
        }
        let json = json.unwrap();
        // Get list of personas.
        let players: &Value = &json["response"]["players"];
        // Iterate over each persona.
        for player in players.as_array().unwrap() {
            // Get relevant information from the persona object.
            let state = *&player["personastate"].as_u64().unwrap();
            let persona_name = player["personaname"].as_str().unwrap();
            let steam_id = player["steamid"].as_str().unwrap();
            // Check if persona is already in the cache.
            if let Some(old_state) = status_cache.get_mut(steam_id) {
                // Check if user just came online.
                if *old_state == 0 && state >= 1 {
                    // Send the notification.
                    notifica::notify(&format!("{} has come online!", persona_name), "").unwrap();
                }
                // Update the old state with the new state.
                *old_state = state.clone() as u8;
            } else {
                // Add the persona to the cache.
                status_cache.insert(steam_id.to_owned(), state as u8);
            }
        }
        // Suspend the main thread temporarily to not spam the Steam Web API.
        thread::sleep(Duration::from_millis(delay))
    }
}
