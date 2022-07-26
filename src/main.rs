use std::io::Cursor;
use image::{ImageBuffer, Rgba};

#[derive(serde::Deserialize, Debug)]
pub struct Urls {
    pub raw: String,
    pub full: String,
    pub regular: String,
    pub small: String,
    pub thumb: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct ProfileImage {
    pub small: String,
    pub medium: String,
    pub large: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct User {
    pub id: String,
    pub username: String,
    pub name: String,
    pub first_name: String,
    pub last_name: String,
    pub instagram_username: Option<String>,
    pub twitter_username: Option<String>,
    pub portfolio_url: Option<String>,
    pub profile_image: ProfileImage,
    pub links: Links,
}

#[derive(serde::Deserialize, Debug)]
pub struct Links {
    #[serde(rename = "self")]
    pub this: String,

    pub html: String,
    pub download: Option<String>,
    pub photos: Option<String>,
    pub likes: Option<String>,
}

#[derive(serde::Deserialize, Debug)]
pub struct Photo {
    pub id: String,
    pub title: Option<String>,

    pub created_at: String,
    pub updated_at: Option<String>,
    pub promoted_at: Option<String>,

    pub width: u32,
    pub height: u32,

    pub color: String,
    pub blur_hash: String,

    pub description: Option<String>,
    pub alt_description: Option<String>,

    pub cover_photo: Option<String>,

    pub urls: Urls,
}

#[derive(serde::Deserialize, Debug)]
pub struct Response {
    pub total: u32,
    pub total_pages: u32,
    pub results: Vec<Photo>,
}

async fn write_to_file() {
    let base = "https://api.unsplash.com/";
    let query = "cat";
    let key = std::env::var("ACCESS_KEY").unwrap();

    let url = format!("{base}/search/photos?query={query}");
    println!("{url}");

    let text = reqwest::Client::new()
        .get(url)
        .header("Authorization", format!("Client-ID {}", key))
        .send()
        .await.unwrap().text().await.unwrap();

    std::fs::write("cat.json", text).unwrap();
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // write_to_file().await;

    let text = std::fs::read_to_string("cat.json").unwrap();
    let response: Response = serde_json::from_str(&text).unwrap();

    let bytes = blurhash_wasm::decode(&response.results[0].blur_hash, 50, 50).unwrap();
    let buffer: ImageBuffer<Rgba<u8>, _> = ImageBuffer::from_vec(50, 50, bytes).unwrap();
    buffer.save("cat.png").unwrap();

    println!("{response:#?}");
    Ok(())
}
