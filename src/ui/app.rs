use std::{collections::VecDeque, io::BufReader, time::Duration};
use iced::futures::channel::mpsc::{self, Sender};
use iced::futures::SinkExt;
use iced::time::{self, milliseconds};

use ahash::AHashMap;
use iced::widget::{lazy, svg, Column};
use iced::{
    gradient, widget::{button, column, container, horizontal_space, image, row, scrollable, slider, slider::HandleShape, text, Button}, window::Settings, Alignment, Border, Center, Element, Length::{self, Fill}, Size, Theme};

use rodio::decoder::DecoderBuilder;
use rodio::{source::EmptyCallback, OutputStream, Sink};
use iced::futures::StreamExt;

use iced::futures::Stream;
use iced::{stream, Subscription, Task};

use crate::musiclib::music_library::{self, AlbumKey, MusicLibrary, Song};
use super::icons::Icon;
use super::content_views::{album_page, artist_page, queue_page, settings_page};

const SIDEBAR_SIZE: f32 = 80.;

//height and width of album artwork stored in memory
pub const IMG_SIZE: u32 = 120;


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

fn control_svg_style(theme: &Theme, _status: svg::Status) -> svg::Style {
    let palette = theme.extended_palette();
    let mut style = svg::Style::default();
    style.color = Some(palette.secondary.base.text.into());
    style
}

fn sidebar_button(txt: &str, msg: ContentView) -> Button<'_, Message> {
    button(text(txt)
    .align_x(Center)
    .align_y(Center))
    .height(Length::Fixed(SIDEBAR_SIZE))
    .width(Length::Fixed(SIDEBAR_SIZE))
    .style(sidebar_button_style)
    .on_press(Message::ContentChanged(msg))
    .into()
}

fn song_scroll_style(theme: &Theme, status: scrollable::Status) -> scrollable::Style {
    let palette = theme.extended_palette();
    let mut style = scrollable::default(theme, status);
    style.vertical_rail.background = Some(palette.background.weakest.color.into());
    //style.horizontal_rail.scroller
    //style.vertical_rail.scroller.border.radius = 0.into();
    //style.vertical_rail.border.radius = 0.into();

    style
}


enum ButtonType {
    Text(String),
    Svg(Icon)
}

fn top_control_button(kind: ButtonType, msg: ControlMsg) -> Button<'static, Message> {
    match kind {
        ButtonType::Text(txt) => {
            button(text(txt)
            .align_x(Center)
            .align_y(Center)
            .size(14))
            .height(Length::Fixed(36.))
            .width(Length::Fixed(36.))
            .style(control_button_style)
            .on_press(Message::ControlChange(msg))
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
            .on_press(Message::ControlChange(msg))
            .into()
        }
    }
}

fn lower_control_button(kind: ButtonType, msg: ControlMsg) -> Button<'static, Message> {
    match kind {
        ButtonType::Text(txt) => {
            button(text(txt)
            .align_x(Center)
            .align_y(Center)
            .size(12))
            .height(Length::Fixed(24.))
            .width(Length::Fixed(24.))
            .style(control_button_style)
            .on_press(Message::ControlChange(msg))
            .into()
        },
        

        ButtonType::Svg(icon) => {
            button(container(svg(svg::Handle::from(icon.icon_data()))
            .style(control_svg_style).height(Length::Fill)).center(Length::Fixed(24.))
            ).height(Length::Fixed(26.))
            .width(Length::Fixed(32.))
            .style(control_button_style)
            .on_press(Message::ControlChange(msg))
            .padding(3)
            .into()
        }
    }
}



#[derive(Debug, Clone)]
pub enum Message {
    BackgroundWorker(Sender<Message>),
    ThemeChanged(Theme),
    ContentChanged(ContentView),
    ControlChange(ControlMsg),
    AddAlbumToQueue(AlbumKey),
    SongFinished,
    ClearQueue
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

#[derive(PartialEq, Debug)]
enum LoopState {
    None,
    Single,
    All
}

enum ShuffleState {
    Off,
    On
}

pub struct Player {
    _stream_handle: OutputStream,
    sink: Sink,
    pub song_queue: VecDeque<music_library::Song>,
    volume: f32,
    progress: f32,
    sliding: bool,
    pub current_song: Option<Song>,
    pub next_appended: bool,
    state: PlayerState,
    looping: LoopState,
    // queue: Arc<SourcesQueueInput>,
    // queue_handle: SourcesQueueOutput
}

pub struct App {
    current_view: ContentView,
    pub theme: Theme,
    pub library: MusicLibrary,
    pub imghandles: AHashMap<AlbumKey, image::Handle>,
    pub player: Player,
    pub version: u32,
    pub durtick: u32,
    pub sender: Option<Sender<Message>>,
    song_text_id: scrollable::Id,
}


impl App {
    fn new() -> Self {
        let mut lib = music_library::scan_library("C:/Users/Isaac/Music".to_string(), IMG_SIZE);
        //let stream_handle = rodio::OutputStreamBuilder::open_default_stream().unwrap();
        let stream_handle = rodio::OutputStreamBuilder::open_default_stream().unwrap();
        let sink = rodio::Sink::connect_new(stream_handle.mixer());
        sink.pause();
        //let (queue, queue_handle) = queue::queue(true);
        //pull all the album art out of the lib structure, clone, then delete
        let mut ih = AHashMap::new();
        for (key, info) in lib.get_all_albums().iter_mut() {
            let h = image::Handle::from_rgba(
                IMG_SIZE, 
                IMG_SIZE,
                match &info.artwork {
                    Some(art) =>art.clone(),
                    //blank image bytes rgba(0,0,0,255)
                    None => [[0, 0, 0, 255]; (IMG_SIZE*IMG_SIZE) as usize].as_flattened().to_vec().into_boxed_slice()
                }
            );
            ih.entry(key.clone()).or_insert(h);
        }
        lib.delete_all_art();
        sink.set_volume(0.5);

        Self {
            current_view: ContentView::Queue,
            theme: Theme::CatppuccinMacchiato,
            library: lib,
            player: Player {
                _stream_handle: stream_handle,
                sink: sink,
                volume: 0.5,
                progress: 0.,
                sliding: false,
                song_queue: VecDeque::new(),
                current_song: None,
                next_appended: false,
                state: PlayerState::Idle,
                looping: LoopState::None,
                // queue: queue,
                // queue_handle: queue_handle
            },
            imghandles: ih,
            version: 0,
            durtick: 0,
            sender: None,
            song_text_id: scrollable::Id::unique(),
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
            Message::BackgroundWorker(sender) => {
                self.sender = Some(sender);
            }
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

                    },
                    ControlMsg::Stop => {
                        self.player.sink.stop();
                        self.player.sink.pause();
                        self.player.current_song = None;
                        self.player.song_queue.clear();
                        self.player.next_appended = false;
                        if self.player.state == PlayerState::Active {self.player.state = PlayerState::Idle};
                        self.player.progress = 0.;
                        self.version -= 1;
                    },
                    ControlMsg::Forward => {
                        self.player.sink.skip_one();
                        if !self.player.sink.empty() {self.player.sink.play();}
                    },
                    ControlMsg::Back => {
                        if self.player.next_appended {
                            if let Ok(file) = std::fs::File::open(&self.player.current_song.as_ref().unwrap().file_path) {
                                self.player.sink.stop();
                                let src  = rodio::Decoder::new(BufReader::new(file)).unwrap();
                                self.player.sink.append(src);
                                let sender = self.sender.as_ref().unwrap().clone();
                                self.player.sink.append(EmptyCallback::new(Box::new(move || {
                                    //send Message::SongFinished to the update loop
                                    _ = sender.clone().try_send(Message::SongFinished);
                                })));
                                //look, if it fails again unlucky + i dont care + i dont know what to do
                                //might be courteous to return the user to where they were before trying to seek
                            }
                            self.player.next_appended = false;
                        }
                        self.seek(0.);
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

                        //self.player.sink
                    },
                    ControlMsg::UpdateDuration => {
                        if let Some(song) = &self.player.current_song {
                            self.durtick += 1;
                            let cur = self.player.sink.get_pos().as_secs_f32();
                            let dur = song.duration.as_secs_f32();
                            self.player.progress = cur / dur;                                
                            if  dur - cur < 0.4 &&
                                !self.player.next_appended &&
                                self.player.looping != LoopState::Single &&
                                !self.player.song_queue.is_empty() {
                                //if song almost finished, append next song to sink
                                self.add_song_to_sink(self.player.song_queue.front().unwrap().clone());
                                self.player.next_appended = true;
                            }
                            let pos = ((self.durtick % 37) as f32 * 0.05) - 0.4;
                            return scrollable::snap_to::<Message>(
                                self.song_text_id.clone(),
                                scrollable::RelativeOffset {
                                    x: pos,
                                    y: 0.
                                }
                            )
                        }

                        //println!("event");
                    }
                    _ => {}
                }
            }
            Message::ClearQueue => {
                self.version = 0;
                if !self.player.next_appended {self.player.song_queue.clear();}
            },
            Message::SongFinished => {
                //println!("song finished");
                self.get_next_song();
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
        }
        Task::none()
    }
    
    fn view(&self) -> Element<'_, Message> {

        //container().
        //button("A").
        let sidebar = 
        container(scrollable(column![
            sidebar_button("Queue", ContentView::Queue),
            sidebar_button("Albums", ContentView::Albums),
            sidebar_button("Artists", ContentView::Artists),
            sidebar_button("Settings", ContentView::Settings)
            .height(30)
            ])
            
        )
        .width(Length::Fixed(SIDEBAR_SIZE))
        .height(Fill)
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
        let content = match self.current_view {
            ContentView::Albums => album_page(self),
            ContentView::Artists => artist_page(self),
            ContentView::Queue => queue_page(self),
            ContentView::Settings => settings_page(self),
            //_ => {queue_page(self)}
        };

        let toggle_data;
        if self.player.sink.is_paused() {
            toggle_data = Icon::Play;
        }
        else {
            toggle_data = Icon::Pause;
        }
        let control_buttons = row![
            top_control_button(ButtonType::Svg(Icon::Back), ControlMsg::Back),
            top_control_button(ButtonType::Svg(toggle_data), ControlMsg::TogglePlayback),
            top_control_button(ButtonType::Svg(Icon::Stop), ControlMsg::Stop),
            top_control_button(ButtonType::Svg(Icon::Forward), ControlMsg::Forward),
        ].spacing(4).height(Fill).align_y(Center);
        let control_sliders: row::Row<'_, Message> = row![
            text(current_pos).size(14),
            slider(0.0..=1.0, self.player.progress, |v| Message::ControlChange(ControlMsg::Sliding(v)))
                .on_release(Message::ControlChange(ControlMsg::Seek(self.player.progress))).step(0.002).style(slider_style),
            text(remaining_dur).size(14),
        ].spacing(8).height(Fill).align_y(Center);
        let top_controls = row![control_buttons, control_sliders].height(38).spacing(8);



        let lower_controls = row![
            horizontal_space(),
            lower_control_button(ButtonType::Svg(Icon::Loop), ControlMsg::LoopingChanged),
            lower_control_button(ButtonType::Svg(Icon::Shuffle), ControlMsg::ShuffleChanged),
            container(
                row![text(format!("{:.0}%", self.player.volume*100.)).width(38).align_x(Alignment::End).align_y(Alignment::Center),
                slider(0.0..=1.0, self.player.volume, |v| Message::ControlChange(ControlMsg::SetVolume(v))).step(0.01).style(slider_style).width(120)
                ].spacing(8).align_y(Center)
            ).style(container::bordered_box).padding(4)
        ].align_y(Center).height(Length::Fill).spacing(4);
        

        
        let song_text: Column<'_, Message>;
        if self.player.current_song.is_some() {       
            song_text = column![
                scrollable(text(self.player.current_song.as_ref().unwrap().title.clone())
                    .size(16).align_x(Alignment::Start).height(Length::Shrink).wrapping(text::Wrapping::None))
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
                    .size(14).align_x(Alignment::Start))
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

        let controls = container(row![ar, column![top_controls, row![song_text.spacing(4), lower_controls]]].spacing(4))
            .width(Fill)
            .height(Length::Fixed(90.))
            //.max_height(72)
            .padding(4)
            .style(controls_style);

        let main_area = column![content, controls]
            .width(Fill)
            .height(Fill);
        
        row![sidebar, main_area].into()

    }

    //this is always called when a song is to be added to the sink, so this handles next song logic
    fn get_next_song(&mut self) {
        self.version += 1;
        self.durtick = 0;
        //if next song already appended, update info and return
        if self.player.next_appended {
            self.player.next_appended = false;
            self.player.current_song = Some(self.player.song_queue.pop_front().unwrap());
            return;
        }

        if !self.player.song_queue.is_empty() {
            self.add_song_to_sink(self.player.song_queue.front().unwrap().clone());
            self.player.current_song = Some(self.player.song_queue.pop_front().unwrap());

        }
        else {
            self.player.current_song = None;
            self.player.sink.pause();
            self.player.progress = 0.;
            self.player.state = PlayerState::Idle;
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
        let sender = self.sender.as_ref().unwrap().clone();
        self.player.sink.append(EmptyCallback::new(Box::new(move || {
            //send Message::SongFinished to the update loop
            _ = sender.clone().try_send(Message::SongFinished);
        })));
        //if !self.player.next_appended {self.player.current_song = Some(song);}
            
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
        
        fn playback_updater() -> impl Stream<Item = Message> {
            stream::channel(100, async |mut output| {
                // Create channel
                let (sender, mut receiver) = mpsc::channel::<Message>(100);
                // Send the sender back to the application
                _ = output.send(Message::BackgroundWorker(sender)).await;
        
                loop {
                    _ = receiver.next().await;  
                    _ = output.send(Message::SongFinished).await;
                }
            })
        }


        let worker_subscription = Subscription::run(playback_updater);

        Subscription::batch(vec![duration_update, worker_subscription])
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
                let sender = self.sender.as_ref().unwrap().clone();
                self.player.sink.append(EmptyCallback::new(Box::new(move || {
                    //send Message::SongFinished to the update loop
                    _ = sender.clone().try_send(Message::SongFinished);
                })));
                //look, if it fails again unlucky + i dont care + i dont know what to do
                //might be courteous to return the user to where they were before trying to seek
                if pos > 0. {_ = self.player.sink.try_seek(Duration::from_secs_f32(seek_pos));}
            }
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
    Albums,
    Artists,
    Queue,
    Settings
}

fn format_duration(dur: Duration) -> String {
    let total_seconds = dur.as_secs();
    //let milliseconds = self.duration.subsec_millis();

    //let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;

    format!("{:02}:{:02}", minutes, seconds)
}


pub fn run() -> iced::Result {

    let mut window_settings = Settings::default();
    window_settings.size = Size::new(640., 380.);
    window_settings.min_size = Some(Size::new(640., 380.));

    iced::application(App::title, App::update, App::view)
    .theme(App::theme)
    .window(window_settings)
    .subscription(App::subscription)
    .run()
}