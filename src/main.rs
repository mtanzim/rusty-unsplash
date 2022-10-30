use dotenv::dotenv;
use std::{
    env,
    fs::File,
    io::{self, Write},
    path::Path,
};

use rusty_unsplash::Unsplash;

fn main() {
    dotenv().ok();
    let base_api: String = env::var("BASE_API").expect("Cannot read BASE_API");
    let access_key: String = env::var("ACCESS_KEY").expect("Cannot read ACCESS_KEY");

    let us = Unsplash::new(access_key.as_str(), base_api.as_str());
    println!("{:?}", us);

    println!("Please enter the collection id: ");
    let mut collection_id = String::new();
    io::stdin()
        .read_line(&mut collection_id)
        .expect("Failed to read line for collection id");
    let collection_id = collection_id.trim();
    let collection_ids = vec![collection_id];

    let mut num_pages = String::new();
    println!("Please enter the number of pages you would like to download (between 1 to 5): ");
    io::stdin()
        .read_line(&mut num_pages)
        .expect("Failed to read line for number of pages");
    let num_pages: usize = match num_pages.trim().parse() {
        Ok(v) if v >= 1 && v <= 5 => v,
        _ => {
            println!("Invalid number of pages!");
            return;
        }
    };
    let urls = us.collect_urls(&collection_ids, num_pages);
    let num_images = urls.len();
    println!("Found {} images", num_images);

    println!("How many would you like to download?");

    let mut num_downloads = String::new();
    io::stdin()
        .read_line(&mut num_downloads)
        .expect("Failed to read line for number of downloads");

    let num_downloads: usize = match num_downloads.trim().parse() {
        Ok(v) if v >= 1 && v <= num_images => v,
        _ => {
            println!("Invalid number of downloads!");
            return;
        }
    };

    let urls_to_download = urls.iter().take(num_downloads);
    for (i, url) in urls_to_download.enumerate() {
        println!("{}) {}\n", i + 1, url);
        let text = us.download_file(url);
        match text {
            Some(t) => {
                let filename = format!("downloads/{i}.png");
                let path = Path::new(&filename);
                let display = path.display();

                let mut file = match File::create(&path) {
                    Err(why) => {
                        println!("couldn't create file {}: {}", display, why);
                        continue;
                    }
                    Ok(file) => file,
                };
                match file.write_all(t.as_bytes()) {
                    Err(why) => {
                        println!("couldn't write to file {}: {}", display, why);
                        continue;
                    }
                    Ok(_) => println!("Downloaded image to {}", display),
                }
            }
            _ => println!("No data downloaded"),
        }
    }
}
