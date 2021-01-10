extern crate rspotify;
extern crate yaml_rust;

use clap::{load_yaml, App};
use rspotify::client::Spotify;
use rspotify::oauth2::{SpotifyClientCredentials, SpotifyOAuth};
use rspotify::util::get_token;
use yaml_rust::YamlLoader;

//static client_id: &'static str = env!("client_id");
//static client_secret: &'static str = env!("client_secret");
static REDIRECT_URI: &str = "http://localhost:8889/callback";

#[tokio::main]
async fn main() {
    let config_path: &str = &format!(
        "{}/.config/spt-utils/client.yml",
        std::env::var("HOME").expect("value of $HOME")
    );

    let auth_yaml = YamlLoader::load_from_str(
        &std::fs::read_to_string(config_path).expect("contents as string of file at config_path"),
    )
    .expect("Yaml containing client.yml");

    let auth = &auth_yaml[0];

    let client_id = auth["client_id"].as_str().expect("client_id string");
    let client_secret = auth["client_secret"]
        .as_str()
        .expect("client_secret string");

    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();

    let mut oauth = SpotifyOAuth::default()
        .client_id(client_id)
        .client_secret(client_secret)
        .redirect_uri(REDIRECT_URI)
        .scope("user-read-currently-playing user-read-playback-state app-remote-control")
        .build();

    match get_token(&mut oauth).await {
        Some(token_info) => {
            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info)
                .build();
            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();
            if matches
                .subcommand_matches("get-currently-playing")
                .is_some()
            {
                let res = get_currently_playing(spotify).await;
                if let Some(s) = res {
                    println!("{:?}", s)
                };
            } else if matches.subcommand_matches("next-song").is_some() {
                match next_song(spotify).await {
                    Ok(_) => {}
                    Err(e) => eprintln!("{}", e),
                }
            } else if matches.subcommand_matches("prev-song").is_some() {
                match prev_song(spotify).await {
                    Ok(_) => {}
                    Err(e) => eprintln!("{}", e),
                }
            } else if matches.subcommand_matches("play-pause").is_some() {
                match play_pause(spotify).await {
                    Ok(_) => {}
                    Err(e) => eprintln!("{}", e),
                }
            }
        }
        None => println!("auth failed"),
    };
}

async fn play_pause(spotify: Spotify) -> Result<(), String> {
    let devices = spotify.device().await;
    let device = Some(
        (&devices
            .expect("Devices structure")
            .devices
            .get(0)
            .expect("First device structure in vec")
            .id)
            .clone(),
    );

    let playing = spotify.current_user_playing_track().await;
    match playing {
        Err(e) => Err(format!("no result, {:?}", e)),
        Ok(p) => match p {
            None => return Ok(()),
            Some(p) => {
                if p.is_playing {
                    match spotify.pause_playback(device).await {
                        Ok(_) => Ok(()),
                        Err(_) => Err(String::from("Error changing to next song")),
                    }
                } else {
                    match spotify.start_playback(device, None, None, None, None).await {
                        Ok(_) => Ok(()),
                        Err(_) => Err(String::from("Error changing to next song")),
                    }
                }
            }
        },
    }
}

async fn next_song(spotify: Spotify) -> Result<(), String> {
    let devices = spotify.device().await;
    let device = Some(
        (&devices
            .expect("Devices structure")
            .devices
            .get(0)
            .expect("First device structure in vec")
            .id)
            .clone(),
    );
    match spotify.next_track(device).await {
        Ok(_) => Ok(()),
        Err(_) => Err(String::from("Error changing to next song")),
    }
}

async fn prev_song(spotify: Spotify) -> Result<(), String> {
    let devices = spotify.device().await;
    let device = Some(
        (&devices
            .expect("Devices structure")
            .devices
            .get(0)
            .expect("First device structure in vec")
            .id)
            .clone(),
    );
    match spotify.previous_track(device).await {
        Ok(_) => Ok(()),
        Err(_) => Err(String::from("Error changing to previous song")),
    }
}

async fn get_currently_playing(spotify: Spotify) -> Option<String> {
    let playing = spotify.current_user_playing_track().await;
    match playing {
        Err(_) => None,
        Ok(p) => match p {
            None => None,
            Some(p) => match p.item {
                None => None,
                Some(track) => Some(format!(
                    "{} - {}",
                    track.name,
                    track.artists.get(0).expect("first track artist").name
                )),
            },
        },
    }
}
