use std::env;
use std::io::Write;
use std::path::Path;
use std::process::{Child, Command};

use zip::ZipArchive;

pub struct ChromeDriver {
    child_process: Child,
}

impl ChromeDriver {
    pub async fn new() -> ChromeDriver {
        let chrome_version = get_chrome_version();
        let specific_version = get_specific_version(chrome_version).await;

        // If that version doesnt exists in the chrome driver folder, download it
        let path = Path::new("chromedriver").join(&specific_version);
        if !path.exists() {
            download_chromedriver(&specific_version).await;
        }

        // Start Chrome driver
        let child_process = start_driver(&specific_version).await;

        ChromeDriver { child_process }
    }

    pub async fn stop_driver(&mut self) {
        self.child_process.kill().expect("Cant kill driver process");
        // self.child_process.wait().expect("Waiting for responce from chromedriver"); // This may be required on other platforms
    }
}

async fn start_driver(specific_version: &str) -> Child {
    match env::consts::OS {
        "windows" => Command::new(format!(
            ".\\chromedriver\\{specific_version}\\chromedriver.exe"
        ))
        // .stdout(std::process::Stdio::null())
        // .stderr(std::process::Stdio::null())
        .spawn(),
        "linux" => todo!("Add the Chrome driver Exacutable path for Linux"),
        "macos" => todo!("Add the Chrome driver Exacutable path for MacOs"),
        _ => panic!("Could not determine OS in use."),
    }
    .expect("chromedriver command failed to start")
}

async fn get_specific_version(version: String) -> String {
    let major_version = version
        .split('.')
        .next()
        .expect(" Could not Parse Major Version number of Chrome")
        .to_owned();

    // Using the locally installed version of googlechrome, we can retreive a
    // list of version numbers that may suit us. Once we get the specific version
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

    // dbg!(&version_text);

    let xml = roxmltree::Document::parse(&version_text).unwrap();
    let specific_version_xml = xml
        .root()
        .first_child()
        .unwrap()
        .children()
        .filter(|p| p.tag_name().name() == "CommonPrefixes")
        .map(|f| f.children().last().map(|f| f.text().unwrap().to_string()))
        .last() // The Last tag should contain the latest version
        .unwrap()
        .unwrap();

    // let mut file = fs::File::create("test.xml").await.unwrap();
    // file.write_all(&res.as_bytes()).await.unwrap();

    specific_version_xml.trim_end_matches('/').to_string()
}

/// This function pulls the latest version of the Chrome Driver that is
/// required to start before we can start automaticating the browser
async fn download_chromedriver(specific_version: &str) {
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

    let folder = Path::new("chromedriver").join(specific_version);
    let zip_path = folder.clone().join("download.zip");

    tokio::fs::create_dir_all(&folder).await.unwrap();

    let mut file = std::fs::OpenOptions::new()
        .create_new(true)
        .read(true)
        .write(true)
        .open(&zip_path)
        .unwrap();

    file.write_all(&download_file).unwrap();

    ZipArchive::new(file).unwrap().extract(folder).unwrap();
}

fn get_chrome_version() -> String {
    match env::consts::OS {
        "windows" => {
            let output = Command::new("powershell" )
                .arg(r"(Get-Item 'C:\Program Files\Google\Chrome\Application\chrome.exe').VersionInfo.FileVersion")
                .output()
                .expect("Could not determine Chrome browser version. Please make sure its installed.");
            // dbg!(&output);

            String::from_utf8(output.stdout)
                .unwrap()
                .trim_end_matches("\r\n")
                .to_string()
        }

        "linux" => todo!("Add the Chrome browser Exacutable path for linux"),

        "macos" => {
            let output = Command::new(
                "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
            )
            .arg("--version")
            .output()
            .expect("Could not determine Chrome browser version. Please make sure its installed.");
            // dbg!(&output);

            String::from_utf8(output.stdout)
                .unwrap()
                .trim_start_matches("Google Chrome ")
                .trim_end_matches(" \n")
                .to_string()
        }
        _ => panic!("Could not determine OS in use."),
    }
}
