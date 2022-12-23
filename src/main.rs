use std::io;
use std::path::PathBuf;
use std::time::Duration;

use thirtyfour::cookie::SameSite;
// use termion::{color, style};
use thirtyfour::prelude::*;
use tokio;

use terminal_color_builder::OutputFormatter as tcb;
use serde::{Deserialize, Serialize};
#[derive(Clone, Serialize, Deserialize)]

struct MyCookie{
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

impl MyCookie {
    fn get_same_Site(&self) -> SameSite{
        if self.sameSite == "lax"{
            return SameSite::None;
        }
        SameSite::None
        
    }
}



async fn add_cookie(driver: &mut WebDriver){

    let s = tokio::fs::read_to_string("src/cookies.txt").await.unwrap();

    let cookies: Vec<MyCookie> = serde_json::from_str(&s).unwrap();

    for cookie in cookies{
        let cookie_builder =Cookie::build(cookie.clone().name, cookie.clone().value)
        .path(cookie.clone().path)
        .domain(cookie.clone().domain)
        .same_site(cookie.clone().get_same_Site())
        .finish();

        driver.add_cookie(cookie_builder).await.unwrap();
    }
}







async fn preform_search(driver: &mut WebDriver) -> WebDriverResult<()> {
    // let  RF =  color::Fg(color::Red);
    // driver.goto("https://wikipedia.org").await?;
    //driver.goto("https://youtube.com").await?;
    //driver.goto("https://google.com").await?;
    driver.goto("https://www.youtube.com").await?;







    //  let elem_form = driver.find(By::Id("search-form")).await?;

    //let login_cookie = Cookie::new("LOGIN_INFO","AFmmF2swRgIhAITX-pnUZC8IpM6jemKzGOSyqwta4kQQbez4YFbVJIOxAiEAwERX6fmM4gv5XjxRwXIwGPncTxz0lOLpn9Hy9Drio7A:QUQ3MjNmek9zZWdMOFRlOHFSYzUwWDdoX0F5TjRFa0loLVUxaVBGM3ZieEQyc3FTUlg0WGtscHdfb1lwWjAxekNRNEFEZTJMa014TlI4MjBKTDZmdUNsWnNzak1QaHJrMmh1ZVFSTG5raV9EeFpmd1pWRU1LVTVFSTVXMndiOVRvMUc4Mm5RbXhSbDVPY0w2aXk2NlhYTXBhZHhKa2RaRDJyOUdMRjlDdDYweFdZVjhlOG9jNW1scWlaVkVWbTFPWkZPWkNjV2FmSEd1Uk9ORHVDWDVGWEVCTzZUZlU3a0E0dw==");
    //driver.add_cookie(login_cookie).await.unwrap();

    //let cookie = Cookie::build("LOGIN_INFO", "AFmmF2swRgIhAITX-pnUZC8IpM6jemKzGOSyqwta4kQQbez4YFbVJIOxAiEAwERX6fmM4gv5XjxRwXIwGPncTxz0lOLpn9Hy9Drio7A:QUQ3MjNmek9zZWdMOFRlOHFSYzUwWDdoX0F5TjRFa0loLVUxaVBGM3ZieEQyc3FTUlg0WGtscHdfb1lwWjAxekNRNEFEZTJMa014TlI4MjBKTDZmdUNsWnNzak1QaHJrMmh1ZVFSTG5raV9EeFpmd1pWRU1LVTVFSTVXMndiOVRvMUc4Mm5RbXhSbDVPY0w2aXk2NlhYTXBhZHhKa2RaRDJyOUdMRjlDdDYweFdZVjhlOG9jNW1scWlaVkVWbTFPWkZPWkNjV2FmSEd1Uk9ORHVDWDVGWEVCTzZUZlU3a0E0dw==")
    //.domain("www.youtube.com").path("/").same_site(SameSite::None).finish();
    //driver.add_cookie(cookie).await.unwrap();
    add_cookie(driver).await;
    driver.refresh().await?;

    //driver
    //    .screenshot(&PathBuf::from("driver.png"))
    //    .await
    //    .unwrap();

    // Find element from element.
    //let elem_text = driver.find(By::Css("input[title='Search']")).await?;
    // let publish_button = driver
    //    .find(By::Css("circular style-scope yt-icon-button"))
    //    .await?;
    //publish_button.click().await?;

    tokio::time::sleep(Duration::from_secs_f32(0.25)).await;
    // Type in the search terms.
    //elem_text.send_keys("selenium").await?;

    //elem_text
    //    .screenshot(&PathBuf::from("input.png"))
    //    .await
    //    .unwrap();

    let elem_button = driver.find(By::Css("input[type='submit']")).await?;
    elem_button.click().await?;

    // Look for header to implicitly wait for the page to load.
    let stat = driver.find(By::Id("result-stats")).await?;

    stat.screenshot(&PathBuf::from("stat.png")).await.unwrap();
    let s = stat.text().await?;

    println!("{s}");

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
