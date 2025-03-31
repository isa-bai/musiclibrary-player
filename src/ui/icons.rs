use std::sync::LazyLock;

use iced::widget::svg::{self, Handle};

const PLAY_ICON: LazyLock<Handle> = LazyLock::new(|| {
    Handle::from_memory(include_bytes!("../../assets/play.svg").to_vec())
});
const PAUSE_ICON: LazyLock<Handle> = LazyLock::new(|| {
    Handle::from_memory(include_bytes!("../../assets/paused.svg").to_vec())
});
const STOP_ICON: LazyLock<Handle> = LazyLock::new(|| {
    Handle::from_memory(include_bytes!("../../assets/stop.svg").to_vec())
});
const FORWARD_ICON: LazyLock<Handle> = LazyLock::new(|| {
    Handle::from_memory(include_bytes!("../../assets/forward.svg").to_vec())
});
const BACK_ICON: LazyLock<Handle> = LazyLock::new(|| {
    Handle::from_memory(include_bytes!("../../assets/back.svg").to_vec())
});
const LOOP_ICON: LazyLock<Handle> = LazyLock::new(|| {
    Handle::from_memory(include_bytes!("../../assets/loop.svg").to_vec())
});
const LOOP1_ICON: LazyLock<Handle> = LazyLock::new(|| {
    Handle::from_memory(include_bytes!("../../assets/loop1.svg").to_vec())
});
const SHUFFLE_ICON: LazyLock<Handle> = LazyLock::new(|| {
    Handle::from_memory(include_bytes!("../../assets/shuffle.svg").to_vec())
});
const NOTE_ICON: LazyLock<Handle> = LazyLock::new(|| {
    Handle::from_memory(include_bytes!("../../assets/note.svg").to_vec())
});
const ALBUM_ICON: LazyLock<Handle> = LazyLock::new(|| {
    Handle::from_memory(include_bytes!("../../assets/album.svg").to_vec())
});
const ARTIST_ICON: LazyLock<Handle> = LazyLock::new(|| {
    Handle::from_memory(include_bytes!("../../assets/artist.svg").to_vec())
});
const MINIMISE_ICON: LazyLock<Handle> = LazyLock::new(|| {
    Handle::from_memory(include_bytes!("../../assets/minimise.svg").to_vec())
});
const MAXIMISE_ICON: LazyLock<Handle> = LazyLock::new(|| {
    Handle::from_memory(include_bytes!("../../assets/maximise.svg").to_vec())
});
const UNMAXIMISE_ICON: LazyLock<Handle> = LazyLock::new(|| {
    Handle::from_memory(include_bytes!("../../assets/unmaximise.svg").to_vec())
});
const CLOSE_ICON: LazyLock<Handle> = LazyLock::new(|| {
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