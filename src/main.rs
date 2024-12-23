mod utils;

use std::env;
use std::fs;
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
    let mut res: ModGroup = ModGroup { name: "".to_string(), mods: vec![] };
    for mod_group in mods.mods.iter() {
        if mod_group.name == term {
            res = (*mod_group).clone();
            found = true;
        }
    }

    if found {return Ok(res)} else {return Err(format!("Could not find '{}'", term))}
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mods: MMUConfig = load_config();

    if args.len() == 2{
        // search for given mod group
        let mod_group_result = search(&mods, args[1].clone());

        let mod_group = match mod_group_result {
            Ok(res) => res,
            Err(message) => {
                warning!("{}", message);
                return
            }
        };

        println!("{:?}", mod_group)
    } else {
        fatal!("You did not provide a valid number of arguments!")
    }
}
