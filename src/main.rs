use std::time::Duration;

use terminal_color_builder::OutputFormatter as tcb;
use thirtyfour::prelude::*;

mod cookies;
mod video;

use video::Video;

async fn preform_upload(driver: &mut WebDriver, video: Video) -> WebDriverResult<()> {
    driver.goto("https://www.youtube.com").await?;

    // let elem_form = driver.find(By::Id("search-form")).await?;

    cookies::add_cookie(driver).await; // add_cookie(driver).await;
                                       // driver.refresh().await?;

    // tokio::time::sleep(Duration::from_secs_f32(0.25)).await;

    //let elem_button = driver.find(By::Css("input[type='submit']")).await?;
    //elem_button.click().await?;
    driver.goto("https://www.youtube.com/upload").await?;

    //let elem_button = driver.find(By::Css("button[id='button']")).await?;
    //elem_button.click().await?;

    tokio::time::sleep(Duration::from_secs_f32(0.25)).await;

    // Find element from element.
    driver
        .query(By::Css("input[type='file']"))
        .wait(Duration::from_secs_f32(15.0), Duration::from_secs_f32(0.50))
        .first()
        .await?
        .send_keys(video.path)
        .await?;

    // This waits until the title is displayed
    driver
        .query(By::Css(
            "div[class='input-container title style-scope ytcp-video-metadata-editor-basics']",
        ))
        .wait(Duration::from_secs_f32(60.0), Duration::from_secs_f32(1.0))
        .first()
        .await?
        .find(By::Css("div[id='textbox']"))
        .await?
        .send_keys(video.title)
        .await?;

    // This waits until the title is displayed
    driver
        .query(By::Css(
            "div[class='input-container description style-scope ytcp-video-metadata-editor-basics']"
        ))
        .wait(Duration::from_secs_f32(60.0), Duration::from_secs_f32(1.0))
        .first()
        .await?
        .find(By::Css("div[id='textbox']"))
        .await?
        .send_keys(video.description)
        .await?;




    // Checking the not for Kids button 
    driver
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


    //click the show more button    
    driver
        .query(By::Css("div[class='toggle-section style-scope ytcp-video-metadata-editor']"))
        .wait(Duration::from_secs_f32(5.0), Duration::from_secs_f32(0.10))
        .first()
        .await?
        .query (By::XPath(r"//*[@id='toggle-button']"))
        .wait(Duration::from_secs_f32(5.0), Duration::from_secs_f32(0.10))
        .first()
        .await? 
        .click()
        .await?;


    //Adding Tags this is not working 
    //driver
    //    .query(By::Css("div[class='chip-and-bar style-scope ytcp-chip-bar']"))
    //    .wait(Duration::from_secs_f32(20.0), Duration::from_secs_f32(1.0))
    //   .first()
    //    .await?
    //    .find(By::XPath(r"//*[@id='text-input']"))
    //    .await?
    //    .click()
    //    //.send_keys(video.tag)
    //    .await?;


    // stat.screenshot(&PathBuf::from("stat.png")).await.unwrap();
    // let s = stat.text().await?;

    // println!("{s}");

    Ok(())
}

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let caps = DesiredCapabilities::chrome();
    let mut driver = WebDriver::new("http://localhost:9515", caps).await?;

    let video = Video {
        title: "Test Title".to_string(),
        description: "Test description".to_string(),
        path: r"C:\Users\godle\OneDrive\Desktop\Movie Edit\Clips\clip1.mp4"
            .to_string(),
        madeforkids: "false".to_string(),
        tag: "Minecraft" .to_string()
    };

    let result = preform_upload(&mut driver, video).await;

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
