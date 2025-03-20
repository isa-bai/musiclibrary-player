
use std::net::Shutdown;

use iced::{gradient, padding, widget::{button, column, container, horizontal_space, image, lazy, pick_list, row, scrollable, svg, text, tooltip, Button, Column}, Alignment, Element, Length::{self, Fill, Shrink}, Theme};
use super::{app::{App, Message, IMG_SIZE}, icons::Icon};


const ARTWORK_BORDER: u32 = 4;

pub fn album_page(app: &App) -> Element<'_, Message> {
    lazy(app.version, move |_version| {
        let mut grid = Column::new().padding(padding::top(20));

        let mut this_row: Vec<Element<'_, Message>> = Vec::new();
        this_row.push(horizontal_space().into());
        for (key, _info) in app.library.get_all_albums() {
            let h = app.imghandles.get(key).unwrap();

            if this_row.len() >= 6 {
                //empty 10px container to offset scrollbar centering
                this_row.push(container("").width(10).into());

                grid = grid.push(row(this_row));
                this_row = Vec::new();
                this_row.push(horizontal_space().into());
            }
            this_row.push(
                //album entry_____________________________
                button(tooltip(
                    container(
                        image(h).width(IMG_SIZE).height(IMG_SIZE)
                        )
                        .clip(true)
                        .style(artwork_container_style)
                        .width(IMG_SIZE)
                        .height(IMG_SIZE)
                        .center_x(IMG_SIZE+(2*ARTWORK_BORDER))
                        .center_y(IMG_SIZE+(2*ARTWORK_BORDER)),
                text(
                    format!("{}\n{}", key.artist, key.title)
                ).wrapping(text::Wrapping::None),
                tooltip::Position::Top
                ).style(tooltip_style).padding(8))
                .height(IMG_SIZE+(2*ARTWORK_BORDER))
                .width(IMG_SIZE+(2*ARTWORK_BORDER))
                .on_press(Message::AddAlbumToQueue(key.to_owned()))
                .padding(0)
                .into()
                //__________________________________________
            );
            this_row.push(horizontal_space().into());
        }
        //offset scrollbar
        this_row.push(container("").width(10).into());
        grid = grid.push(row(this_row).padding(padding::bottom(20)));
        //println!("some image");


        container(scrollable(grid.spacing(20))
        .width(Fill)
        )
        .style(gradient_background)
        .width(Fill)
        .height(Fill)
    }).into()
    
}

// #[derive(Debug, Clone)]
// enum ArtistPageMessage {
//     Expand,
//     Close
// }

pub fn artist_page(app: &App) -> Element<'_, Message> {
    let list = container(scrollable(
        column(
            app.library.get_artists().iter().enumerate()
            .map(|artist| {
                row![
                    //button(">"),
                    container(text(artist.1.0.name.clone()))
                    .style(move |t|artist_style(t, artist.0))
                    .width(Fill)
                    .padding(5)
                ].into()

                
            })
        )
    ).style(scrollable_style))
    .width(Fill)
    .height(Fill);
    list.into()

}


pub fn queue_page(app: &App) -> Element<'_, Message> {

    let clearbtn: Button<'_, Message> = match app.player.next_appended {
        false => button("Clear").on_press(Message::ClearQueue),
        true => button("Clear")
    };
    let header = container(
        row![
            button("None"),
            horizontal_space(),
            text("Queue")
                .width(Fill)
                .height(36)
                .center()
                .size(24),
            horizontal_space(),
            clearbtn


        ]
    ).width(Fill).height(36).style(header_style);
    let mut l: Column<'_, Message> = Column::new();
    if let Some(cur_song) = &app.player.current_song {
        //button(">"),
        l = l.push(container(text(format!("{:02}: {} - {}    {}", cur_song.track_number, cur_song.artists[0], cur_song.title, cur_song.format_duration())))
        .style(container::primary)
        .width(Fill)
        .padding(5));

    }
    for (i, song) in app.player.song_queue.iter().enumerate(){
        let item = row![
            //button(">"),
            container(text(format!("{:02}: {} - {}    {}", &song.track_number, &song.artists[0], &song.title, &song.format_duration())))
            .style(move |t|artist_style(t, i))
            .width(Fill)
            .padding(5)
        ];
        l = l.push(item);
    };
    
    
    let list = container(scrollable(
        l
        // column(
            
        //     app.player.song_queue.iter().enumerate()
        //     .map(|song: (usize, &crate::musiclib::music_library::Song)| {
        //         row![
        //             //button(">"),
        //             container(text(format!("{:02}: {} - {}    {}", &song.1.track_number, &song.1.artists[0], &song.1.title, &song.1.format_duration())))
        //             .style(move |t|artist_style(t, song.0))
        //             .width(Fill)
        //             .padding(5)
        //         ].into()

                
        //     })
        // )
    ).style(scrollable_style))
    .width(Fill)
    .height(Fill);

    column![header, list].into()
}

pub fn songs_page(_app: &App) -> Element<'_, Message> {
    container("songs page")
    .width(Fill)
    .height(Fill)
    .padding(10).into()
    // container("many settings !")
    // .width(Fill)
    // .height(Fill)
    // .padding(10).into()
}

pub fn settings_page(app: &App) -> Element<'_, Message> {
    container(row![
        text("Theme: ")
        .shaping(text::Shaping::Advanced)
        .height(28)
        .size(14)
        .align_y(Alignment::Center),
        pick_list(Theme::ALL, Some(&app.theme), Message::ThemeChanged)
        .width(Shrink)
        .text_size(14)
    ].height(28)).width(Fill)
    .height(Fill)
    .padding(10).into()
}

#[derive(Debug, Clone, PartialEq)]
pub enum CollectionView {
    Songs,
    Albums,
    Artists
}

pub fn collection_page(app: &App) -> Element<'_, Message> {
    let header = container(
        row![
            horizontal_space(),
            header_button(Icon::Note, "Songs".into(), CollectionView::Songs),
            horizontal_space(),
            header_button(Icon::Album, "Albums".into(), CollectionView::Albums),
            horizontal_space(),
            header_button(Icon::Artist, "Artists".into(), CollectionView::Artists),
            horizontal_space(),
            container("").width(10)
        ]
    ).width(Fill).height(44).style(header_style).padding(4);

    let content = match app.collection_view {
        CollectionView::Songs => songs_page(app),
        CollectionView::Albums => album_page(app),
        CollectionView::Artists => artist_page(app),
        //_ => container("").into()
    };
    column![header, container(content).height(Fill)].into()
    //container(header).height(Fill).into()
}

fn header_style(theme: &Theme) -> container::Style {
    let palette = theme.extended_palette();
    let mut style = container::Style::default();
    style.background = Some(palette.background.base.color.into());
    style.text_color = Some(palette.background.base.text.into());
    style.border.radius = 6.0.into();
    style.border.width = 2.;
    style.border.color = palette.background.strong.color;
    style
}

pub fn tooltip_style(theme: &Theme) -> container::Style {
    let palette = theme.extended_palette();
    let mut style = container::Style::default();
    let gradient = gradient::Linear::new(0)
        .add_stop(0.0, palette.background.base.color)
        //.add_stop(0., palette.background.base.color)
        .add_stop(1.0, palette.background.weak.color);
    
    style.background = Some(gradient.into());
    style.text_color = Some(palette.background.strong.text.into());
    style.border.radius = 6.0.into();
    style.border.width = 1.5;
    style.border.color = palette.background.strongest.color;
    style
}

fn artwork_container_style(theme: &Theme) -> container::Style {
    let palette = theme.extended_palette();
    let mut style = container::Style::default();
    style.background = Some(palette.secondary.base.color.into());
    style.border.color = palette.background.strongest.color;
    style.border.radius = 0.0.into();
    style.border.width = ARTWORK_BORDER as f32;
    style
}

fn gradient_background(theme: &Theme) -> container::Style {
    let palette = theme.extended_palette();
    let mut style = container::Style::default();

    let gradient = gradient::Linear::new(0)
        .add_stop(0.1, palette.background.weak.color)
        //.add_stop(0., palette.background.base.color)
        .add_stop(0.7, palette.background.base.color);
    
    style.background = Some(gradient.into());

    style

}

fn artist_style(theme: &Theme, alt: usize) -> container::Style {
    let palette = theme.extended_palette();
    let mut style = container::Style::default();
    if alt % 2 == 0 {
        style.background = Some(palette.background.weak.color.into());
    }
    else {
        //let c1 = palette.background.base.color.into_rgba8();
        //let c2 = palette.background.weak.color.into_rgba8();
        //c2[0]+c1[0]) >> 1, (c2[1]+c1[1]) >> 1, (c2[2]+c1[2]) >> 1
        style.background = Some(palette.background.base.color.into());
    }
    style.border.width = 0.5;
    style.border.color = palette.background.strong.color;
    style
}

fn scrollable_style(theme: &Theme, status: scrollable::Status) -> scrollable::Style {
    let palette = theme.extended_palette();
    let mut style = scrollable::default(theme, status);
    style.vertical_rail.background = Some(palette.background.weakest.color.into());
    //style.vertical_rail.scroller.border.radius = 0.into();
    //style.vertical_rail.border.radius = 0.into();

    style
}

fn header_button(icon: Icon, txt: String, msg: CollectionView) -> Button<'static, Message> {


    button(container(row![
        svg(svg::Handle::from(icon.icon_data()))
        .style(header_svg_style)
        .width(36),
        text(txt).size(24)
        .align_y(Alignment::Center)
        .height(36)
    ].spacing(4))
    .width(Fill)
    .align_x(Alignment::Start)
    .center(Length::Fill)
    .height(36)
    ).height(Length::Fixed(36.))
    .padding(4)
    .width(130)
    .style(header_button_style)
    .on_press(Message::CollectionViewChange(msg))
    .into()
}

fn header_button_style(theme: &Theme, status: button::Status) -> button::Style {
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

fn header_svg_style(theme: &Theme, _status: svg::Status) -> svg::Style {
    let palette = theme.extended_palette();
    let mut style = svg::Style::default();
    style.color = Some(palette.secondary.base.text.into());
    style
}