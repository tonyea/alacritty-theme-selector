use dialoguer::Select;
use std::fs;
use std::io;
use regex::Regex;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Theme {
    name: String,
    colors: Vec<String>,
}


// Function to modify the config file
fn modify_config_file(selected_file: &str) {
    // Add your logic to modify the config file using the selected file
    println!("Modifying config file using {}...", selected_file);
    let config_path: &str = "/home/aebin/.config/alacritty/alacritty.toml";

    // regex to find and replace the import statement in the config file pertaining to the theme \/themes\/themes\/([^"]+)\.toml
    let config = fs::read_to_string(config_path).unwrap();
    let regexp = Regex::new(r"/themes/themes/(.*)\.toml").unwrap();
    let new_config = regexp.replace_all(&config, format!("/themes/themes/{}", selected_file)).to_string();
    fs::write(config_path, new_config).unwrap();
    println!("Config file modified successfully!");

}

fn main() -> io::Result<()> {

    if let Some(proj_dirs) = ProjectDirs::from("", "", "Alacritty") {
        // println!("Project directory: {}", proj_dirs.config_dir().to_str().unwrap());
        let config_dir: &std::path::Path = proj_dirs.config_dir();

        let config_file = fs::read_to_string(config_dir.join("alacritty.toml")).unwrap_or("".to_string());
        dbg!(config_file);
    }

    // Define folder to read
    let folder_path: &str = "/home/aebin/.config/alacritty/themes/themes";

    let files = fs::read_dir(folder_path)?
        .map(|res: Result<fs::DirEntry, io::Error>| res.map(|e: fs::DirEntry| e.file_name()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    // Convert file names to strings
    let file_choices: Vec<String> = files
        .iter()
        .filter_map(|f| f.to_str().map(|s| s.to_string()))
        .collect();

    // Prompt the user with a multiple choice selection
    let selected_file = Select::new()
        .items(&file_choices)
        .with_prompt("Select a file to modify the config file:")
        .interact();

    match selected_file {
        Result::Ok(index) => {
            let selected_file_name = &file_choices[index];
            modify_config_file(selected_file_name);
        }
        Result::Err(err) => {
            println!("Error: {}", err);
        }
    }

    Ok(())
}