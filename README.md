# Purpose

A Rust implementation of Spotify status module for Polybar. 

# Usage

Append the following module to polybar/config or corresponding config file.
```
[module/spotify-rs]
type = custom/script
exec = /path/to/rs_spotify_status/target/release/rs_spotify_status -p 33333 
tail = true
format-underline = #50fa7b
```

# Logic

The script module will spawn a process which will utilize two threads:

* One thread acting as a UDP server to listen for any commands from polybar or from another place,
* One thread periodically monitoring the Spotify DBUS for track or playback changes.

# Current limitations

* While the backend side of things are ready, we are not really able to utilize
the commands through polybar configuration. The referred [example](https://github.com/polybar/polybar-scripts/tree/master/polybar-scripts/player-mpris-tail)
utilizes it masterfully but have not figured it out yet.

* While there does not seem to be much of a need to offer detailed customization,
it should be relatively easy to add some by extending arguments.

Notes:

* Please note that this project is inspired from other examples such as:
[polybar-scripts](https://github.com/polybar/polybar-scripts/tree/master/polybar-scripts/player-mpris-tail) and [spolyfy](https://github.com/Taptiive/spolyfy)

# Additional References

* [DBUS - MediaPlayer interface](https://specifications.freedesktop.org/mpris-spec/latest/Player_Interface.html#)
* [DBUS - Rust Interface](https://docs.rs/crate/dbus/0.9.6)
