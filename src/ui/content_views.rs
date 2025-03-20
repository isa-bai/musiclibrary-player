use iced::{gradient, padding, widget::{button, column, container, horizontal_space, image, lazy, pick_list, row, scrollable, svg, text, text::{LineHeight, Wrapping}, tooltip, Button, Column}, Alignment, Element, Length::{self, Fill, FillPortion, Shrink}, Theme};
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
        false => button("Clear").height(36).on_press(Message::ClearQueue),
        true => button("Clear").height(36)
    };
    let header = container(
        row![
            button("None")
            .height(36),
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

    let fields =
        container(row![
            container(text("#").wrapping(Wrapping::None)).style(move |t|queue_style(t, usize::MAX, app.player.queue_pos))
            .width(FillPortion(30))
            .clip(true)
            .padding(padding::Padding {top: 0.,right: 4.,bottom: 0.,left: 4.,}),
            container(text("Title").wrapping(Wrapping::None)).style(move |t|queue_style(t, usize::MAX, app.player.queue_pos))
            .width(FillPortion(260))
            .clip(true)
            .padding(padding::Padding {top: 0.,right: 4.,bottom: 0.,left: 4.,}),
            container(text("Artist").wrapping(Wrapping::None)).style(container::bordered_box).style(move |t|queue_style(t, usize::MAX, app.player.queue_pos))
            .width(FillPortion(200))
            .clip(true)
            .padding(padding::Padding {top: 0.,right: 4.,bottom: 0.,left: 4.,}),
            container(text("Length").wrapping(Wrapping::None)).style(move |t|queue_style(t, usize::MAX, app.player.queue_pos))
            .width(FillPortion(60))
            .clip(true)
            .padding(padding::Padding {top: 0.,right: 4.,bottom: 0.,left: 4.,}),
        ]
    );



    let mut l: Column<'_, Message> = Column::new();

    
    for (i, song) in app.player.song_queue.iter().enumerate(){
        let item = row![

        container(text(format!("{:02}", i + 1)).wrapping(Wrapping::None)).style(move |t|queue_style(t, i, app.player.queue_pos))
        .width(FillPortion(30))
        .clip(true)
        .padding(padding::Padding {
            top: 0.,
            right: 4.,
            bottom: 0.,
            left: 4.,
        }),
        container(text(song.title.clone()).wrapping(Wrapping::None)).style(move |t|queue_style(t, i, app.player.queue_pos))
        .width(FillPortion(260))
        .clip(true)
        .padding(padding::Padding {
            top: 0.,
            right: 4.,
            bottom: 0.,
            left: 4.,
        }),
        container(text(song.artists.join(" / ")).wrapping(Wrapping::None)).style(container::bordered_box).style(move |t|queue_style(t, i, app.player.queue_pos))
        .width(FillPortion(200))
        .clip(true)
        .padding(padding::Padding {
            top: 0.,
            right: 4.,
            bottom: 0.,
            left: 4.,
        }),
        container(text(song.format_duration()).wrapping(Wrapping::None)).style(move |t|queue_style(t, i, app.player.queue_pos))
        .width(FillPortion(60))
        .clip(true)
        .padding(padding::Padding {
            top: 0.,
            right: 4.,
            bottom: 0.,
            left: 4.,
        })
            //button(">"),
            // container(
            //     text(format!("{:02}: {} - {}    {}", i + 1, &song.artists[0], &song.title, &song.format_duration()))
            //     .size(16)
            //     .line_height(LineHeight::Relative(1.))
            //     .height(24)
            //     .align_y(Alignment::Center)
            // )
            // .style(move |t|queue_style(t, i, app.player.queue_pos))
            // .width(Fill)
            // //.padding(5)
            // .height(24)
        ];
        l = l.push(item);
    };
    
    
    let list = container(scrollable(
        l

    ).style(scrollable_style))
    .width(Fill)
    .height(Fill);

    column![header, fields, list].into()
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CollectionView {
    Songs,
    Albums,
    Artists
}

pub fn collection_page(app: &App) -> Element<'_, Message> {
    let header = container(
        row![
            horizontal_space(),
            header_button(Icon::Note, "Songs".into(), CollectionView::Songs, &app.collection_view),
            horizontal_space(),
            header_button(Icon::Album, "Albums".into(), CollectionView::Albums, &app.collection_view),
            horizontal_space(),
            header_button(Icon::Artist, "Artists".into(), CollectionView::Artists, &app.collection_view),
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

fn queue_style(theme: &Theme, alt: usize, pos: usize) -> container::Style {
    let palette = theme.extended_palette();
    let mut style = container::Style::default();
    match alt {
        alt if alt == pos => {
            style.background = Some(palette.primary.base.color.into());
            style.text_color = Some(palette.primary.base.text);
        }
        alt if alt == usize::MAX => {
            style.background = Some(palette.secondary.base.color.into());
            style.text_color = Some(palette.secondary.base.text);
        }
        _ => {
            if alt % 2 == 0 {
                style.background = Some(palette.background.weak.color.into());
                style.text_color = Some(palette.background.weak.text);
            }
            else {
                //let c1 = palette.background.base.color.into_rgba8();
                //let c2 = palette.background.weak.color.into_rgba8();
                //c2[0]+c1[0]) >> 1, (c2[1]+c1[1]) >> 1, (c2[2]+c1[2]) >> 1
                style.background = Some(palette.background.base.color.into());
                style.text_color = Some(palette.background.base.text);
            }           
        }
        
    }
    style.border.color = palette.background.strong.color;
    style.border.width = 1.;
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

fn header_button(icon: Icon, txt: String, msg: CollectionView, app_view: &CollectionView) -> Button<'_, Message> {

    button(container(row![
        svg(svg::Handle::from(icon.icon_data()))
        .style(header_svg_style)
        .width(36),
        text(txt).size(22)
        .align_y(Alignment::Center)
        .height(36)
        .line_height(LineHeight::Relative(1.))
    ].spacing(4))
    .width(Fill)
    .align_x(Alignment::Start)
    .center(Length::Fill)
    .height(36)
    ).height(Length::Fixed(36.))
    .padding(4)
    .width(130)
    .style(move |t,s| header_button_style(t,s, msg, app_view.clone()))
    .on_press(Message::CollectionViewChange(msg))
    .into()
}

fn header_button_style(theme: &Theme, status: button::Status, view: CollectionView, app_view: CollectionView) -> button::Style {
    let palette = theme.extended_palette();
    let mut style = button::Style::default();
    
    //style.text_color = palette.background.weak.text;
    //style.border.width = 2.;
    style.border.color = palette.background.strong.color;
    style.border.radius = 6.0.into();
    // style.border.color = palette.background.strong.color;

    match status {
        button::Status::Active => {
            style.background = Some(palette.background.base.color.into());
            style.text_color = palette.background.base.text;
            if view == app_view {style.background = Some(palette.background.strong.color.into());}
        },
        button::Status::Pressed => {
            style.background = Some(palette.background.strong.color.into());
            style.text_color = palette.background.strong.text;
            if view == app_view {style.background = Some(palette.background.strong.color.into());}
        },
        button::Status::Hovered => {
            style.background = Some(palette.background.weak.color.into());
            style.border.width = 2.;
            style.text_color = palette.background.weak.text;
        },
        _ => {}
    }
    style
}

fn header_svg_style(theme: &Theme, _status: svg::Status) -> svg::Style {
    let palette = theme.extended_palette();
    let mut style = svg::Style::default();
    style.color = Some(palette.background.base.text.into());
    style
}