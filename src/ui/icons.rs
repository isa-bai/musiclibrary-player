use std::sync::LazyLock;

use iced::widget::svg::{self, Handle};

pub static PLAY_ICON: LazyLock<Handle> = LazyLock::new(|| {
    Handle::from_memory(include_bytes!("../../assets/play.svg").to_vec())
});
pub static PAUSE_ICON: LazyLock<Handle> = LazyLock::new(|| {
    Handle::from_memory(include_bytes!("../../assets/paused.svg").to_vec())
});
pub static STOP_ICON: LazyLock<Handle> = LazyLock::new(|| {
    Handle::from_memory(include_bytes!("../../assets/stop.svg").to_vec())
});
pub static FORWARD_ICON: LazyLock<Handle> = LazyLock::new(|| {
    Handle::from_memory(include_bytes!("../../assets/forward.svg").to_vec())
});
pub static BACK_ICON: LazyLock<Handle> = LazyLock::new(|| {
    Handle::from_memory(include_bytes!("../../assets/back.svg").to_vec())
});
pub static LOOP_ICON: LazyLock<Handle> = LazyLock::new(|| {
    Handle::from_memory(include_bytes!("../../assets/loop.svg").to_vec())
});
pub static LOOP1_ICON: LazyLock<Handle> = LazyLock::new(|| {
    Handle::from_memory(include_bytes!("../../assets/loop1.svg").to_vec())
});
pub static SHUFFLE_ICON: LazyLock<Handle> = LazyLock::new(|| {
    Handle::from_memory(include_bytes!("../../assets/shuffle.svg").to_vec())
});
pub static NOTE_ICON: LazyLock<Handle> = LazyLock::new(|| {
    Handle::from_memory(include_bytes!("../../assets/note.svg").to_vec())
});
pub static ALBUM_ICON: LazyLock<Handle> = LazyLock::new(|| {
    Handle::from_memory(include_bytes!("../../assets/album.svg").to_vec())
});
pub static ARTIST_ICON: LazyLock<Handle> = LazyLock::new(|| {
    Handle::from_memory(include_bytes!("../../assets/artist.svg").to_vec())
});
pub static MINIMISE_ICON: LazyLock<Handle> = LazyLock::new(|| {
    Handle::from_memory(include_bytes!("../../assets/minimise.svg").to_vec())
});
pub static MAXIMISE_ICON: LazyLock<Handle> = LazyLock::new(|| {
    Handle::from_memory(include_bytes!("../../assets/maximise.svg").to_vec())
});
pub static UNMAXIMISE_ICON: LazyLock<Handle> = LazyLock::new(|| {
    Handle::from_memory(include_bytes!("../../assets/unmaximise.svg").to_vec())
});
pub static CLOSE_ICON: LazyLock<Handle> = LazyLock::new(|| {
    Handle::from_memory(include_bytes!("../../assets/close.svg").to_vec())
});

#[derive(Clone, Copy)]
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
    Artist,
    Minimise,
    Maximise,
    Unmaximise,
    Close
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
            Icon::Minimise => MINIMISE_ICON.clone(),
            Icon::Maximise => MAXIMISE_ICON.clone(),
            Icon::Unmaximise => UNMAXIMISE_ICON.clone(),
            Icon::Close => CLOSE_ICON.clone(),
        }
    }
}