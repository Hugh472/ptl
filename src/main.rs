fn main() {
    println!("Hello, world!");
}
use clap::{App, Arg};
use reqwest::Error;
use serde::Deserialize;
use std::process::Command;
use std::{thread, time};

#[derive(Deserialize)]
struct AppList {
    applist: Apps,
}

#[derive(Deserialize)]
struct Apps {
    apps: Vec<App>,
}

#[derive(Deserialize)]
struct App {
    appid: u32,
    name: String,
}

async fn fetch_app_list() -> Result<Vec<App>, Error> {
    let url = "https://api.steampowered.com/ISteamApps/GetAppList/v2/";
    let response: AppList = reqwest::get(url).await?.json().await?;
    Ok(response.applist.apps)
}

async fn fetch_app_id(game_name: &str, apps: &[App]) -> Option<u32> {
    for app in apps {
        if app.name == game_name {
            return Some(app.appid);
        }
    }
    None
}

fn run_protontricks(app_id: u32, mod_exe: &str) {
    Command::new("protontricks")
        .arg("-c")
        .arg(format!("wine C:/path/to/{}", mod_exe))
        .arg(format!("{}", app_id))
        .spawn()
        .expect("Failed to run protontricks");
}

fn launch_steam_game(app_id: u32) {
    Command::new("steam")
        .arg(format!("steam://run/{}", app_id))
        .spawn()
        .expect("Failed to launch Steam game");
}

#[tokio::main]
async fn main() {
    let matches = App::new("Protontricks Game Mod Launcher")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("Launches a mod manager for a specified game and then the game itself")
        .arg(Arg::new("game_name")
            .short('g')
            .long("game")
            .value_name("GAME")
            .help("The name of the game")
            .takes_value(true))
        .arg(Arg::new("mod_exe")
            .short('m')
            .long("mod")
            .value_name("EXE")
            .help("The path to the mod manager executable")
            .takes_value(true))
        .get_matches();

    let apps = fetch_app_list().await.expect("Failed to fetch app list");

    if matches.is_present("game_name") && matches.is_present("mod_exe") {
        let game_name = matches.value_of("game_name").unwrap();
        let mod_exe = matches.value_of("mod_exe").unwrap();

        match fetch_app_id(game_name, &apps).await {
            Some(app_id) => {
                println!("Found App ID: {}", app_id);
                run_protontricks(app_id, mod_exe);

                // Wait a bit to ensure the mod manager .exe has launched
                thread::sleep(time::Duration::from_secs(5));

                launch_steam_game(app_id);
            }
            None => eprintln!("Game not found: {}", game_name),
        }
    } else {
        println!("Available games in your Steam library:");
        for app in apps {
            println!("{}", app.name);
        }
        println!("\nUsage: protontricks_launcher -g <game_name> -m <path_to_mod_manager_exe>");
    }
}
