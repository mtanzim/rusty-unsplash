use std::{
    fs::File,
    io::{self, Cursor},
    path::Path,
};

use serde::{Deserialize, Serialize};

pub struct Downloader<'a> {
    path_prefix: &'a str,
    urls: Vec<&'a str>,
}

impl<'a> Downloader<'a> {
    pub fn new(path_prefix: &'a str, urls: Vec<&'a str>) -> Downloader<'a> {
        Downloader { path_prefix, urls }
    }

    fn download(&self, url: &str, i: usize) {
        let dl_bytes = reqwest::blocking::get(url)
            .ok()
            .and_then(|r| r.bytes().ok());
        match dl_bytes {
            Some(bytes) => {
                let filename = format!("{}/{i}.png", self.path_prefix);
                let path = Path::new(&filename);
                let display = path.display();

                let mut file = match File::create(&path) {
                    Err(why) => {
                        println!("couldn't create file {}: {}", display, why);
                        return;
                    }
                    Ok(file) => file,
                };

                let mut content = Cursor::new(bytes);
                match io::copy(&mut content, &mut file) {
                    Err(why) => {
                        println!("couldn't write to file {}: {}", display, why);
                        return;
                    }
                    Ok(_) => println!("Downloaded image {} to {}", url, display),
                }
            }
            _ => println!("No data downloaded"),
        }
    }

    pub fn download_all(&self) {
        for (i, url) in self.urls.iter().enumerate() {
            self.download(url, i)
        }
    }
}

#[derive(Debug)]
pub struct Unsplash<'a> {
    access_key: &'a str,
    base_api: &'a str,
}

impl<'a> Unsplash<'a> {
    pub fn new(access_key: &'a str, base_api: &'a str) -> Unsplash<'a> {
        Unsplash {
            access_key,
            base_api,
        }
    }

    fn extract_image_url(&self, api_url: &str) -> Option<Vec<String>> {
        let resp = reqwest::blocking::get(api_url).ok()?.text().ok()?;
        let deserialized: UnsplashResponse = serde_json::from_str(&resp).ok()?;
        Some(
            deserialized
                .iter()
                .map(|item| item.urls.full.to_string())
                .collect(),
        )
    }

    pub fn collect_urls(&self, collection_ids: &[&str], pages: usize) -> Vec<String> {
        let mut urls: Vec<String> = Vec::new();
        for collection_id in collection_ids {
            for page in 0..pages {
                let api_url = format!(
                    "{}/collections/{}/photos/?client_id={}&page={}",
                    self.base_api, collection_id, self.access_key, page
                );
                let image_urls = self.extract_image_url(api_url.as_str());
                let mut image_urls = match image_urls {
                    Some(urls) => urls,
                    None => continue,
                };
                urls.append(&mut image_urls);
            }
        }
        urls
    }
}

// used quicktype.io
pub type UnsplashResponse = Vec<WelcomeElement>;

#[derive(Debug, Serialize, Deserialize)]
pub struct WelcomeElement {
    id: String,
    created_at: String,
    updated_at: String,
    promoted_at: Option<String>,
    width: i64,
    height: i64,
    color: String,
    blur_hash: String,
    description: Option<String>,
    alt_description: Option<String>,
    urls: Urls,
    links: WelcomeLinks,
    likes: i64,
    liked_by_user: bool,
    current_user_collections: Vec<Option<serde_json::Value>>,
    sponsorship: Option<serde_json::Value>,
    topic_submissions: TopicSubmissions,
    user: User,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WelcomeLinks {
    #[serde(rename = "self")]
    links_self: String,
    html: String,
    download: String,
    download_location: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TopicSubmissions {
    nature: Option<Animals>,
    wallpapers: Option<Animals>,
    #[serde(rename = "arts-culture")]
    arts_culture: Option<Animals>,
    #[serde(rename = "color-theory")]
    color_theory: Option<Animals>,
    #[serde(rename = "textures-patterns")]
    textures_patterns: Option<Animals>,
    animals: Option<Animals>,
    people: Option<People>,
    #[serde(rename = "street-photography")]
    street_photography: Option<Animals>,
    #[serde(rename = "architecture-interior")]
    architecture_interior: Option<Animals>,
    architecture: Option<Animals>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Animals {
    status: Status,
    approved_on: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct People {
    status: Status,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Urls {
    raw: String,
    full: String,
    regular: String,
    small: String,
    thumb: String,
    small_s3: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    id: String,
    updated_at: String,
    username: String,
    name: String,
    first_name: String,
    last_name: String,
    twitter_username: Option<String>,
    portfolio_url: Option<String>,
    bio: Option<String>,
    location: Option<String>,
    links: UserLinks,
    profile_image: ProfileImage,
    instagram_username: Option<String>,
    total_collections: i64,
    total_likes: i64,
    total_photos: i64,
    accepted_tos: bool,
    for_hire: bool,
    social: Social,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserLinks {
    #[serde(rename = "self")]
    links_self: String,
    html: String,
    photos: String,
    likes: String,
    portfolio: String,
    following: String,
    followers: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProfileImage {
    small: String,
    medium: String,
    large: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Social {
    instagram_username: Option<String>,
    portfolio_url: Option<String>,
    twitter_username: Option<String>,
    paypal_email: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "approved")]
    Approved,
    #[serde(rename = "rejected")]
    Rejected,
}
