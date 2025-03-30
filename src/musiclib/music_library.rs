use std::{collections::BTreeMap, ffi::OsStr, path::PathBuf, sync::{Arc, Mutex} , time::Duration};
use tokio::runtime::Runtime;
use tokio::task;

use ahash::AHashSet;
use image::{imageops::FilterType, DynamicImage};
//use parking_lot::Mutex;
use jwalk::WalkDir;
//use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use symphonia::core:: meta::StandardTagKey;
use super::metadata_probe;

#[derive(Default, Clone, Debug)]
pub struct Song {
    pub disc_number: u32,
    pub track_number: u32,
    pub title: String,
    pub duration: Duration,
    pub artists: Vec<String>,
    pub file_path: PathBuf,
    pub album_title: String,
}

impl Song {
    pub fn format_duration(&self) -> String {
        let total_seconds = self.duration.as_secs();
        let minutes = (total_seconds % 3600) / 60;
        let seconds = total_seconds % 60;
    
        format!("{:02}:{:02}", minutes, seconds)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AlbumKey {
    pub title: String,
    pub artist: String,
}

#[derive(Debug)]
pub struct AlbumInfo {
    pub discs: Vec<Disc>,
    pub artwork: Option<Box<[u8]>>,
}

#[derive(Debug)]
pub struct Disc {
    pub disc_number: u32,
    pub tracks: Vec<Song>
}

#[derive(Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ArtistKey {
    pub name: String,
}

pub struct Artist {
    pub albums: BTreeMap<AlbumKey, AlbumInfo>
}

#[derive(Default)]
pub struct MusicLibrary {
    pub artists: BTreeMap<ArtistKey, Artist>
}

impl MusicLibrary {
    pub fn add_song(&mut self, song: Song, album_key: AlbumKey, art: Option<Box<[u8]>>) {

        let artist_key = ArtistKey {name: album_key.artist.clone()};
        
        // Get or create the Artist entry in the library.
        let artist_entry = self.artists.entry(artist_key).or_insert(Artist {
            albums: BTreeMap::new(),
        });

        // Get or create the AlbumInfo entry in the Artist's albums.
        // let album_entry = artist_entry
        //     .albums
        //     .entry(album_key)
        //     .or_insert(AlbumInfo {
        //         discs: vec![],
        //         artwork: art,
        //     });
        let album_entry;
        match art {
            None => {
                album_entry = artist_entry
                .albums
                .entry(album_key)
                .or_insert(AlbumInfo {
                    discs: vec![],
                    artwork: art,
                }); 
            },
            _ => {
                album_entry = artist_entry
                .albums
                .entry(album_key)
                .or_insert(AlbumInfo {
                    discs: vec![],
                    artwork: None,
                });
                album_entry.artwork = art;
            },
        }

        // Get the disc number from the song.
        let disc_number = song.disc_number;

        // Find an existing Disc with the same disc number.
        let disc = album_entry
            .discs
            .iter_mut()
            .find(|d| d.disc_number == disc_number);

        // If a Disc with the same number exists, add the song to it.
        // Otherwise, create a new Disc and add the song.
        match disc {
            Some(disc) => disc.tracks.push(song),
            None => {
                let new_disc = Disc {
                    disc_number,
                    tracks: vec![song],
                };
                album_entry.discs.push(new_disc);
            }
        }
    }

    pub fn total_songs(&self) -> usize {
        self.artists
            .values()
            .flat_map(|artist| artist.albums.values())
            .flat_map(|album| album.discs.iter())
            .map(|disc| disc.tracks.len())
            .sum()
    }

    pub fn total_artists(&self) -> usize {
        self.artists
            .values()
            .count()
    }

    pub fn total_albums(&self) -> usize {
        self.artists
            .values()
            .flat_map(|artist| artist.albums.values())
            .count()
    }

    pub fn get_artists(&self) -> Vec<(&ArtistKey, &Artist)> { //-> &BTreeMap<ArtistKey, Artist>
        return self.artists.iter().collect();
    }

    pub fn get_all_albums(&self) -> Vec<(&AlbumKey, &AlbumInfo)> {
        self.artists
            .values()
            .flat_map(|artist| artist.albums.iter())
            .collect()
    }

    pub fn delete_art(&mut self, album_key: &AlbumKey) {
        let artist = self.artists.get_mut(&ArtistKey { name: album_key.artist.clone()}).unwrap();
        artist.albums.get_mut(album_key).unwrap().artwork = None;
    }

    pub fn delete_all_art(&mut self) {
        for artist in self.artists.values_mut() {
            for album in artist.albums.values_mut() {
                album.artwork = None;
            }
        }
    }

    pub fn get_albuminfo(&self, album_key: &AlbumKey) -> Option<&AlbumInfo> {
        if let Some(x) = self.artists.get(&ArtistKey { name: album_key.artist.clone()}) {
            if let Some(y) = x.albums.get(album_key) {
                return Some(y)
            }
        }
        None
    }

    pub fn album_has_art(&self, album_key: &AlbumKey) -> bool {
        if let Some(x) = self.artists.get(&ArtistKey { name: album_key.artist.clone()}) {
            if let Some(y) = x.albums.get(album_key) {
                return y.artwork.is_some();
            }
        }
        return false;
    }

    pub fn get_artist_albums(&self, artist_key: &ArtistKey) -> Vec<(&AlbumKey, &AlbumInfo)> {
        self.artists.get_key_value(artist_key).unwrap().1.albums.iter().collect()
    }

    pub fn sort(&mut self) {
        for artist in self.artists.values_mut() {
            for album in artist.albums.values_mut() {
                album.discs.sort_by(|a, b| a.disc_number.cmp(&b.disc_number));
                for disc in album.discs.iter_mut() {
                    disc.tracks.sort_by(|a, b| a.track_number.cmp(&b.track_number));
                }
            }
        }
    }



}

pub fn scan_library(path: String, img_size: u32) -> MusicLibrary {  

    //_________________Creating Library Structs

    //let mut library = MusicLibrary::default();
    let lib_mutex = Arc::new(Mutex::new(MusicLibrary::default()));
    //_________________END OF
    
    // let audio_extensions: AHashSet<OsString> = AHashSet::from_iter(
    //     ["aiff", "aif", "caf", "m4a", "mka", "ogg", "oga", "wav", "flac", "mp3"]
    //     .map(OsString::from)
    // );
    let audio_extensions: [&OsStr; 10] = 
    ["mp3", "aif", "caf", "m4a", "mka", "ogg", "oga", "wav", "flac", "aiff"]
    .map(OsStr::new);

    let artworks_processed: Arc<Mutex<AHashSet<AlbumKey>>> = Arc::new(Mutex::new(AHashSet::new()));
    let count = Arc::new(Mutex::new(0));

    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let mut tasks = Vec::new();
        for entry in WalkDir::new(path.trim_end_matches('"'))
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|f| f.path().extension().is_some_and(|f| audio_extensions.contains(&f)))
            .map(|f| f.path().to_path_buf())
            .collect::<Vec<PathBuf>>()
        {
            //let audio_extensions_clone = audio_extensions.clone();
            let library_clone = lib_mutex.clone();
            let artworks_processed_clone = artworks_processed.clone();
            let count_clone = count.clone();

            tasks.push(task::spawn(probe_file(
                entry,
                library_clone,
                artworks_processed_clone,
                count_clone,
                img_size
            )));
        }
        for task in tasks {
            task.await.unwrap();
        }
    });
    
    let mut lib = {    
        let mut library_guard = lib_mutex.lock().unwrap();
        std::mem::take(&mut *library_guard) // Take the library
    };
    lib.sort();
    lib

}

async fn probe_file(
    entry: PathBuf,
    library: Arc<Mutex<MusicLibrary>>,
    artworks_processed: Arc<Mutex<AHashSet<AlbumKey>>>,
    count: Arc<Mutex<i32>>,
    img_size: u32
) {
    if let (Some(m), dur ) = metadata_probe::probe_metadata(&entry) {
        let mut current_song = Song::default();
        let mut album_key = AlbumKey::default();
        current_song.file_path = entry;
        current_song.duration = dur;

        //iterate through and check for standard tags we need
        for tag in m.tags() {
            if tag.is_known() {
                //unwrap should be fine as is_known() is only true if std_key is some
                match tag.std_key.unwrap() {
                    StandardTagKey::TrackTitle => {
                        current_song.title = tag.value.to_string();
                    },
                    StandardTagKey::Artist => {
                        current_song.artists.push(tag.value.to_string());
                    },
                    StandardTagKey::AlbumArtist => {
                        if album_key.artist == String::default() {
                            album_key.artist = tag.value.to_string();
                        }
                    },
                    StandardTagKey::Album => {
                        album_key.title = tag.value.to_string();
                    },
                    StandardTagKey::TrackNumber => {
                        if let Ok(val) = tag.value.to_string().parse() {
                            current_song.track_number = val;
                        }
                    },
                    StandardTagKey::DiscNumber => {
                        if let Ok(val) = tag.value.to_string().parse::<u32>() {
                            current_song.disc_number = val;
                        }
                    },
                    _ => {}
                }
            }
        }
        //get album art
        let album_art;
        let mut guard = artworks_processed.lock().unwrap();
        //if album_key has not had art processed yet
        if !guard.contains(&album_key) {
            guard.insert(album_key.clone());
            drop(guard);
            if let Some(art) = m.visuals().first() {
                //println!("IMAGE !!");
                let img = image::load_from_memory(&art.data);
                if img.is_ok() {
                    let rs = img.unwrap().resize(img_size, img_size, FilterType::Triangle);
                    match rs {
                        DynamicImage::ImageRgba8(img) => {
                            // Directly use the raw data from Rgba8 image.
                            album_art = Some(img.into_raw().into_boxed_slice());
                        }
                        _ => {
                            // Convert other image types to Rgba8.
                            let img = rs.to_rgba8();
                            album_art = Some(img.into_raw().into_boxed_slice());
                        }
                    }
                    *count.lock().unwrap() += 1;
                }
                else {
                    album_art = None;
                }
                
                //album_art = Some(art.to_owned().data);
            }
            else {
                album_art = None;
            }
        }
        else {
            drop(guard);
            album_art = None;
        }
        if album_key.artist.is_empty() {
           album_key.artist = "Unknown".to_string();
        }
        if album_key.title.is_empty() {
            album_key.title = "Unknown".to_string();
         }
        current_song.album_title = album_key.title.clone();
        //println!("{:?}", album_art.clone().unwrap().iter());
        if current_song.artists.len() == 0 {
            current_song.artists.push(album_key.artist.clone());
        }
        if current_song.disc_number == 0 {
            current_song.disc_number = 1;
        }
        //after we get the metadata we need, use logic to populate library
        library.lock().unwrap().add_song(current_song, album_key, album_art);
    }
}