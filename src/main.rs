use std::io;
use std::path::PathBuf;

// use termion::{color, style};
use thirtyfour::prelude::*;
use tokio;

use terminal_color_builder::OutputFormatter as tcb;

async fn preform_search(driver: &WebDriver) -> WebDriverResult<()> {
    // let  RF =  color::Fg(color::Red);
    // driver.goto("https://wikipedia.org").await?;
    //driver.goto("https://youtube.com").await?;
    driver.goto("https://google.com").await?;
    //  let elem_form = driver.find(By::Id("search-form")).await?;

    driver
        .screenshot(&PathBuf::from("driver.png"))
        .await
        .unwrap();

    // Find element from element.
    let elem_text = driver.find(By::Css("input[title='Search']")).await?;

    // Type in the search terms.
    elem_text.send_keys("selenium").await?;

    elem_text
        .screenshot(&PathBuf::from("input.png"))
        .await
        .unwrap();

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
    let driver = WebDriver::new("http://localhost:9515", caps).await?;

    let result = preform_search(&driver).await;

    match result {
        Ok(_) => println!("Successfully Ran Process"),
        Err(e) => println!(
            "Process Failed! Error : {}",
            tcb::new().fg().red().text(e.to_string()).print()
        ),
    }

    // Always explicitly close the browser.
    driver.quit().await?;

    Ok(())
}
