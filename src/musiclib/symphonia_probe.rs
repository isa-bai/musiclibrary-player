use std::{fs::File, path::Path, time::Duration};

use symphonia::core::{formats::FormatOptions, io::{MediaSource, MediaSourceStream}, meta::{MetadataOptions, MetadataRevision}, probe::Hint};

pub fn probe_metadata(path: &Path) -> (Option<MetadataRevision>, Duration) {
    //let source = Box::new(File::open(path).unwrap()) as Box<dyn MediaSource>;
    let mut song_duration = Duration::default();
    let source;
    if let Ok(s) = File::open(path).map(|file| Box::new(file) as Box<dyn MediaSource>) {
        source = s;
    }
    else {
        return (None, song_duration);
    }

    let mut hint = Hint::new();
    hint.with_extension("flac");

    let mss = MediaSourceStream::new(source, Default::default());
    let format_opts = FormatOptions::default();
    let metadata_opts: MetadataOptions = Default::default();

    let probed = symphonia::default::get_probe().format(&hint, mss, &format_opts, &metadata_opts);
    if probed.is_ok() {
        let mut probed = probed.unwrap();


        let tracks = probed.format.tracks();
        for(_, track) in tracks.iter().enumerate() {
            let params = &track.codec_params;
            if let Some(n_frames) = params.n_frames {
                if let Some(tb) = params.time_base {
                    let seconds = (n_frames as f64 * tb.numer as f64) / tb.denom as f64;
                    song_duration = Duration::from_secs_f64(seconds);
                }
            }
        }

        // Prefer metadata that's provided in the container format, over other tags found during the
        // probe operation.
        if let Some(metadata_rev) = probed.format.metadata().current() {
            if !metadata_rev.tags().is_empty() {
                return (Some(metadata_rev.to_owned()), song_duration);
            }
        }

    }
    return (None, song_duration);
}