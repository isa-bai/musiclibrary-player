use base64::prelude::*;
use std::io::Cursor;
use std::time::{
    SystemTime,
    UNIX_EPOCH
};
use std::{
    collections::VecDeque,
    io::BufReader,
    time::Duration
};
use iced::futures::channel::mpsc::{
    self,
    Sender
};
use iced::futures::SinkExt;
use iced::mouse::Interaction;
use iced::time::{
    self,
    milliseconds
};

use ahash::AHashMap;
use iced::widget::text::LineHeight;

use iced::{
    window::show_system_menu,
    gradient,
    widget::{
        button,
        column,
        container,
        horizontal_space,
        image, row, scrollable,
        slider,
        slider::HandleShape,
        text,
        Button,
        lazy,
        mouse_area,
        svg,
        vertical_space,
        Column
    },
    window::Settings,
    Alignment, Border,
    Center,
    Element,
    Length::{
        self,
        Fill
    },
    Size,
    Theme
};

use ::image::{ImageBuffer, Rgba, RgbaImage};
use rodio::{
    decoder::DecoderBuilder,
    source::EmptyCallback,
    OutputStream,
    Sink
};
use iced::futures::StreamExt;

use iced::futures::Stream;
use iced::{
    mouse,
    stream,
    window,
    Padding,
    Subscription,
    Task
};

use crate::config::PROGRAM_CFG;
use crate::discord::presence::{get_cover_art, DiscordClient, DiscordMessage, PresenceData};
use crate::musiclib::music_library::{self, AlbumKey, MusicLibrary, Song};
use crate::websocket::websocket::{self, SongData, WebsocketMessage};
use super::icons::Icon;
use super::content_views::{library_page, queue_page, settings_page, LibraryView};


const SIDEBAR_SIZE: f32 = 80.;


fn sidebar_button_style(theme: &Theme, status: button::Status) -> button::Style {
    let palette = theme.extended_palette();
    let mut style = button::Style::default();
    style.background = Some(palette.background.base.color.into());
    style.text_color = palette.background.base.text;
    style.border.width = 1.;
    style.border.color = palette.background.strong.color;

    match status {
        button::Status::Hovered => {
            style.background = Some(palette.background.weak.color.into());
            style.text_color = palette.background.weak.text;
            
        }
        _ => {}
    }

    style
}

fn sidebar_style(theme: &Theme) -> container::Style {
    let palette = theme.extended_palette();
    container::Style {
        background: Some(palette.background.base.color.into()),
        border: Border {
            width: 1.0,
            radius: 0.0.into(),
            color: palette.background.strong.color,
            
        },
        ..container::Style::default()
    }
}

fn content_container_style(theme: &Theme) -> container::Style {
    let palette = theme.extended_palette();
    container::Style {
        background: Some(palette.background.base.color.into()),
        border: Border {
            width: 1.0,
            radius: 0.0.into(),
            color: palette.background.strong.color,
            
        },
        ..container::Style::default()
    }
}

fn controls_style(theme: &Theme) -> container::Style {
    let palette = theme.extended_palette();
    let mut style = container::Style::default();
    let gradient = gradient::Linear::new(0)
        .add_stop(0.1, palette.background.base.color)
        .add_stop(0.7, palette.background.weak.color);
    style.background = Some(gradient.into());
    style.border.color = palette.background.strong.color;
    style.border.width = 1.;
    style
}

fn slider_style(theme: &Theme, status: slider::Status) -> slider::Style {
    let palette = theme.extended_palette();
    let mut style = slider::default(theme, status);
    style.handle.shape = HandleShape::Rectangle { width: 6, border_radius: 0.0.into() };
    style.handle.border_width = 1.;
    style.handle.border_color = palette.background.strong.color;
    style.rail.width = 6.;
    style.rail.border.radius = 0.0.into();
    style

}

fn control_button_style(theme: &Theme, status: button::Status) -> button::Style {
    let palette = theme.extended_palette();
    let mut style = button::secondary(theme, status);
    // style.background = Some(palette.background.weak.color.into());
    //style.text_color = palette.background.weak.text;
    style.border.width = 2.;
    style.border.color = palette.secondary.strong.color;
    style.border.radius = 6.0.into();
    // style.border.color = palette.background.strong.color;

    style
}

fn lower_control_button_style(theme: &Theme, status: button::Status, is_on: bool) -> button::Style {
    let palette = theme.extended_palette();
    let mut style;
    if is_on {
        style = button::primary(theme, status);
        style.border.color = palette.primary.strong.color;
    } else {
        style = button::secondary(theme, status);
        style.border.color = palette.secondary.strong.color;
    }
    // style.background = Some(palette.background.weak.color.into());
    //style.text_color = palette.background.weak.text;
    
    style.border.width = 2.;
    //style.border.color = palette.secondary.strong.color;
    style.border.radius = 6.0.into();
    // style.border.color = palette.background.strong.color;

    style
}

fn lower_control_svg_style(theme: &Theme, _status: svg::Status, is_on: bool) -> svg::Style {
    let palette = theme.extended_palette();
    let mut style = svg::Style::default();
    if is_on {
        style.color = Some(palette.primary.base.text.into());
    } else {
        style.color = Some(palette.secondary.base.text.into());
    }
    style
}

fn control_svg_style(theme: &Theme, status: svg::Status) -> svg::Style {
    lower_control_svg_style(theme, status, false)
}

fn sidebar_button(icon: Option<Icon>, txt: &str, msg: ContentView) -> Button<'_, Message> {
    if icon.is_some() {
        let mut items = Column::new();
        items = items.push(
            svg(svg::Handle::from(icon.unwrap().icon_data()))
            .height(SIDEBAR_SIZE/3.)
            .width(SIDEBAR_SIZE/3.)
            .style(titlebar_svg_style))
            .align_x(Center);

        items = items.push(text(txt)
        .size(16)
        .height(Fill)
        .width(SIDEBAR_SIZE)
        .align_x(Center)
        .align_y(Center))
        .align_x(Center);
        button(items.height(SIDEBAR_SIZE).spacing(6))
        .height(Length::Fixed(SIDEBAR_SIZE))
        .width(Length::Fixed(SIDEBAR_SIZE))
        .style(sidebar_button_style)
        .on_press(Message::ContentChanged(msg))
        .padding(Padding {
            top: 14.,
            right: 4.,
            bottom: 14.,
            left: 4.,
        })
        .into()
    } else {
        button(text(txt)
        .align_x(Center)
        .align_y(Center))
        .height(Length::Fixed(SIDEBAR_SIZE))
        .width(Length::Fixed(SIDEBAR_SIZE))
        .style(sidebar_button_style)
        .on_press(Message::ContentChanged(msg))
        .into()
    }

}

fn song_scroll_style(theme: &Theme, status: scrollable::Status) -> scrollable::Style {
    let palette = theme.extended_palette();
    let mut style = scrollable::default(theme, status);
    style.vertical_rail.background = Some(palette.background.weakest.color.into());

    style
}

pub enum ButtonType {
    _Text(String),
    Svg(Icon)
}

fn top_control_button(kind: ButtonType, msg: &ControlMsg) -> Button<'_, Message> {
    match kind {
        ButtonType::_Text(txt) => {
            button(text(txt)
            .align_x(Center)
            .align_y(Center)
            .size(14))
            .height(Length::Fixed(36.))
            .width(Length::Fixed(36.))
            .style(control_button_style)
            .on_press(Message::ControlChange(msg.clone()))
            .into()
        },
        ButtonType::Svg(icon) => {
            button(container(svg(svg::Handle::from(icon.icon_data()))
            .style(control_svg_style)).center(Length::Fixed(36.))
            //.align_x(Center)
            //.align_y(Center)
            ).height(Length::Fixed(36.))
            .width(Length::Fixed(36.))
            .style(control_button_style)
            .on_press(Message::ControlChange(msg.clone()))
            .into()
        }
    }
}




#[derive(Debug, Clone)]
pub enum Message {
    Window(WindowMsg),
    SongChangeWorker(Sender<Message>),
    DiscordWorker(Sender<DiscordMessage>),
    WebsocketWorker(Sender<WebsocketMessage>),
    ThemeChanged(Theme),
    ContentChanged(ContentView),
    CollectionViewChange(LibraryView),
    ControlChange(ControlMsg),
    AddAlbumToQueue(AlbumKey),
    SongFinished,
    ClearQueue
}

#[derive(Debug, Clone)]
pub enum WindowMsg {
    CloseWindow,
    TitlebarClick(bool),
    DragWindow,
    OpenSystemMenu,
    //MinimiseRequest,
    MinimiseWindow,
    MaximiseRequest,
    MaximiseWindow(bool),
    SetWindowId(Option<window::Id>),
}

#[derive(Debug, Clone)]
pub enum ControlMsg {
    TogglePlayback,
    Stop,
    Forward,
    Back,
    Sliding(f32),
    Seek(f32),
    SetVolume(f32),
    LoopingChanged,
    ShuffleChanged,
    UpdateDuration,
}

#[derive(PartialEq, Debug)]
enum PlayerState {
    Idle,
    Active
}
// #[derive(PartialEq, Debug)]
// enum WindowState {
//     None,
//     Minimised,
//     Maximised
// }

#[derive(PartialEq, Debug)]
enum LoopState {
    None,
    All,
    Song,
}

#[derive(PartialEq, Debug)]
enum ShuffleState {
    Off,
    On
}

pub struct Player {
    _stream_handle: OutputStream,
    sink: Sink,
    pub song_queue: VecDeque<music_library::Song>,
    pub queue_pos: usize,
    volume: f32,
    progress: f32,
    sliding: bool,
    pub current_song: Option<Song>,
    pub next_appended: bool,
    last_skipped: bool,
    pub queue_cleared: bool,
    state: PlayerState,
    loop_state: LoopState,
    shuffle_state: ShuffleState
}

pub struct App {
    current_view: ContentView,
    pub theme: Theme,
    pub library: MusicLibrary,
    pub imghandles: AHashMap<AlbumKey, image::Handle>,
    pub player: Player,
    pub version: u32,
    pub durtick: u32,
    pub song_update_sender: Option<Sender<Message>>,
    pub discord_sender: Option<Sender<DiscordMessage>>,
    pub websocket_sender: Option<Sender<WebsocketMessage>>,
    song_text_id: scrollable::Id,
    pub library_view: LibraryView,
    window_id: Option<window::Id>,
    title_clicked: bool,
    maximise_icon: Icon,
}


impl App {
    fn new() -> Self {

    let img_size = PROGRAM_CFG.img_size();

        let mut lib = music_library::scan_library(PROGRAM_CFG.library_path(), img_size);
        let stream_handle = rodio::OutputStreamBuilder::open_default_stream().unwrap();
        let sink = rodio::Sink::connect_new(stream_handle.mixer());
        sink.pause();

        let size = (img_size*img_size) as usize;
        let mut blank_img: Vec<u8> = Vec::with_capacity(size);
        //blank image bytes rgba(0,0,0,255)
        for _ in 0..size {
            blank_img.push(0);
            blank_img.push(0);
            blank_img.push(0);
            blank_img.push(255);
        }

        //pull all the album art out of the lib structure, clone, then delete
        let mut ih = AHashMap::new();
        for (key, info) in lib.get_all_albums().iter_mut() {
            let h = image::Handle::from_rgba(
                img_size, 
                img_size,
                match &info.artwork {
                    Some(art) => art.clone(),
                    None => {
                        blank_img.clone().into_boxed_slice()
                    }
                }
            );
            ih.entry(key.clone()).or_insert(h);
        }
        lib.delete_all_art();
        sink.set_volume(0.5);

        Self {
            current_view: ContentView::Library,
            library_view: LibraryView::Albums,
            theme: Theme::CatppuccinMocha,
            library: lib,
            player: Player {
                _stream_handle: stream_handle,
                sink: sink,
                volume: 0.5,
                progress: 0.,
                sliding: false,
                song_queue: VecDeque::new(),
                queue_pos: 0,
                current_song: None,
                last_skipped: false,
                queue_cleared: false,
                next_appended: false,
                state: PlayerState::Idle,
                loop_state: LoopState::None,
                shuffle_state: ShuffleState::Off
            },
            imghandles: ih,
            version: 0,
            durtick: 0,
            song_update_sender: None,
            discord_sender: None,
            websocket_sender: None,
            song_text_id: scrollable::Id::unique(),
            window_id: None,
            //window_state: WindowState::None,
            title_clicked: false,
            maximise_icon: Icon::Maximise
        }
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }

    fn title(&self) -> String {
        if self.player.current_song.is_some() {
            format!("{} by {}",
            self.player.current_song.as_ref().unwrap().title,
            self.player.current_song.as_ref().unwrap().artists[0]
            )
        }
        else {
            format!("Music Player")
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {

        match message {
            Message::Window(msg) => {
                match msg {
                    WindowMsg::MaximiseRequest => {
                        return window::get_maximized(self.window_id.unwrap())
                        .map(|r| Message::Window(WindowMsg::MaximiseWindow(!r)));
                    },
                    WindowMsg::MaximiseWindow(b) => {
                        self.title_clicked = false;
                        if b {self.maximise_icon = Icon::Unmaximise;}
                        else {self.maximise_icon = Icon::Maximise;}
                        return window::maximize(self.window_id.unwrap(), b);
                    }
                    WindowMsg::MinimiseWindow => {
                        self.title_clicked = false;
                        return window::minimize(self.window_id.unwrap(), true);
                    }
                    WindowMsg::TitlebarClick(b) => {
                        self.title_clicked = b;
                    }
                    WindowMsg::DragWindow => {
                        if self.title_clicked {
                            self.title_clicked = false;
                            return window::drag(self.window_id.unwrap());
                        }
                    }
                    WindowMsg::CloseWindow => {
                        std::process::exit(0);
                        //return window::close(self.window_id.unwrap());
                    }
                    WindowMsg::SetWindowId(id) => {
                        self.window_id = id;
                    }
                    WindowMsg::OpenSystemMenu => {
                        return show_system_menu(self.window_id.unwrap());
                    }
                }
            },
            Message::SongChangeWorker(sender) => {
                self.song_update_sender = Some(sender);
                return window::get_latest().map(|f| Message::Window(WindowMsg::SetWindowId(f)));
            },
            Message::DiscordWorker(sender) => {
                self.discord_sender = Some(sender);
            },
            Message::WebsocketWorker(sender) => {
                self.websocket_sender = Some(sender);
            },
            Message::ThemeChanged(theme) => {
                self.theme = theme;
            },
            Message::ContentChanged(msg) => {
                if self.current_view != msg {
                    self.current_view = msg;
                }
            },
            Message::ControlChange(msg) => {
                match msg {
                    ControlMsg::SetVolume(vol) => {
                        self.player.sink.set_volume(vol);
                        self.player.volume = vol;
                    },
                    ControlMsg::TogglePlayback => {
                        if self.player.sink.is_paused() && (!self.player.song_queue.is_empty() || !self.player.current_song.is_none()) {
                            if self.player.current_song.is_none() {self.get_next_song()};
                            self.player.sink.play();
                            if self.player.state == PlayerState::Idle {self.player.state = PlayerState::Active};
                        } else {
                            self.player.sink.pause();
                        }
                        self.update_discord_presence();
                        self.update_websocket();

                    },
                    ControlMsg::Stop => {
                        self.stop_player();
                    },
                    ControlMsg::Forward => {
                        self.player.last_skipped = true;
                        self.player.sink.skip_one();
                        if !self.player.sink.empty() {self.player.sink.play();}
                    },
                    ControlMsg::Back => {
                        //if next song appended already, clear out sink and put current song back in
                        if self.player.next_appended {
                            if let Ok(file) = std::fs::File::open(&self.player.current_song.as_ref().unwrap().file_path) {
                                self.player.sink.stop();
                                let src  = rodio::Decoder::new(BufReader::new(file)).unwrap();
                                self.player.sink.append(src);
                                let sender = self.song_update_sender.as_ref().unwrap().clone();
                                self.player.sink.append(EmptyCallback::new(Box::new(move || {
                                    //send Message::SongFinished to the update loop
                                    _ = sender.clone().try_send(Message::SongFinished);
                                })));
                                //look, if it fails again unlucky + i dont care + i dont know what to do
                                //might be courteous to return the user to where they were before trying to seek
                            }
                            self.player.next_appended = false;
                            self.update_discord_presence();
                            return Task::none();
                        }
                        //if more than 4 seconds into song or no previous song go to start, else go last song
                        if  self.player.sink.get_pos().as_secs_f32() > 4. ||
                            self.player.queue_pos == 0 ||
                            self.player.song_queue.get(self.player.queue_pos - 1).is_none() 
                        {
                            self.seek(0.);
                        } else {
                            self.player.sink.stop();
                            self.player.queue_pos -= 1;
                            //append previous song.
                            self.add_song_to_sink(self.player.song_queue.get(self.player.queue_pos).unwrap().clone());
                            self.player.current_song = Some(self.player.song_queue.get(self.player.queue_pos).unwrap().clone());
                            self.update_websocket();
                        }
                        self.update_discord_presence();
                    },
                    ControlMsg::Sliding(val) => {
                        self.player.sliding = true;
                        self.player.progress = val;
                    },
                    ControlMsg::Seek(val) => {
                        self.player.sliding = false;
                        if !self.player.next_appended {
                            self.seek(val);
                        }
                    },
                    ControlMsg::UpdateDuration => {
                        if let Some(song) = &self.player.current_song {
                            let cur = self.player.sink.get_pos().as_secs_f32();
                            let dur = song.duration.as_secs_f32();
                            self.player.progress = cur / dur;                                
                            /*if song almost finished,
                            not single looping,
                            and another song is in queue,
                            then append next song to sink pre-emptively*/
                            if dur - cur < 0.4 &&
                                !self.player.next_appended &&
                                self.player.loop_state != LoopState::Song &&
                                !self.player.queue_cleared &&
                                self.player.song_queue.get(self.player.queue_pos + 1).is_some() {
                                self.add_song_to_sink(self.player.song_queue.get(self.player.queue_pos + 1).unwrap().clone());
                                self.player.next_appended = true;
                            }
                            let pos = ((self.durtick % 36) as f32 * 0.05) - 0.4;
                            self.durtick += 1;
                            return scrollable::snap_to::<Message>(
                                self.song_text_id.clone(),
                                scrollable::RelativeOffset {
                                    x: pos,
                                    y: 0.
                                }
                            )
                        }
                    },
                    ControlMsg::ShuffleChanged => {
                        match self.player.shuffle_state {
                            ShuffleState::Off => self.player.shuffle_state = ShuffleState::On,
                            ShuffleState::On => self.player.shuffle_state = ShuffleState::Off
                        }
                    },
                    ControlMsg::LoopingChanged => {
                        match self.player.loop_state {
                            LoopState::None => self.player.loop_state = LoopState::All,
                            LoopState::All => self.player.loop_state = LoopState::Song,
                            LoopState::Song => self.player.loop_state = LoopState::None
                        }
                    },
                    //_ => {}
                }
            }
            Message::ClearQueue => {
                self.version = 0;
                if !self.player.next_appended {
                    self.player.queue_pos = 0;
                    self.player.queue_cleared = true;
                    self.player.song_queue.clear();
                }
            },
            Message::SongFinished => {
                self.get_next_song();
                self.update_discord_presence();
                self.update_websocket();

            }
            Message::AddAlbumToQueue(key) => {
                if let Some(info) = self.library.get_albuminfo(&key) {
                    for disc in info.discs.iter() {
                        for song in disc.tracks.iter() {
                            //add to queue.
                            self.player.song_queue.push_back(song.clone());

                        }
                    }
                    self.current_view = ContentView::Queue;
                }
            }
            Message::CollectionViewChange(view) => {
                self.library_view = view;
            }
            //_ => {}
        }
        Task::none()
    }
    
    fn view(&self) -> Element<'_, Message> {
        let sidebar = 
        container(column![
            sidebar_button(Some(Icon::Play) ,"Queue", ContentView::Queue),
            sidebar_button(Some(Icon::Note), "Library", ContentView::Library),
            //---separator element
            container(vertical_space())
            .style(|t: &Theme| {
                let mut style = container::Style::default();
                style.border.width = 1.;
                style.border.radius = 0.0.into();
                style.border.color = t.extended_palette().background.strong.color;
                style
            }).height(Fill).width(Fill),
            //---
            sidebar_button(None, "Settings", ContentView::Settings)
            .height(30)
            ]
        )
        .width(Length::Fixed(SIDEBAR_SIZE))
        .height(Fill)
        .padding(0)
        .style(sidebar_style);
        
        let current_pos;
        if self.player.state == PlayerState::Active && !self.player.sink.empty() {
            //current_pos = format_duration(self.player.sink.get_pos());
            current_pos = format_duration(self.player.current_song.as_ref().unwrap().duration.mul_f32(self.player.progress));
        }
        else {
            current_pos = "-:--:--".to_string();
        }
        let remaining_dur: String;
        if self.player.current_song.is_some() {
            //let rd = self.player.sink.get_pos();
            let rd = self.player.current_song.as_ref().unwrap().duration.mul_f32(self.player.progress);
            remaining_dur = format_duration(
            Duration::from_secs_f32(
            self.player.current_song.as_ref().unwrap().duration.as_secs_f32() - rd.as_secs_f32()));
        }
        else {
            remaining_dur = "-:--:--".to_string();
        }
        
        let mut content_padding = Padding::new(1.);
        let content = container(match &self.current_view {
            ContentView::Queue => {
                content_padding = Padding { top: 1., right: 1., bottom: 0. , left: 1. };
                queue_page(self)
            },
            ContentView::Library => library_page(self),
            ContentView::Settings => settings_page(self),
            //_ => {queue_page(self)}
        }).style(content_container_style).padding(content_padding);

        let toggle_data;
        if self.player.sink.is_paused() {
            toggle_data = Icon::Play;
        }
        else {
            toggle_data = Icon::Pause;
        }

        // CONTROL BUTTONS

        let control_buttons = row![
            top_control_button(ButtonType::Svg(Icon::Back), &ControlMsg::Back),
            top_control_button(ButtonType::Svg(toggle_data), &ControlMsg::TogglePlayback),
            top_control_button(ButtonType::Svg(Icon::Stop), &ControlMsg::Stop),
            top_control_button(ButtonType::Svg(Icon::Forward), &ControlMsg::Forward),
        ].spacing(4).height(Fill).align_y(Center);
        let control_sliders: row::Row<'_, Message> = row![
            text(current_pos).size(14),
            slider(0.0..=1.0, self.player.progress, |v| Message::ControlChange(ControlMsg::Sliding(v)))
                .on_release(Message::ControlChange(ControlMsg::Seek(self.player.progress)))
                .step(0.002)
                .style(slider_style)
                .hover_interaction(Interaction::default())
                .drag_interaction(Interaction::default()),
            text(remaining_dur).size(14),
        ].spacing(8).height(Fill).align_y(Center);
        let top_controls = row![control_buttons, control_sliders].height(38).spacing(8);

        //LOWER CONTROLS
        let mut loop_icon = Icon::Loop;
        if self.player.loop_state == LoopState::Song {
            loop_icon = Icon::Loop1
        }
        let lower_controls = row![
            horizontal_space(),
            self.lower_control_button(ButtonType::Svg(loop_icon), ControlMsg::LoopingChanged),
            self.lower_control_button(ButtonType::Svg(Icon::Shuffle), ControlMsg::ShuffleChanged),
            container(
                row![text(format!("{:.0}%", self.player.volume*100.)).width(38).align_x(Alignment::End).align_y(Alignment::Center),
                slider(0.0..=1.0, self.player.volume, |v| Message::ControlChange(ControlMsg::SetVolume(v)))
                .step(0.01)
                .style(slider_style)
                .width(120)
                .hover_interaction(Interaction::default())
                .drag_interaction(Interaction::default())
                ].spacing(8).align_y(Center)
            ).style(container::bordered_box).padding(4)
        ].align_y(Center).height(Length::Fill).spacing(4);
        

        
        let song_text: Column<'_, Message>;
        if self.player.current_song.is_some() {       
            song_text = column![
                scrollable(text(self.player.current_song.as_ref().unwrap().title.clone())
                    .size(16).align_x(Alignment::Start).line_height(LineHeight::Relative(1.4))
                    .wrapping(text::Wrapping::None))
                    .id(self.song_text_id.clone())
                    .style(song_scroll_style).width(210)
                    // .horizontal(),
                    .direction(scrollable::Direction::Horizontal(
                        scrollable::Scrollbar::new()
                            .width(0)
                            .scroller_width(0)
                    )),
                    //self.player.current_song.as_ref().unwrap().artists[0].clone()
                    scrollable(text(self.player.current_song.as_ref().unwrap().artists.join(" / "))
                    .size(14).align_x(Alignment::Start).wrapping(text::Wrapping::None))
                    .id(self.song_text_id.clone())
                    .style(song_scroll_style).width(210)
                    // .horizontal(),
                    .direction(scrollable::Direction::Horizontal(
                        scrollable::Scrollbar::new()
                            .width(0)
                            .scroller_width(0)
                    )),
            ].height(Length::Fill).width(210).spacing(0);
        }
        else {
            song_text = Column::new();
        }
           
        let ar: Element<'_, Message> = lazy(self.version, move |_version| {
        let playing_art;
        if self.player.current_song.is_some() {       

            let ih = self.imghandles.get_key_value(&AlbumKey {
                title: self.player.current_song.as_ref().unwrap().album_title.clone(),
                artist: self.player.current_song.as_ref().unwrap().artists[0].clone()});
            if ih.is_some() {
                playing_art = container(
                    image(ih.unwrap().1
                ).height(78).width(78))
            }

            else {
                playing_art = container(
                    ""
                );
            }
        }
        else {
            playing_art = container(
                ""
            );
        }
        playing_art.style(|t: &Theme| {
            let palette = t.extended_palette();
            container::Style {
                border: Border {
                    width: 2.0,
                    radius: 0.0.into(),
                    color: palette.background.strongest.color,
                },
                ..container::Style::default()
            }
        }).width(82).height(82).center(Length::Fixed(82.))
        }).into();

        let controls = container(row![ar, column![top_controls, row![song_text, lower_controls]]].spacing(4))
            .width(Fill)
            .height(Length::Fixed(90.))
            //.max_height(72)
            .padding(4)
            .style(controls_style);

        let main_area = column![content, controls]
            .width(Fill)
            .height(Fill);
        let mut titlebar = mouse_area(
            container(row![
                svg(svg::Handle::from(Icon::Album.icon_data())).height(16).width(16).style(titlebar_svg_style),
                text(self.title()).size(12).height(28).align_y(Alignment::Center),
                horizontal_space(),
            ]
            .align_y(Alignment::Center)
            .height(30)
            .spacing(6)
            )
            .padding(4)
        )
        .interaction(mouse::Interaction::None)
        .on_press(Message::Window(WindowMsg::TitlebarClick(true))) 
        .on_double_click(Message::Window(WindowMsg::MaximiseRequest))
        .on_release(Message::Window(WindowMsg::TitlebarClick(false)))
        .on_exit(Message::Window(WindowMsg::TitlebarClick(false)))
        .on_right_release(Message::Window(WindowMsg::OpenSystemMenu))
        ;
        if self.title_clicked {
           titlebar = titlebar.on_move(|_| Message::Window(WindowMsg::DragWindow));
        }

        let bar = row![
            titlebar,
            titlebar_button(Icon::Minimise, &WindowMsg::MinimiseWindow),
            titlebar_button(self.maximise_icon, &WindowMsg::MaximiseRequest),
            titlebar_button(Icon::Close, &WindowMsg::CloseWindow),
        ];
        let decoration = container(bar)
        .width(Fill)
        .height(30)
        .padding(Padding {
            top: 2.,
            right: 2.,
            bottom: 2.,
            left: 4.,
        })
        .style(titlebar_style);
    
        
        // let lower_middle = mouse_area(container("").height(5).width(Fill)).interaction(Interaction::ResizingVertically);
        // let lower_border = row![lower_middle];
        column![decoration, container(row![sidebar, main_area])
        .padding(Padding {
            top: 0.,
            right: 1.,
            bottom: 1.,
            left: 1.,
        })
        .style(|t| {
            let palette = t.extended_palette();
            let mut style = container::Style::default();
            style.border.width = 2.;
            style.border.color = palette.background.strong.color;
            style
        })
        ].into()
    }

    fn stop_player(&mut self) {
        self.player.sink.stop();
        self.player.sink.pause();
        self.player.current_song = None;
        self.update_discord_presence();
        self.update_websocket();
        //if clear_queue {self.player.song_queue.clear();}
        self.player.next_appended = false;
        if self.player.state == PlayerState::Active {self.player.state = PlayerState::Idle};
        self.player.progress = 0.;
        self.player.queue_pos = 0;
        self.version += 1;
    }
    //this is always called when a song is to be added to the sink, so this handles next song logic
    fn get_next_song(&mut self) {
        self.version += 1;
        self.durtick = 0;

        //if next song already appended, update info and return
        if self.player.next_appended {
            self.player.next_appended = false;
            self.player.queue_pos += 1;
            //unwrap should be fine because of the check in UpdateDuration message which is the only way to get here
            self.player.current_song = Some(self.player.song_queue.get(self.player.queue_pos).unwrap().clone());
            self.player.last_skipped = false;
            return;
        }

        //if single looping and user did not skip, just replay current song, do nothing else
        if self.player.loop_state == LoopState::Song && self.player.current_song.is_some() && !self.player.last_skipped && !self.player.queue_cleared {
            self.add_song_to_sink(self.player.current_song.as_ref().unwrap().clone());
            return;
        }
        //in any other case last_skipped is irrelevant
        self.player.last_skipped = false;

        //if no song is in player or queue was cleared, and there is a song at the front of the queue, play first song
        //currently shouldnt be possible for first index of song_queue to be none here but better to check anyway
        if (self.player.current_song.is_none() || self.player.queue_cleared) && self.player.song_queue.get(0).is_some() {
            self.player.queue_cleared = false;
            self.player.queue_pos = 0;
            self.add_song_to_sink(self.player.song_queue.get(self.player.queue_pos).unwrap().clone());
            self.player.current_song = Some(self.player.song_queue.get(self.player.queue_pos).unwrap().clone());
            return;
        }
        self.player.queue_cleared = false;

        // todo - if nothing past queue_pos and check for looping all
        if self.player.song_queue.get(self.player.queue_pos + 1).is_some() {
            //if next song exists, add it
            self.player.queue_pos += 1;
            self.add_song_to_sink(self.player.song_queue.get(self.player.queue_pos).unwrap().clone());
            self.player.current_song = Some(self.player.song_queue.get(self.player.queue_pos).unwrap().clone());

        }
        else {
            //otherwise check if looping is all, and if so, go to start
            if self.player.loop_state == LoopState::All && self.player.song_queue.get(0).is_some() {
                self.player.queue_pos = 0;
                self.add_song_to_sink(self.player.song_queue.get(self.player.queue_pos).unwrap().clone());
                self.player.current_song = Some(self.player.song_queue.get(self.player.queue_pos).unwrap().clone());
            } else {
                //otherwise stop player without clearing queue.
                self.stop_player();
            }
        }
    }

    fn add_song_to_sink(&mut self, song: Song) {
        if let Ok(file) = std::fs::File::open(&song.file_path) {
            //let src  = rodio::Decoder::new(BufReader::new(file)).unwrap();
            let src = DecoderBuilder::new()
            .with_seekable(true)
            .with_data(file)
            .with_hint("flac")
            .with_gapless(true)
            .build().unwrap();

        
        self.player.sink.append(src);
        let sender = self.song_update_sender.as_ref().unwrap().clone();
        self.player.sink.append(EmptyCallback::new(Box::new(move || {
            //send Message::SongFinished to the update loop
            _ = sender.clone().try_send(Message::SongFinished);
        })));
        //if !self.player.next_appended {self.player.current_song = Some(song);}
            
        }
    }

    fn update_discord_presence(&mut self) {
        if self.discord_sender.is_some() {

            if self.player.current_song.is_none() || self.player.sink.is_paused() {

                _ = self.discord_sender.as_ref().unwrap().clone()
                .try_send(DiscordMessage::ClearPresence);

            } else {
                let start = SystemTime::now().duration_since(UNIX_EPOCH.into())
                .unwrap().as_secs();
                _ = self.discord_sender.as_ref().unwrap().clone()
                .try_send(DiscordMessage::SetPresence((PresenceData {
                    artist: self.player.current_song.as_ref().unwrap().artists.join(" / "),
                    song_title: self.player.current_song.as_ref().unwrap().title.to_owned(),
                    album_title: self.player.current_song.as_ref().unwrap().album_title.to_owned(),
                    album_artist: self.player.current_song.as_ref().unwrap().artists[0].to_owned(),
                    current_pos: self.player.sink.get_pos().as_secs(),
                    song_duration: self.player.current_song.as_ref().unwrap().duration.as_secs(),
                }, start)));

            }
        }
    }

    fn update_websocket(&mut self) {
        if self.websocket_sender.is_some() {

            if self.player.current_song.is_none() || self.player.sink.is_paused() {

                _ = self.websocket_sender.as_ref().unwrap().clone()
                .try_send(WebsocketMessage::Clear);

            } else {
                let kvp = self.imghandles.get_key_value(&AlbumKey {
                    title: self.player.current_song.as_ref().unwrap_or(&Song::default()).album_title.clone(),
                    artist: self.player.current_song.as_ref().unwrap_or(&Song::default()).artists[0].clone()
                });

                if kvp.is_none() {
                    return;
                }

                let handle = kvp.unwrap().1;
                

                let mut img: Option<ImageBuffer<Rgba<u8>, Vec<u8>>> = None;
                match handle {
                    image::Handle::Rgba { id: _, width, height, pixels } => {
                        img = RgbaImage::from_vec(*width, *height, pixels.to_vec());
                    },
                    _ => {}
                }

                if img.is_some() {
                    let img = img.unwrap();
                    let mut buffer = Cursor::new(Vec::new());
                    match img.write_to(&mut buffer, ::image::ImageFormat::Png) {
                        Ok(_) => {
                            let b64img = format!("{}{}", "data:image/png;base64,", BASE64_STANDARD.encode(buffer.into_inner()));
                            _ = self.websocket_sender.as_ref().unwrap().clone()
                                .try_send(WebsocketMessage::UpdateSongData(SongData {
                                    title: self.player.current_song.as_ref().unwrap_or(&Song::default()).title.clone(),
                                    artist: self.player.current_song.as_ref().unwrap_or(&Song::default()).artists.join(" / "),
                                    album: self.player.current_song.as_ref().unwrap_or(&Song::default()).album_title.clone(),
                                    b64img: b64img,
                                    clear: None
                                }));
                        },
                        Err(_) => {
                            
                        },
                    }

                }

            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        let duration_update = match self.player.state {
            PlayerState::Idle => Subscription::none(),
            PlayerState::Active { .. } => {
                if self.player.sliding {
                    Subscription::none()
                } else {
                    time::every(milliseconds(250))
                    .map(|_| {
                        Message::ControlChange(ControlMsg::UpdateDuration)
                    })
                }
            }
        };

        fn discord_update() -> impl Stream<Item = Message> {
            stream::channel(100, async |mut output| {
                
                let mut client = DiscordClient::new();

                let mut current_img: String = String::new();
                let mut current_album: String = String::new();
                let mut current_artist: String = String::new();

                let (sender, mut receiver) = mpsc::channel::<DiscordMessage>(100);
                
                // Send the sender back to the application
                _ = output.send(Message::DiscordWorker(sender)).await;
                
                client.block_until_connected();
        
                loop {
                    let mut final_msg = receiver.next().await;
                    // discard all but the final message
                    while let Ok(msg) = receiver.try_next() {
                        final_msg = Some(msg.unwrap());
                    }


                    if final_msg.is_some() {
                        let mut data: Option<PresenceData> = None;
                        let res = match final_msg.unwrap() {
                            DiscordMessage::SetPresence((presence_data, start)) => {

                                if presence_data.album_title != current_album || presence_data.album_artist != current_artist {
                                    if let Ok(res) = get_cover_art(&presence_data.album_title, &presence_data.album_artist).await {
                                        current_img = res;
                                    }
                                }

                                let r = client.set_presence(&presence_data, &current_img, start);
                                data = Some(presence_data);
                                r
                            },
                            DiscordMessage::ClearPresence => {
                                client.clear_presence()
                            }
                        };

                        match res {
                            Ok(_) => {
                                if data.is_some() {
                                    current_album = data.as_ref().unwrap().album_title.to_owned();
                                    current_artist = data.unwrap().album_artist;
                                }
                                
                            },
                            Err(_) => {
                                client.shutdown();
                                client.block_until_connected();
                            }
                        }
                    }
                }

            })
        }

        fn playback_updater() -> impl Stream<Item = Message> {
            stream::channel(100, async |mut output| {
                // Create channel
                let (sender, mut receiver) = mpsc::channel::<Message>(100);
                // Send the sender back to the application
                _ = output.send(Message::SongChangeWorker(sender)).await;
        
                loop {
                    _ = receiver.next().await;  
                    _ = output.send(Message::SongFinished).await;
                }
            })
        }

        fn ws_update() -> impl Stream<Item = Message> {
            stream::channel(100, async |mut output| {
                // Create channel
                let (sender, receiver) = mpsc::channel::<WebsocketMessage>(100);
                // Send the sender back to the application
                _ = output.send(Message::WebsocketWorker(sender)).await;
                
                _ = websocket::ws_main(receiver).await;

            })
        }



        let worker_subscription = Subscription::run(playback_updater);
        let discord_subscription = Subscription::run(discord_update);
        let ws_subscription = Subscription::run(ws_update);
        
        let mut subs = vec![duration_update, worker_subscription];
        if PROGRAM_CFG.discord_rp_enabled() {
            subs.push(discord_subscription);
        }
        if PROGRAM_CFG.ws_enabled() {
            subs.push(ws_subscription);
        }

        Subscription::batch(subs)

    }

   fn seek(&mut self, pos: f32) {
    if !self.player.sink.empty() {
        let song = self.player.current_song.as_ref().unwrap();
        let seek_pos = song.duration.as_secs_f32() * pos;
        
        if let Err(_) = self.player.sink.try_seek(Duration::from_secs_f32(seek_pos)) {
            //couldnt seek so reload the song from the start...
            if let Ok(file) = std::fs::File::open(&self.player.current_song.as_ref().unwrap().file_path) {
                self.player.sink.stop();
                let src  = rodio::Decoder::new(BufReader::new(file)).unwrap();
                self.player.sink.append(src);
                let sender = self.song_update_sender.as_ref().unwrap().clone();
                self.player.sink.append(EmptyCallback::new(Box::new(move || {
                    //send Message::SongFinished to the update loop
                    _ = sender.clone().try_send(Message::SongFinished);
                })));
                //look, if it fails again unlucky + i dont care + i dont know what to do
                //might be courteous to return the user to where they were before trying to seek
                if pos > 0. {_ = self.player.sink.try_seek(Duration::from_secs_f32(seek_pos));}
            }
        }
        self.update_discord_presence();
    }
} 
    
fn lower_control_button(&self, kind: ButtonType, msg: ControlMsg) -> Button<'_, Message> {
    let is_on;
    match msg {
        ControlMsg::LoopingChanged => {
            is_on = !(self.player.loop_state == LoopState::None) 

        }
        ControlMsg::ShuffleChanged => {
            is_on = self.player.shuffle_state == ShuffleState::On
        }
        _ => is_on = false
    }

    match kind {
        ButtonType::_Text(txt) => {
            button(text(txt)
            .align_x(Center)
            .align_y(Center)
            .size(12))
            .height(Length::Fixed(24.))
            .width(Length::Fixed(24.))
            .style(move |t,s| lower_control_button_style(t,s,is_on))
            .on_press(Message::ControlChange(msg))
            .into()
        },
        
        ButtonType::Svg(icon) => {
            button(container(svg(svg::Handle::from(icon.icon_data()))
            .style(move |t,s| lower_control_svg_style(t,s,is_on)).height(Length::Fill)).center(Length::Fixed(24.))
            ).height(Length::Fixed(26.))
            .width(Length::Fixed(32.))
            .style(move |t,s| lower_control_button_style(t,s,is_on))
            .on_press(Message::ControlChange(msg))
            .padding(3)
            .into()
        }
    }
}

    
}

impl Default for App {
    fn default() -> Self {
        App::new()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ContentView {
    Queue,
    Settings,
    Library
}

fn titlebar_style(theme: &Theme) -> container::Style {
    let palette = theme.extended_palette();
    let mut style = container::Style::default();
    style.background = Some(palette.background.base.color.into());
    style.text_color = Some(palette.background.base.text.into());
    style.border.radius = 0.0.into();
    style.border.width = 2.;
    style.border.color = palette.background.strong.color;
    style
}


fn titlebar_button_style(theme: &Theme, status: button::Status, exit: bool) -> button::Style {
    let palette = theme.extended_palette();
    let mut style = button::Style::default();
    // style.background = Some(palette.background.weak.color.into());
    //style.text_color = palette.background.weak.text;
    style.border.width = 0.;
    style.background = Some(palette.background.base.color.into());
    style.border.color = palette.secondary.strong.color;
    style.border.radius = 0.0.into();
    // style.border.color = palette.background.strong.color;
    match status {
        button::Status::Hovered => {
            if exit {style.background = Some(palette.danger.weak.color.into());}
            else {style.background = Some(palette.background.weak.color.into());}
        },
        button::Status::Pressed => {
            if exit {style.background = Some(palette.danger.strong.color.into());}
            else {style.background = Some(palette.background.strong.color.into());}
        },
        _ => {}
    }
    style
}

fn titlebar_button(icon: Icon, msg: &WindowMsg) -> Button<'_, Message> {
    let exit;
    match msg {
        WindowMsg::CloseWindow => exit = true,
        _ => exit = false
    }
    button(container(svg(svg::Handle::from(icon.icon_data()))
    .style(titlebar_svg_style).width(10)).center(Length::Fixed(10.))
    //.align_x(Center)
    //.align_y(Center)
    ).height(Length::Fixed(30.))
    .width(Length::Fixed(45.))
    .style(move |t,s| titlebar_button_style(t, s, exit))
    .on_press(Message::Window(msg.clone()))
    .into()

}

fn titlebar_svg_style(theme: &Theme, _status: svg::Status) -> svg::Style {
    let palette = theme.extended_palette();
    let mut style = svg::Style::default();
    style.color = Some(palette.background.base.text.into());

    style
}

fn format_duration(dur: Duration) -> String {
    let total_seconds = dur.as_secs();
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;

    format!("{:02}:{:02}", minutes, seconds)
}

pub fn run() {
    let mut window_settings = Settings::default();
    window_settings.size = Size::new(640., 410.);
    window_settings.decorations = false;
    window_settings.transparent = true;
    window_settings.min_size = Some(Size::new(640., 410.));
    window_settings.icon = Some(window::icon::from_file_data(include_bytes!("../../assets/icon.png"), None).unwrap());
    _ = iced::application(App::title, App::update, App::view)
    .theme(App::theme)
    .window(window_settings)
    .subscription(App::subscription)
    .run();
}