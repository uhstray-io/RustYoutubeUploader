use serde::{Deserialize, Serialize};
use thirtyfour::{cookie::SameSite, Cookie, WebDriver};
use tokio::fs;

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
    pub fn get_same_site(&self) -> SameSite {
        if self.sameSite == "lax" {
            return SameSite::None;
        }
        SameSite::None
    }
}

/// add_cookie imports cookies that you've exported from your browser and
/// applys them to the driver. Once you reloaded, you should be logged in.
pub async fn add_cookie(driver: &mut WebDriver) {
    let s = fs::read_to_string("configs/cookies.json").await.unwrap();

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
