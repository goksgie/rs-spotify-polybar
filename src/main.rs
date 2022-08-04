mod utils;
mod arguments;
mod commands;

use std::net::UdpSocket;
use std::time::Duration;
use std::thread;
use dbus::blocking::{Connection};
use clap::Parser;
use serde_json;

/// Given the connection session to DBUS, it attempts to update the
/// track status. If no track is found, then it will be same as no-op.
/// The function includes an infinite loop. Thus, it would be best to use it
/// within a thread.
fn status_update_loop(conn_session: &Connection) {
    let mut proxy = utils::create_proxy_for_spotify(&conn_session);
    let mut old_song_info = String::from("");
    let mut old_icon_playback_status = String::from("");

    loop {
        thread::sleep(Duration::from_millis(3500));
        let current_song_info: String;

        if proxy.is_none() || !utils::is_connectable(&proxy.as_ref().unwrap()) {
            proxy = utils::create_proxy_for_spotify(&conn_session);

            // this failure indicates a connectivity issue, at least momentarily
            // the next update will fix it if it is aa temporary issue
            if old_icon_playback_status != utils::ICON_STOPPED.to_string() {
                old_icon_playback_status = utils::ICON_STOPPED.to_string();
                println!("{} {}", utils::ICON_SPOTIFY, utils::ICON_STOPPED);
            }

            continue;
        }
        
        // if we cannot obtain the song string, it is likely due to
        // some weird error.
        match utils::create_song_string(proxy.as_ref().unwrap()) {
            Ok(song) => current_song_info = song,
            Err(_) => current_song_info = String::from("An error occured while parsing"),
        }
        
        let icon_playback_status = match utils::get_playback_status(&proxy.as_ref().unwrap()) {
            Ok(utils::PlaybackStatus::Playing) => utils::ICON_PLAYING, 
            Ok(utils::PlaybackStatus::Paused) => utils::ICON_PAUSED,
            Err(_) | Ok(_) => utils::ICON_STOPPED,
        };

        let icon_playback_status = icon_playback_status.to_string();
        
        // only produce output when there is a need, which may occur
        // whether the song name or the playback status changed.
        if old_song_info != current_song_info 
            || old_icon_playback_status != icon_playback_status {
            old_song_info = current_song_info;
            old_icon_playback_status = icon_playback_status;
            
            // if the connection is broken and somehow we ended up with
            // stopped state, we will not produce the song information.
            // instead, we will go with stopped version
            if old_icon_playback_status == utils::ICON_STOPPED.to_string() {
                println!("{} {}", utils::ICON_SPOTIFY, utils::ICON_STOPPED);
                continue;
            }

            println!(
                " {} {}",
                old_icon_playback_status,
                old_song_info);
        }
    }
}

/// Creates a server on localhost:port_number and listens for any
/// command to be passed from polybar. 
fn command_loop(conn_session: &Connection, mut port: u32) {
    let mut local_host = format!("127.0.0.1:{}", port);
    let socket = UdpSocket::bind(local_host);
    while socket.is_err() {
        // on multi-monitor setup, status bar spawned as separate
        // entities. Thus, it also spawns multiple scripts. In this case,
        // in order to have buttons function, we bump the socket number by one
        // so that it will work.
        port += 1;
        local_host = format!("127.0.0.1:{}", port);
        let socket = UdpSocket::bind(local_host);
        thread::sleep(Duration::from_millis(1000));
    }

    let socket = socket.unwrap();
    socket.set_nonblocking(false).unwrap();
    let mut proxy = utils::create_proxy_for_spotify(&conn_session);
    let mut buf = [0; 2048];

    loop {
        // if proxy is not alive, then it does not make sense to execute
        // any commands. This could happen when the spotify application
        // is not working but somehow we have received a command to execute.
        // we can simply ignore that.
        if proxy.is_none() || !utils::is_connectable(&proxy.as_ref().unwrap()) {
            proxy = utils::create_proxy_for_spotify(&conn_session);
            thread::sleep(Duration::from_millis(3500));
            continue;
        }

        let (sz_data, _) = socket.recv_from(&mut buf).unwrap();
        let data = &buf[..sz_data];
        let deserialized_cmd_res: Result<commands::SongCommand, serde_json::Error> =
            serde_json::from_slice(&data);
        // an unsupported command will be ignored.
        // Additionally, if the proxy is no longer active,
        // we will not be able to execute the command. Thus, it is better
        // to test for connectivity here as a preemtive manner before proceeding. 
        if deserialized_cmd_res.is_err() {
            continue; 
        }

        let deserialized_cmd = deserialized_cmd_res.unwrap();
        
        // check the status, if it is stopped, we cannot execute commands
        match utils::get_playback_status(&proxy.as_ref().unwrap()) {
            Err(_) | Ok(utils::PlaybackStatus::Stopped) => continue,
            _ => {},
        };
        
        // now that we have a legit command, we can execute it:
        if deserialized_cmd.execute(&proxy.as_ref().unwrap()).is_err() {
            println!("An error occured while executing a command: {:?}", deserialized_cmd);
            panic!("An error while executing command: {:?}", deserialized_cmd);
        }
    }
}

/// Creates two threads, 
/// one periodically checks for status updates on DBUS for Spotify
/// one checking for any commands from the polybar or any other application
/// that controls the bar.
fn main() {
    let args = arguments::Arguments::parse();
    let port = args.port;

    if port <= 1024 || port > 63335 {
        panic!("Invalid port number is given, expected: 1024-63335");
    }

    let command_thread = thread::spawn(move || {
        let session = Connection::new_session().unwrap();
        command_loop(&session, port); 
    });
    let status_thread = thread::spawn(|| {
        let session = Connection::new_session().unwrap();
        status_update_loop(&session); 
    });

    for handler in [command_thread, status_thread] {
        if handler.join().is_err() {
            panic!("Could not join spawned thread");    
        }
    }
}
