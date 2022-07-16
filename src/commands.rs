use crate::utils;
use dbus::blocking::{Connection, Proxy};
use serde::{Serialize, Deserialize};

/// Defines enum of defined commands that can
/// control the behavior of currently playing
/// song.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum SongCommand {
    /// Sends a NEXT command over the DBUS to
    /// skip the current song. If skipping is
    /// not possible (no next song in the list)
    /// then it will take no affect.
    Next,

    /// Sends a PREVIOUS command over the DBUS to
    /// rewind or revisit the previous song. If
    /// there is no song prior to the current one
    /// then it will only rewind it.
    Previous,

    /// Sends a PAUSE command over the DBUS to pause
    /// the current song. If it is already paused, then
    /// it will take no affect.
    Pause,

    /// Sends a PLAYPAUSE command over the DBUS. Affectively,
    /// if the current status is paused, then it will play.
    /// Otherwise, it will pause the song.
    PlayPause,

    /// Sends a STOP command over the DBUS. If the current
    /// song already stopped, then it will have no affect.
    Stop,

    /// Sends a PLAY command over the DBUS. If the current song
    /// is already playing, then it will have no affect.
    Play,
}

impl SongCommand {
    /// Executes the command based on the current type.
    /// Returns Ok(()), when the command is executed successfully,
    /// an error otherwise.
    pub fn execute(&self, proxy: &Proxy<&Connection>, ) -> Result<(), dbus::Error> {
        match self {
            SongCommand::Next => self.next(&proxy),
            SongCommand::Previous => self.previous(&proxy),
            SongCommand::Pause => self.pause(&proxy),
            SongCommand::PlayPause => self.play_pause(&proxy),
            SongCommand::Stop => self.stop(&proxy),
            SongCommand::Play => self.play(&proxy)
        }
    }

    /// Sends a NEXT command over the DBUS to the spotify.
    fn next(&self, proxy: &Proxy<&Connection>) -> Result<(), dbus::Error> {
        proxy.method_call(utils::MEDIA_PLAYER_DATA_PATH, "Next", ()) 
    }

    /// Sends a PREVIOUS command over the DBUS to the spotify.
    fn previous(&self, proxy: &Proxy<&Connection>) -> Result<(), dbus::Error>{
        proxy.method_call(utils::MEDIA_PLAYER_DATA_PATH, "Previous", ()) 
    }

    /// Sends a PAUSE command over the DBUS to the spotify.
    fn pause(&self, proxy: &Proxy<&Connection>) -> Result<(), dbus::Error>{
        proxy.method_call(utils::MEDIA_PLAYER_DATA_PATH, "Pause", ()) 
    }

    /// Sends a PLAYPAUSE command over the DBUS to the spotify.
    fn play_pause(&self, proxy: &Proxy<&Connection>) -> Result<(), dbus::Error>{
        proxy.method_call(utils::MEDIA_PLAYER_DATA_PATH, "PlayPause", ()) 
    }

    /// Sends a STOP command over the DBUS to the spotify.
    fn stop(&self, proxy: &Proxy<&Connection>) -> Result<(), dbus::Error>{
        proxy.method_call(utils::MEDIA_PLAYER_DATA_PATH, "Stop", ()) 
    }
    
    /// Sends a PLAY command over the DBUS to the spotify.
    fn play(&self, proxy: &Proxy<&Connection>) -> Result<(), dbus::Error>{
        proxy.method_call(utils::MEDIA_PLAYER_DATA_PATH, "Play", ()) 
    }
}
