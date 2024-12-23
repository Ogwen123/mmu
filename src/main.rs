mod utils;

use std::env;
use std::fs;
use std::path::Path;
use std::ops::Deref;
use serde::{Deserialize, Serialize};
use crate::utils::logger::{fatal, info, warning};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Mod {
    name: String,
    download_link: String
}
#[derive(Serialize, Deserialize, Debug, Clone)]
struct ModGroup {
    name: String,
    mods: Vec<Mod>,
    location: String
}
#[derive(Serialize, Deserialize)]
struct MMUConfig {
    mods: Vec<ModGroup>
}

fn load_config() -> MMUConfig {
    let binding = fs::read_to_string("./mmu_config.json")
        .expect("Should have been able to read the file");
    let contents = binding.as_str();

    let data: MMUConfig = serde_json::from_str(contents).unwrap();

    data
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

    // loop through each mod in the list and delete the old mods and download the new one to replace it
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
