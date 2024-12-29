mod utils;

use std::{env, format, io};
use std::env::current_exe;
use regex::{Regex};
use reqwest;
use std::fs;
use std::fs::{remove_file, File};
use std::path::Path;
use reqwest::header::USER_AGENT;
use serde_json::{from_str};
use crate::utils::logger::{fatal, info, success, warning};
use crate::utils::types::{MMUConfig, ModGroup, Mod, APIResult, ReleaseData, ParsedPath};

fn parse_path(path: &str) -> ParsedPath {
    let mut split_path: Vec<&str> = path.split("\\").collect();
    let file_name = split_path[split_path.len() - 1];
    split_path.truncate(split_path.len() - 1);

    ParsedPath {
        path: split_path.join("\\"),
        file_name: file_name.to_string()
    }
}

fn load_config() -> MMUConfig {
    let folder_path_buf = current_exe().unwrap();
    let folder_path = folder_path_buf.to_str().unwrap();

    let path = parse_path(folder_path).path;

    let binding = fs::read_to_string(format!("{}\\mmu_config.json", path))
        .expect("Should have been able to read the file");
    let contents = binding.as_str();

    let data: MMUConfig = from_str(contents).unwrap();

    data
}

fn build_github_url(url: String) -> String {
    // check the link is the correct format
    let re = Regex::new(r"https://github.com/[a-zA-Z-]+/[a-zA-Z-]+").unwrap();
    let is_valid_res = re.find(&url);

    let is_valid = match is_valid_res {
        Some(res) => res,
        None => {
            warning!("Could not match regex for {}", url);
            return "".to_string()
        }
    };

    if is_valid.len() != url.len() {
        warning!("'{}' is not a valid url, you should provide links to github repos.", url);
        return "".to_string()
    }

    let split_url: Vec<&str> = url.split("/").collect();

    let repo_id = split_url[split_url.len() - 2].to_owned() + "/" + split_url[split_url.len() - 1]; // have to add a &str to a String not a &str to a &str apparently

    format!("https://api.github.com/repos/{}/releases/latest", repo_id)
}

fn get_download_link(api_url: String, mod_data: &Mod) -> Result<ReleaseData, String> {
    let client = reqwest::blocking::Client::new();
    let res = client.get(api_url)
        .header(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:135.0) Gecko/20100101 Firefox/135.0")
        .send()
        .unwrap()
        .text()
        .unwrap();

    let res_json: APIResult = from_str::<APIResult>(res.as_str())
        .unwrap();

    for release in res_json.assets.iter().cloned() {
        let split_name: Vec<&str> = mod_data.pattern.split("*").collect();


        if release.name.starts_with(split_name[0]) && release.name.ends_with(split_name[1]) {
            return Ok(release);
        }
    }

    Err(format!("Skipping {}: Could not a file in the latest release that matches the pattern.", mod_data.name))
}

fn search(mods: &MMUConfig, term: String) -> Result<ModGroup, String> {
    let mut found = false;
    let mut res: ModGroup = ModGroup { name: "".to_string(), mods: vec![], location: "".to_string() };

    for mod_group in mods.mods.iter() {
        if mod_group.name == term {
            res = (*mod_group).clone();
            found = true;
        }
    }

    if found {Ok(res)} else {Err(format!("Could not find '{}'", term))}
}

fn update(mod_group: &ModGroup) {
    // check the mod path exists
    if Path::new(mod_group.location.as_str()).exists() == false {
        warning!("The location found in the config file does not exist.");
        return
    }


    for m in mod_group.mods.iter() {
        let link = build_github_url(m.clone().download_link);

        if link.len() == 0 {return}

        let release_data_res = get_download_link(link, m);

        let release_data = match release_data_res {
            Ok(res) => res,
            Err(message) => {
                warning!("{}", message);
                continue
            }
        };

        let file_name = parse_path(&release_data.browser_download_url).path;

        // check that the current file is not already the latest version
        if Path::new(format!("{}\\{}", mod_group.location, file_name).as_str()).exists() == true {
            info!("{} is already on the latest version.", m.name);
            continue
        }

        // download the new file
        let client = reqwest::blocking::Client::new();
        let send_res = client.get(&release_data.browser_download_url)
            .header(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:135.0) Gecko/20100101 Firefox/135.0")
            .send();

        let text_res = match send_res {
            Ok(res) => res.bytes(),
            Err(_) => {
                warning!("Failed to download {}, moving to next mod.", m.name);
                continue
            }
        };

        let mut res = match text_res {
            Ok(res) => res,
            Err(_) => {
                warning!("Failed to parse download result for {}, moving to next mod.", m.name);
                continue
            }
        };
        // delete the old file
        let paths = fs::read_dir(&mod_group.location).unwrap();

        let split_pattern: Vec<&str> = m.pattern.split("*").collect();

        let mut done = false;

        for p in paths {
            let path = p.unwrap().path();
            let path_string = path
                .to_str()
                .unwrap();

            let split_name: Vec<&str> = path_string
                .split("\\")
                .collect();

            let old_file_name = split_name[split_name.len() - 1];

            if old_file_name.starts_with(split_pattern[0]) && old_file_name.ends_with(split_pattern[1]) {
                remove_file(format!("{}\\{}", mod_group.location, old_file_name)).expect(format!("Could not delete old {} file", m.name).as_str());
                done = true;
                break
            }
        }

        if done == false {
            warning!("Did not find old {} file to delete.", m.name)
        }

        // save the new file
        let mut out = File::create(format!("{}\\{}", &mod_group.location, file_name))
            .expect(format!("Failed to create the new file for {}", m.name).as_str());

        io::copy(&mut &res[..], &mut out).expect(format!("Failed to write data to the new file for {}", m.name).as_str());

        success!("Updated {}", m.name);
    }

}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mods: MMUConfig = load_config();

    if args.len() == 2 {
        // search for given mod group

        let mod_group_result = search(&mods, args[1].clone());

        let mod_group = match mod_group_result {
            Ok(res) => res,
            Err(message) => {
                warning!("{}", message);
                return
            }
        };

        update(&mod_group)

    } else {
        fatal!("You did not provide a valid number of arguments!")
    }
}
