use serde::{Deserialize, Serialize};

pub struct ParsedPath {
    pub path: String,
    pub file_name: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Mod {
    pub name: String,
    pub pattern: String,
    pub download_link: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ModGroup {
    pub name: String,
    pub mods: Vec<Mod>,
    pub location: String
}

#[derive(Serialize, Deserialize)]
pub struct MMUConfig {
    pub mods: Vec<ModGroup>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ReleaseData {
    pub url: String,
    pub id: usize,
    pub node_id: String,
    pub name: String,
    #[serde(skip)]
    pub label: String,
    #[serde(skip)]
    pub uploader: String,
    pub content_type: String,
    pub state: String,
    pub size: usize,
    pub download_count: usize,
    pub created_at: String,
    pub updated_at: String,
    pub browser_download_url: String
}

#[derive(Deserialize, Serialize)]
pub struct APIResult {
    pub url: String,
    pub assets_url: String,
    pub upload_url: String,
    pub html_url: String,
    pub id: usize,

    #[serde(skip)]
    pub author: String,

    pub node_id: String,
    pub tag_name: String,
    pub target_commitish: String,
    pub name: String,
    pub draft: bool,
    pub prerelease: bool,
    pub created_at: String,
    pub published_at: String,
    pub assets: Vec<ReleaseData>,
    pub tarball_url: String,
    pub zipball_url: String,
    pub body: String
}

