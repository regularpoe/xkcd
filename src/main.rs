#[macro_use]
extern crate clap;
use clap::App;

extern crate image;

use serde::{Deserialize, Serialize};

use std::fs::File;
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
struct Response {
    month: String,
    num: i32,
    link: String,
    year: String,
    news: String,
    safe_title: String,
    transcript: String,
    alt: String,
    img: String,
    title: String,
    day: String,
}

impl Response {
    fn title(&self) -> String {
        self.title.to_string()
    }

    fn number(&self) -> String {
        self.num.to_string()
    }

    fn date(&self) -> String {
        format!("{}/{}/{}", self.day, self.month, self.year)
    }

    fn description(&self) -> String {
        self.alt.to_string()
    }

    fn image(&self) -> String {
        self.img.to_string()
    }
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let yaml = load_yaml!("cli.yaml");
    let opts = App::from_yaml(yaml).get_matches();

    let n = opts.value_of("number").unwrap().parse::<i32>().unwrap();
    let output = opts.value_of("output").unwrap();
    let s = opts.is_present("save");

    let url = match n {
        0 => String::from("https://xkcd.com/info.0.json"),
        _ => format!("https://xkcd.com/{}/info.0.json", n),
    };

    let res = reqwest::get(&url).await?;
    let body = res.text().await?;

    let rsp: Response = match serde_json::from_str(&body) {
        Ok(data) => data,
        Err(why) => panic!("Error: {}", why),
    };

    let text_output = format!(
        "\nTitle: {}\nComic No: {}\nDate: {}\nDescription: {}\nImage: {}",
        rsp.title(),
        rsp.number(),
        rsp.date(),
        rsp.description(),
        rsp.image()
    );

    match output {
        "text" => {
            println!("{}", text_output);
        }
        "json" => {
            println!("{}", body);
        }
        _ => {}
    }

    match s {
        true => {
            let res = reqwest::get(&rsp.image()).await?;
            let body = res.bytes().await?;
            let img = image::load_from_memory(&body).unwrap();

            let fh = &mut File::create(Path::new(&format!("{}_{}.png", rsp.title(), rsp.number())))
                .unwrap();

            match img.write_to(fh, image::ImageFormat::Png) {
                Ok(_) => {}
                Err(why) => panic!("Cannot write to image: {}", why),
            }
        }
        false => {}
    }

    Ok(())
}
