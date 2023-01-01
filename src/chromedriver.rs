use std::env;
use tokio::{self, fs, io::AsyncWriteExt};

pub struct ChromeDriver {
    chrome_version: String,
}

impl ChromeDriver {
    pub async fn new() -> ChromeDriver {
        let version = get_chrome_version();
        dbg!(&version);

        download_chromedriver(version).await;

        ChromeDriver {
            chrome_version: "temp".to_string(),
        }
    }
}

/// This function pulls the latest version of the Chrome Driver that is
/// required to start before we can start automaticating the browser
async fn download_chromedriver(version: String) {
    let major_version = version
        .split(".")
        .nth(0)
        .expect(" Could not Parse Major Version number of Chrome")
        .to_owned();

    // let url = r"https://chromedriver.storage.googleapis.com/index.html?path=108.0.";
    let url = r"https://chromedriver.storage.googleapis.com/?delimiter=/&prefix=108";

    let res = reqwest::get(url).await.unwrap().text().await.unwrap();

    let xml = roxmltree::Document::parse(&res).unwrap();

    for ele in xml.root().first_child().unwrap().children() {
        // dbg!(ele.tag_name().name());

        if ele.tag_name().name() == "CommonPrefixes" {
            for ele2 in ele.children().last() {
                dbg!(ele2.text().unwrap());
            }
        }
    }

    let specific_version_xml = xml
        .root()
        .first_child()
        .unwrap()
        .children()
        .filter(|p| p.tag_name().name() == "CommonPrefixes")
        .map(|f| {
            f.children()
                .last()
                .map(|f| f.text().unwrap_or("NONE").to_string())
        })
        .last()
        .unwrap()
        .unwrap();

    let specific_version = specific_version_xml.trim_end_matches("/");

    // dbg!(&specific_version);

    let mut file = fs::File::create("test.xml").await.unwrap();
    file.write_all(&res.as_bytes()).await.unwrap();

    let url2 = format!(
        r"https://chromedriver.storage.googleapis.com/{}/chromedriver_mac_arm64.zip",
        specific_version
    );
    let res = reqwest::get(url2).await.unwrap().bytes().await.unwrap();

    tokio::fs::create_dir_all("./chromedriver/").await.unwrap();

    let mut file = fs::File::create(format!("./chromedriver/{version}.zip")).await.unwrap();
    file.write_all(&res).await.unwrap();
}

fn get_chrome_version() -> String {
    let command = match env::consts::OS {
        "windows" => todo!("Add the Exacutable path here"),
        "linux" => todo!("Add the Exacutable path here"),
        "macos" => r"/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
        _ => panic!("Could not determine OS in use."),
    };

    let output = std::process::Command::new(command)
        .arg("--version")
        .output()
        .expect("Could not determine Chrome browser version. Please make sure its installed.");

    dbg!(&output);

    let full_text = String::from_utf8(output.stdout).unwrap();

    full_text
        .trim_start_matches("Google Chrome ")
        .trim_end_matches(" \n")
        .to_string()
}
