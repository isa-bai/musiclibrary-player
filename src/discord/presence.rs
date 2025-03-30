use std::time::{Duration, SystemTime, UNIX_EPOCH};

use discord_rich_presence::{activity::{Activity, ActivityType, Assets, Timestamps}, DiscordIpc, DiscordIpcClient};

#[derive(Debug)]
pub enum DiscordMessage {
    //artist, song title, current pos, song duration
    SetPresence(PresenceData),
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

    pub fn connect(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.client.connect()
    }

    pub fn set_presence(&mut self, data: PresenceData) -> Result<(), Box<dyn std::error::Error>> {
        let start = SystemTime::now().duration_since(UNIX_EPOCH.into())
            .unwrap().as_secs() - data.current_pos;
        
        self.client.set_activity(Activity::new()
            .state(&data.artist) //artist
            .details(&data.song_title) //song title
            .timestamps(Timestamps::new()
                .start(start as i64)
                .end((start + data.song_duration) as i64))
            .activity_type(ActivityType::Listening)
            // .assets(Assets::new()
            //     .large_text(&data.album_title)
            //     .large_image("https://crates.io/assets/cargo.png"))
        )
        
    }

    pub fn shutdown(&mut self) {
        self.client.close();
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