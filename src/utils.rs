use std::error::Error;
use std::format;
use std::time::Duration;
use dbus::blocking::{Connection, Proxy};
use dbus::blocking::stdintf::org_freedesktop_dbus::{Peer, Properties};
use dbus::arg::PropMap;

pub static MEDIA_PLAYER_DATA_PATH: &str = "org.mpris.MediaPlayer2.Player";
pub static SPOTIFY_PATH: &str = "org.mpris.MediaPlayer2.spotify";
pub static MEDIA_PLAYER_PROXY_PATH: &str = "/org/mpris/MediaPlayer2";
pub static ICON_PLAYING: &str = " "; 
pub static ICON_PAUSED: &str = " ";
pub static ICON_STOPPED: &str = " ";
pub static ICON_PREVIOUS: &str = " ";
pub static ICON_NEXT: &str = " ";
pub static ICON_SPOTIFY: &str = " ";

/// Defines various statuses may occur during a song's lifetime. 
pub enum PlaybackStatus {
    /// The player status when currently there is a song playing.
    Playing,

    /// The player status when the current song is paused.
    Paused,

    /// The player status when the spotify exited, or stopped.
    Stopped,

    /// An unknown status to be parsed/noted later.
    Unknown,
}

impl From<String> for PlaybackStatus {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "playing" => PlaybackStatus::Playing,
            "paused" => PlaybackStatus::Paused,
            "stopped" => PlaybackStatus::Stopped,
            _ => PlaybackStatus::Unknown,
        }
    }
}

/// Tests whether the open proxy is still connectable
pub fn is_connectable(proxy: &Proxy<&Connection>) -> bool {
    match proxy.ping() {
        Ok(_) => true,
        Err(_) => false
    }
}

/// Attempts to create a proxy using the given session.
/// If successful, it will return the proxy itself.
/// Otherwise, the caller should have a defined behavior.
pub fn create_proxy_for_spotify(conn_session: &Connection) -> Option<Proxy<&Connection>> {
    let proxy = conn_session.with_proxy(
        SPOTIFY_PATH,
        MEDIA_PLAYER_PROXY_PATH,
        Duration::from_millis(5000));

    // test connectivity:
    // if ping did not generate any errors, then it is safe to assume that
    // proxy is functional 
    match proxy.ping() {
        Ok(_) => Some(proxy), 
        Err(_) => None
    }
} 

/// By using the created proxy, it attempts to obtain the metadata from the
/// DBUS and fetch necessary fields so that it can be published.
/// NOTE: If this function cannot obtain the metadata, or any of the fields
///       it will generate an error, which the caller should handle.
pub fn create_song_string(proxy: &Proxy<&Connection>) -> Result<String, Box<dyn Error>> {
    // get the metadata:
    let metadata: PropMap = proxy.get(MEDIA_PLAYER_DATA_PATH, "Metadata")?;
    let mut artist_names = String::from("Unknown");
    if let Some(artists_raw) = metadata.get("xesam:artist") {
        // there is at least one artist and it is represented
        // as an iterable.
        let artists = artists_raw
            .0
            .as_iter()
            .unwrap()
            .map(|artist| artist.as_str().unwrap())
            .collect::<Vec<&str>>();
        artist_names= artists.join(", ");
    };

    // get the track (title) name
    let mut track_name = String::from("Unknown track");
    if let Some(title) = metadata.get("xesam:title") {
       track_name = (*title.0).as_str().unwrap().to_string();
    }

    // get the album
    let mut album_name = String::from("Unknown album");
    if let Some(album) = metadata.get("xesam:album") {
       album_name = (*album.0).as_str().unwrap().to_string();
    }

    Ok(format!("{}: {} - {}", artist_names, track_name, album_name))
}

/// Gets the playback status of the current track.
pub fn get_playback_status(proxy: &Proxy<&Connection>) -> Result<PlaybackStatus, dbus::Error> {
    let status: String = proxy.get(MEDIA_PLAYER_DATA_PATH, "PlaybackStatus")?;

    Ok(PlaybackStatus::from(status))
}
