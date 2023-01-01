use std::os::unix::fs::PermissionsExt;
use std::{env, io};
use tokio::{self, fs, io::AsyncWriteExt};
use zip::{self, ZipArchive};

pub struct ChromeDriver {}

impl ChromeDriver {
    pub async fn new() -> ChromeDriver {
        let version = get_chrome_version();
        // dbg!(&version);

        // If that version doesnt exists in the chrome driver folder,
        // download it


        download_chromedriver(version).await;

        ChromeDriver {}
    }
}

/// This function pulls the latest version of the Chrome Driver that is
/// required to start before we can start automaticating the browser
async fn download_chromedriver(version: String) {
    let major_version = version
        .split('.')
        .next()
        .expect(" Could not Parse Major Version number of Chrome")
        .to_owned();

    // Using the locally installed version of googlechrome, we can retreive a
    //list of version numbers that may suit us. Onece we get the specific version
    // we can download it directly.
    let versions_url = format!(
        "https://chromedriver.storage.googleapis.com/?delimiter=/&prefix={}",
        major_version
    );
    let version_text = reqwest::get(versions_url)
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let xml = roxmltree::Document::parse(&version_text).unwrap();
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
        .last() // The Last tag should contain the latest version
        .unwrap()
        .unwrap();

    let specific_version = specific_version_xml.trim_end_matches('/');

    // let mut file = fs::File::create("test.xml").await.unwrap();
    // file.write_all(&res.as_bytes()).await.unwrap();

    let os_version = match env::consts::OS {
        "windows" => "chromedriver_win32",
        "linux" => "chromedriver_linux64",
        "macos" => "chromedriver_mac_arm64", // We may want to account for intel based macs in the future
        _ => panic!("Could not determine OS in use."),
    };

    let download_url = format!(
        "https://chromedriver.storage.googleapis.com/{}/{}.zip",
        specific_version, os_version
    );

    let download_file = reqwest::get(download_url)
        .await
        .unwrap()
        .bytes()
        .await
        .unwrap();

    tokio::fs::create_dir_all("./chromedriver/").await.unwrap();

    let zip_filename = format!("./chromedriver/{version}.zip");
    let mut file = fs::File::create(&zip_filename).await.unwrap();
    file.write_all(&download_file).await.unwrap();

    unzip_file(&zip_filename, "./chromedriver", &version);
}

fn unzip_file(zip_path: &str, dest_dir: &str, dest_name: &str) {
    let file = std::fs::File::open(zip_path).unwrap();
    let mut zip = ZipArchive::new(file).unwrap();

    for i in 0..zip.len() {
        let mut file = zip.by_index(i).unwrap();
        let outpath = format!("{}/{}", dest_dir, dest_name);

        // std::fs::create_dir_all(&outpath).unwrap();
        let mut outfile = std::fs::File::create(&outpath).unwrap();
        io::copy(&mut file, &mut outfile).unwrap();

        // Get and Set permissions
        #[cfg(unix)]
        {
            if let Some(mode) = file.unix_mode() {
                std::fs::set_permissions(&outpath, std::fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }
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

    // dbg!(&output);

    let full_text = String::from_utf8(output.stdout).unwrap();

    full_text
        .trim_start_matches("Google Chrome ")
        .trim_end_matches(" \n")
        .to_string()
}
