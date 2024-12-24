mod utils;

use std::{env, format};
use regex::Regex;
use reqwest;
use std::fs;
use std::path::Path;
use reqwest::header::USER_AGENT;
use serde_json::{from_str};
use crate::utils::logger::{fatal, warning};
use crate::utils::types::{MMUConfig, ModGroup, Mod, APIResult, ReleaseData};

fn load_config() -> MMUConfig {
    let binding = fs::read_to_string("./mmu_config.json")
        .expect("Should have been able to read the file");
    let contents = binding.as_str();

    let data: MMUConfig = serde_json::from_str(contents).unwrap();

    data
}

fn build_github_url(url: String) -> String {
    // check the link is the correct format
    let re = Regex::new(r"https://github.com/[a-zA-Z]+/[a-zA-Z]+").unwrap();
    let is_valid = re.find(&url).unwrap();

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

fn install(mod_group: &ModGroup) {

}

fn update(mod_group: &ModGroup) {
    println!("{:?}", mod_group);
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

        println!("{:?}", release_data.browser_download_url);

        break
    }

}

fn main() {
    // consts
    const VALID_PRIMARY_ARGS: [&str; 2] = ["update", "install"];

    let args: Vec<String> = env::args().collect();

    let mods: MMUConfig = load_config();

    if args.len() == 3 {
        // search for given mod group

        if !VALID_PRIMARY_ARGS.contains(&&*args[1]) {
            fatal!("Invalid arguments.");
            return
        }

        let mod_group_result = search(&mods, args[2].clone());

        let mod_group = match mod_group_result {
            Ok(res) => res,
            Err(message) => {
                warning!("{}", message);
                return
            }
        };

        match args[1].as_str() {
            "install" => install(&mod_group),
            "update" => update(&mod_group),
            _ => {
                fatal!("Invalid arguments, THIS SHOULD HAVE BEEN CAUGHT EARLIER.")
            }
        }

    } else {
        fatal!("You did not provide a valid number of arguments!")
    }
}
