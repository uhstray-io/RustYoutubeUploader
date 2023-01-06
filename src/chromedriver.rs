use std::env;
use std::io;
use std::path::Path;
use std::process::{Child, Command};

use tokio::{self, fs, io::AsyncWriteExt};
use zip::{self, ZipArchive};

pub struct ChromeDriver {
    child_process: Child,
}

impl ChromeDriver {
    pub async fn new() -> ChromeDriver {
        let chrome_version = get_chrome_version();
        let specific_version = get_specific_version(chrome_version).await;

        // If that version doesnt exists in the chrome driver folder, download it
        let path = format!("/chromedriver/{specific_version}.zip");
        if !std::path::Path::exists(Path::new(path.as_str())) {
            download_chromedriver(&specific_version).await;
        }

        //start driver
        let child_process = start_driver(&specific_version).await;

        ChromeDriver { child_process }
    }

    pub async fn stop_driver(&mut self) {
        dbg!(self.child_process.id());
        self.child_process.kill().expect("Cant kill driver process");
        // self.child_process.wait().expect("Waiting for responce from chromedriver");
    }
}

async fn start_driver(specific_version: &str) -> Child {
    let a = match env::consts::OS {
        "windows" => Command::new(format!(r".\chromedriver\{}.exe", specific_version))
            // .stdout(std::process::Stdio::null())
            // .stderr(std::process::Stdio::null())
            .spawn(),
        "linux" => todo!("Add the Chrome driver Exacutable path for Linux"),
        "macos" => todo!("Add the Chrome driver Exacutable path for MacOs"),
        _ => panic!("Could not determine OS in use."),
    }
    .expect("chromedriver command failed to start");

    dbg!(a.id());

    a
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

    tokio::fs::create_dir_all("./chromedriver/").await.unwrap();

    let zip_filename = format!("./chromedriver/{specific_version}.zip");
    let mut file = fs::File::create(&zip_filename).await.unwrap();
    file.write_all(&download_file).await.unwrap();

    unzip_file(&zip_filename, "./chromedriver", specific_version);
}

fn unzip_file(zip_path: &str, dest_dir: &str, dest_name: &str) {
    let file = std::fs::File::open(zip_path).unwrap();
    let mut zip = ZipArchive::new(file).unwrap();

    for i in 0..zip.len() {
        let mut file = zip.by_index(i).unwrap();

        let extension = match env::consts::OS {
            "windows" => ".exe",
            _ => "",
        }
        .to_string();

        let outpath = format!("{dest_dir}/{dest_name}{extension}");

        // std::fs::create_dir_all(&outpath).unwrap();
        let mut outfile = std::fs::File::create(&outpath).unwrap();
        io::copy(&mut file, &mut outfile).unwrap();

        // Get and Set permissions
        #[cfg(unix)]
        {
            // std::os::unix::fs::PermissionsExt;
            if let Some(mode) = file.unix_mode() {
                std::fs::set_permissions(&outpath, std::fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }
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
