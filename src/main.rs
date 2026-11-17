use std::fs::File;
use std::env;

fn get_page_body(url: &str) -> String {
    let resp = reqwest::blocking::get(url).expect("req failed!");
    let body = resp.text().expect("body invalid!");
    return body;
}
fn find_image_links(body: &str) -> Vec<String> {
    let regex = regex::Regex::new(
        r"https://science\.nasa\.gov/image-article/([A-Za-z0-9]+(-[A-Za-z0-9]+)+)/"
        ).expect("regex wasnt build!");
    return regex.find_iter(body).map(|m| m.as_str().to_owned()).collect();
}

fn find_image_source(body: &str) -> Option<String> {
    let regex = regex::Regex::new(
        r"assets\.science\.nasa\.gov/content/dam/science/(?:[A-Za-z0-9_-]+/)*[A-Za-z0-9_%.-]+\.[A-Za-z]{1,3}"
        ).expect("regex wasnt build!");
    return regex.find(body).map(|m| m.as_str().to_owned());
}

fn save_image(url: &str, file_name: &str, folder_path: &str) -> File {
    std::fs::create_dir_all(folder_path).expect("couldnt create folder!");
    let mut image = std::fs::File::create(folder_path.to_owned() + file_name).unwrap();
    reqwest::blocking::get(url).unwrap().copy_to(&mut image).expect("couldnt download image!");
    return image;
}

fn get_image_name(url: &str) -> String {
    let binding_str = String::from(url);
    let url_parts: Vec<&str> = binding_str.split("/").collect();

    let name: &str = url_parts[url_parts.len()-1];
    return name.to_string().replace("%",""); // for some odd reason, there are % ocassionaly in file
                                             // names (or my parsing for names is just gargabe..)
}

fn get_month() -> String {
    return chrono::Utc::now().format("%B").to_string();
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut folder_path: String = "./images/".to_string() + &get_month() +"/";

    if args.len() > 1 {
        let mut path: String = args[2].to_string();
        if !args[2].ends_with("/") {
            path = path + "/";
        }
        folder_path = path + &get_month() +"/";
        println!("[System]: Using user-specified path: '{folder_path}' !")
    }else{
        println!("[System]: Using default path: '{folder_path}'.");
        println!("[Info]: Use 'nasa-wallpaper-dl -p /path_to_my_folder/here/'");
    }

    let mut page_body: String = get_page_body("https://science.nasa.gov/apod/apod-calendar/");
    let image_urls: Vec<String> = find_image_links(&page_body);

    for url in &image_urls {
        page_body = get_page_body(&url);
        match find_image_source(&page_body) {
            Some(source) => {
                let file_name: String = get_image_name(&source);
                println!("{file_name}");
                save_image(&("https://".to_owned() + &source), &file_name, &folder_path);
            },
            None => eprintln!("no picture-url found at {url} !"),
        }
    }
}
