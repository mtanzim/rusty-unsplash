use serde::{Deserialize, Serialize};

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
    pub fn collect_urls(
        &self,
        collection_ids: &[&str],
        pages: u32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let urls: Vec<String> = collection_ids
            .iter()
            .flat_map(|collection_id| {
                let urls_per_page: Vec<String> = (1..=pages)
                    .flat_map(|page| {
                        let api_url = format!(
                            "{}/collections/{}/photos/?client_id={}&page={}",
                            self.base_api, collection_id, self.access_key, page
                        );
                        let resp = reqwest::blocking::get(api_url)
                            .expect("failed to get response")
                            .text()
                            .expect("failed to parse response to text");
                        let deserialized: UnsplashResponse = serde_json::from_str(&resp)
                            .expect("failed to deserialize resonse to JSON");

                        let urls_per_collection: Vec<String> = deserialized
                            .iter()
                            .map(|item| item.urls.full.to_string())
                            .collect();
                        urls_per_collection
                    })
                    .collect();
                urls_per_page
            })
            .collect();
        println!("{:?}", urls);
        Ok(())
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