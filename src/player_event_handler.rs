use librespot::playback::player::PlayerEvent;
use std::collections::HashMap;
use std::process::Command;

fn run_program(program: &str, env_vars: HashMap<&str, String>) {
    let mut v: Vec<&str> = program.split_whitespace().collect();
    info!("Running {:?} with environment variables {:?}", v, env_vars);
    Command::new(&v.remove(0))
        .args(&v)
        .envs(env_vars.iter())
        .spawn()
        .expect("program failed to start");
}

pub fn run_program_on_events(event: PlayerEvent, onevent: &str) {
    let mut env_vars = HashMap::new();
    match event {
        PlayerEvent::Changed {
            old_track_id,
            new_track_id,
            track_info,
        } => {
            env_vars.insert("PLAYER_EVENT", "change".to_string());
            env_vars.insert("OLD_TRACK_ID", old_track_id.to_base16());
            env_vars.insert("TRACK_ID", new_track_id.to_base16());
            env_vars.insert("TRACK_NAME", track_info.track.name);
            env_vars.insert("TRACK_DURATION", track_info.track.duration.to_string());

            let artists_string: String = track_info.artists.into_iter().map(|x| { x.name }).collect::<Vec<String>>().join(" / ");
            env_vars.insert("TRACK_ARTISTS", artists_string);
            env_vars.insert("TRACK_ALBUM", track_info.album.name);
        }
        PlayerEvent::Started { track_id, track_info } => {
            env_vars.insert("PLAYER_EVENT", "start".to_string());
            env_vars.insert("TRACK_ID", track_id.to_base16());
            env_vars.insert("TRACK_NAME", track_info.track.name);
            env_vars.insert("TRACK_DURATION", track_info.track.duration.to_string());

            let artists_string: String = track_info.artists.into_iter().map(|x| { x.name }).collect::<Vec<String>>().join(" / ");
            env_vars.insert("TRACK_ARTISTS", artists_string);
            env_vars.insert("TRACK_ALBUM", track_info.album.name);
        }
        PlayerEvent::Stopped { track_id, track_info } => {
            env_vars.insert("PLAYER_EVENT", "stop".to_string());
            env_vars.insert("TRACK_ID", track_id.to_base16());
            env_vars.insert("TRACK_NAME", track_info.track.name);
            env_vars.insert("TRACK_DURATION", track_info.track.duration.to_string());

            let artists_string: String = track_info.artists.into_iter().map(|x| { x.name }).collect::<Vec<String>>().join(" / ");
            env_vars.insert("TRACK_ARTISTS", artists_string);
            env_vars.insert("TRACK_ALBUM", track_info.album.name);
        }
    }
    run_program(onevent, env_vars);
}
