extern crate rspotify;
extern crate yaml_rust;

use clap::{load_yaml, App};
use rspotify::client::Spotify;
use rspotify::oauth2::{SpotifyClientCredentials, SpotifyOAuth};
use rspotify::util::get_token;
use yaml_rust::YamlLoader;

//static client_id: &'static str = env!("client_id");
//static client_secret: &'static str = env!("client_secret");
static REDIRECT_URI: &'static str = "http://localhost:8888/callback";

#[tokio::main]
async fn main() {
    let config_path: &str = &format!("{}/.config/spt-utils/client.yml", std::env::var("HOME").unwrap());

    let auth_yaml = YamlLoader::load_from_str(&std::fs::read_to_string(config_path).unwrap()).unwrap();

    let auth = &auth_yaml[0];
    
    let client_id = auth["client_id"].as_str().unwrap();
    let client_secret = auth["client_secret"].as_str().unwrap();




    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();

    let mut oauth = SpotifyOAuth::default()
        .client_id(client_id)
        .client_secret(client_secret)
        .redirect_uri(REDIRECT_URI)
        .scope("user-read-currently-playing user-read-playback-state")
        .build();
    match get_token(&mut oauth).await {
        Some(token_info) => {
            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info)
                .build();
            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();
            if let Some(_) = matches.subcommand_matches("get-currently-playing") {
                let res = get_currently_playing(spotify).await;
                match res {
                    Some(s) => println!("{:?}", s),
                    None => return,
                }
            } else if let Some(_) = matches.subcommand_matches("next-song") {
                match next_song(spotify).await {
                    Ok(_) => return,
                    Err(e) => eprintln!("{}", e),
                }
            } else if let Some(_) = matches.subcommand_matches("prev-song") {
                match prev_song(spotify).await {
                    Ok(_) => return,
                    Err(e) => eprintln!("{}", e),
                }
            } else if let Some(_) = matches.subcommand_matches("play-pause") {
                match play_pause(spotify).await {
                    Ok(_) => return,
                    Err(e) => eprintln!("{}", e),
                }
            }
        }
        None => println!("auth failed"),
    };
}

async fn play_pause(spotify: Spotify) -> Result<(), String> {
    let devices = spotify.device().await;
    let device = Some((&devices.unwrap().devices.get(0).unwrap().id).clone());

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
    let device = Some((&devices.unwrap().devices.get(0).unwrap().id).clone());
    match spotify.next_track(device).await {
        Ok(_) => Ok(()),
        Err(_) => Err(String::from("Error changing to next song")),
    }
}

async fn prev_song(spotify: Spotify) -> Result<(), String> {
    let devices = spotify.device().await;
    let device = Some((&devices.unwrap().devices.get(0).unwrap().id).clone());
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
                    track.artists.get(0).unwrap().name
                )),
            },
        },
    }
}
