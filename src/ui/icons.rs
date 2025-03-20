use iced::widget::svg::{self, Handle};
use lazy_static::lazy_static;
lazy_static! {
    pub static ref PLAY_ICON: Handle = Handle::from_memory(include_bytes!("../../assets/play.svg").to_vec());
    pub static ref PAUSE_ICON: Handle = Handle::from_memory(include_bytes!("../../assets/paused.svg").to_vec());
    pub static ref STOP_ICON: Handle = Handle::from_memory(include_bytes!("../../assets/stop.svg").to_vec());
    pub static ref FORWARD_ICON: Handle = Handle::from_memory(include_bytes!("../../assets/forward.svg").to_vec());
    pub static ref BACK_ICON: Handle = Handle::from_memory(include_bytes!("../../assets/back.svg").to_vec());
    pub static ref LOOP_ICON: Handle = Handle::from_memory(include_bytes!("../../assets/loop.svg").to_vec());
    pub static ref LOOP1_ICON: Handle = Handle::from_memory(include_bytes!("../../assets/loop1.svg").to_vec());
    pub static ref SHUFFLE_ICON: Handle = Handle::from_memory(include_bytes!("../../assets/shuffle.svg").to_vec());
    pub static ref NOTE_ICON: Handle = Handle::from_memory(include_bytes!("../../assets/note.svg").to_vec());
    pub static ref ALBUM_ICON: Handle = Handle::from_memory(include_bytes!("../../assets/album.svg").to_vec());
    pub static ref ARTIST_ICON: Handle = Handle::from_memory(include_bytes!("../../assets/artist.svg").to_vec());
}

pub enum Icon {
    Play,
    Pause,
    Stop,
    Forward,
    Back,
    Loop,
    Loop1,
    Shuffle,
    Note,
    Album,
    Artist
}

impl Icon {
    pub fn icon_data(&self) -> svg::Handle {
        match self {
            Icon::Play => PLAY_ICON.clone(),
            Icon::Pause => PAUSE_ICON.clone(),
            Icon::Stop => STOP_ICON.clone(),
            Icon::Forward => FORWARD_ICON.clone(),
            Icon::Back => BACK_ICON.clone(),
            Icon::Loop => LOOP_ICON.clone(),
            Icon::Loop1 => LOOP1_ICON.clone(),
            Icon::Shuffle => SHUFFLE_ICON.clone(),
            Icon::Note => NOTE_ICON.clone(),
            Icon::Album => ALBUM_ICON.clone(),
            Icon::Artist => ARTIST_ICON.clone(),
        }
    }
}