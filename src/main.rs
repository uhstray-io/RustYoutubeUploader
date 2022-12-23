use std::time::Duration;

use thirtyfour::cookie::SameSite;
use thirtyfour::prelude::*;
use tokio;

use serde::{Deserialize, Serialize};
use terminal_color_builder::OutputFormatter as tcb;

#[derive(Clone, Serialize, Deserialize)]
#[allow(non_snake_case)]
struct CookieConfig {
    domain: String,
    expirationDate: Option<f32>,
    hostOnly: bool,
    httpOnly: bool,
    name: String,
    path: String,
    sameSite: String,
    secure: bool,
    session: bool,
    storeId: String,
    value: String,
    id: i32,
}

impl CookieConfig {
    fn get_same_site(&self) -> SameSite {
        if self.sameSite == "lax" {
            return SameSite::None;
        }
        SameSite::None
    }
}

/// add_cookie imports cookies that you've exported from your browser and
/// applys them to the driver. Once you reloaded, you should be logged in.
async fn add_cookie(driver: &mut WebDriver) {
    let s = tokio::fs::read_to_string("src/cookies.txt").await.unwrap();

    let cookies: Vec<CookieConfig> = serde_json::from_str(&s).unwrap();

    for cookie in &cookies {
        let c = cookie.clone();

        let cookie_builder = Cookie::build(c.name, c.value)
            .path(c.path)
            .domain(c.domain)
            .same_site(cookie.clone().get_same_site())
            .finish();

        driver.add_cookie(cookie_builder).await.unwrap();
    }
}

async fn preform_search(driver: &mut WebDriver) -> WebDriverResult<()> {
    driver.goto("https://www.youtube.com").await?;

    //  let elem_form = driver.find(By::Id("search-form")).await?;

    add_cookie(driver).await;
    driver.refresh().await?;

    tokio::time::sleep(Duration::from_secs_f32(0.25)).await;

    let elem_button = driver.find(By::Css("input[type='submit']")).await?;
    elem_button.click().await?;

    // Look for header to implicitly wait for the page to load.
    // let stat = driver.find(By::Id("result-stats")).await?;

    // stat.screenshot(&PathBuf::from("stat.png")).await.unwrap();
    // let s = stat.text().await?;

    // println!("{s}");

    Ok(())
}

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let caps = DesiredCapabilities::chrome();
    let mut driver = WebDriver::new("http://localhost:9515", caps).await?;

    let result = preform_search(&mut driver).await;

    match result {
        Ok(_) => println!("Successfully Ran Process"),
        Err(e) => println!(
            "Process Failed! Error : {}",
            tcb::new().fg().red().text(e.to_string()).print()
        ),
    }

    // Always explicitly close the browser.
    //driver.quit().await?;

    Ok(())
}
