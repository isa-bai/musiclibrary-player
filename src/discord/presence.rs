use std::time::Duration;

use discord_rich_presence::{activity::{Activity, ActivityType, Assets, Timestamps}, DiscordIpc, DiscordIpcClient};
use reqwest::Client;
use serde_json::Value;

#[derive(Debug)]
pub enum DiscordMessage {
    //artist, song title, current pos, song duration
    SetPresence((PresenceData, u64)),
    ClearPresence
}

#[derive(Debug)]
pub struct PresenceData {
    pub artist: String,
    pub song_title: String,
    pub album_title: String,
    pub current_pos: u64,
    pub song_duration: u64
}


pub struct DiscordClient {
    client: DiscordIpcClient,
    //connected: bool
}

impl DiscordClient {
    pub fn new() -> Self {
        Self {
            client: DiscordIpcClient::new(include_str!("./CLIENT_ID")).unwrap()
        }
    }

    pub fn _connect(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.client.connect()
    }

    pub fn set_presence(&mut self, data: &PresenceData, img: &str, mut start: u64) -> Result<(), Box<dyn std::error::Error>> {
        start = start - data.current_pos;
        println!("{}", img);
        self.client.set_activity(Activity::new()
            .state(&data.artist) //artist
            .details(&data.song_title) //song title
            .timestamps(Timestamps::new()
                .start(start as i64)
                .end((start + data.song_duration) as i64))
            .activity_type(ActivityType::Listening)
            .assets(Assets::new()
                .large_text(&data.album_title)
                .large_image(img))
        )
        
    }

    pub fn shutdown(&mut self) {
        _ = self.client.close();
    } 

    pub fn clear_presence(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.client.clear_activity()
    }
    
    pub fn block_until_connected(&mut self) {
        while let Err(_) = self.client.connect() {
            println!("failed to connect");
            std::thread::sleep(Duration::from_secs(5));
        }
    }
}

impl Default for DiscordClient {
    fn default() -> Self {
        DiscordClient::new()
    }
}

pub async fn get_cover_art(album: &str, artist: &str) -> Result<String, reqwest::Error> {

    let client = Client::builder()
    .user_agent("musiclibrary-player").build()?;

    let url = format!("https://musicbrainz.org/ws/2/release/?query=artist:{artist}%20AND%20release:{album}&fmt=json");
    let start = std::time::Instant::now();
    let response = client.get(url).send().await?;

    let response_json: Value = response.json().await?;

    if let Some(releases) = response_json.get("releases").and_then(Value::as_array) {
        for release in releases {
            if let Some(id) = release.get("id").and_then(Value::as_str) {
                println!("{}", start.elapsed().as_millis());
                //return Ok(format!("https://coverartarchive.org/release/{id}/front"));
                let res = client.head(format!("https://coverartarchive.org/release/{id}/front")).send().await;
                if res.is_ok() {
                    if res.as_ref().unwrap().status().is_success() {
                        return Ok(format!("{}", res.unwrap().url()));
                    }
                }
            }
        }
    }
    

    return Ok(String::new());
}