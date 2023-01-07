use std::time::Duration;

use terminal_color_builder::OutputFormatter as tcb;
use thirtyfour::prelude::*;

mod chromedriver;
mod cookies;
mod video;

use chromedriver::ChromeDriver;
use video::Video;

async fn preform_upload(webdriver: &mut WebDriver, video: Video) -> WebDriverResult<()> {
    webdriver.goto("https://www.youtube.com").await?;
    cookies::add_cookie(webdriver).await?;

    webdriver.goto("https://www.youtube.com/upload").await?;

    // Find hidden input text box and pass text into to specify the video path.
    webdriver
        .query(By::Css("input[type='file']"))
        .wait(Duration::from_secs_f32(15.0), Duration::from_secs_f32(0.50))
        .first()
        .await?
        .send_keys(&video.path)
        .await?;

    // Add video title to textbox
    webdriver
        .query(By::Css(
            "div[class='input-container title style-scope ytcp-video-metadata-editor-basics']",
        ))
        .wait(Duration::from_secs_f32(60.0), Duration::from_secs_f32(1.0))
        .first()
        .await?
        .find(By::Css("div[id='textbox']"))
        .await?
        .send_keys(&video.title)
        .await?;

    // Add video description to textbox
    webdriver
        .query(By::Css(
            "div[class='input-container description style-scope ytcp-video-metadata-editor-basics']"
        ))
        .wait(Duration::from_secs_f32(60.0), Duration::from_secs_f32(1.0))
        .first()
        .await?
        .find(By::Css("div[id='textbox']"))
        .await?
        .send_keys(&video.description)
        .await?;

    // Checking the `not` option for Kids button
    webdriver
        .query(By::Css("div[class='input-container style-scope ytcp-video-metadata-editor-basics']"))
        .wait(Duration::from_secs_f32(5.0), Duration::from_secs_f32(0.10))
        .first()
        .await?
        .query (By::XPath(r"//*[@id='audience']/ytkc-made-for-kids-select/div[4]/tp-yt-paper-radio-group/tp-yt-paper-radio-button[2]"))
        .wait(Duration::from_secs_f32(5.0), Duration::from_secs_f32(0.10))
        .first()
        .await?
        .click()
        .await?;

    // Click the show more button
    webdriver
        .query(By::Css(
            "div[class='toggle-section style-scope ytcp-video-metadata-editor']",
        ))
        .wait(Duration::from_secs_f32(5.0), Duration::from_secs_f32(0.10))
        .first()
        .await?
        .query(By::XPath(r"//*[@id='toggle-button']"))
        .wait(Duration::from_secs_f32(5.0), Duration::from_secs_f32(0.10))
        .first()
        .await?
        .click()
        .await?;

    // Adding Tags to the textbox.
    webdriver
        .query(By::XPath(r"/html/body/ytcp-uploads-dialog/tp-yt-paper-dialog/div/ytcp-animatable[1]/ytcp-ve/ytcp-video-metadata-editor/div/ytcp-video-metadata-editor-advanced/div[5]/ytcp-form-input-container/div[1]/div/ytcp-free-text-chip-bar/ytcp-chip-bar/div/input"))
        .wait(Duration::from_secs_f32(20.0), Duration::from_secs_f32(1.0))
        .first()
        .await?
        .send_keys(&video.get_tags_for_text_input())
        .await?;

    Ok(())
}

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let mut chromedriver = ChromeDriver::new().await;

    let caps = DesiredCapabilities::chrome();
    let mut webdriver = WebDriver::new("http://localhost:9515", caps).await?;

    let video = Video {
        title: "Test Title".to_string(),
        description: "Test description".to_string(),
        path: format!(
            "{}/{}",
            std::env::current_dir().unwrap().to_str().unwrap(),
            r"video.mp4" // Dont use hard coded path here
        ),
        madeforkids: "false".to_string(),
        tags: vec!["Minecraft".to_string(), "MC".to_string()],
    };

    let result = preform_upload(&mut webdriver, video).await;

    match result {
        Ok(_) => println!("Successfully Ran Process"),
        Err(e) => println!(
            "Process Failed! Error : {}",
            tcb::new().fg().red().text(e.to_string()).print()
        ),
    }

    // Always explicitly close the browser, then close the chrome driver.
    // webdriver.quit().await?;

    chromedriver.stop_driver().await;

    Ok(())
}
