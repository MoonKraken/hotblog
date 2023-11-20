use serde::Deserialize;
use serde::Serialize;
#[cfg(feature = "ssr")]
use sqlx::types::chrono::DateTime;
#[cfg(feature = "ssr")]
use sqlx::types::chrono::Utc;
#[cfg(feature = "ssr")]
use sqlx::FromRow;

#[cfg(feature = "hydrate")]
use chrono::DateTime;
#[cfg(feature = "hydrate")]
use chrono::Utc;


#[cfg_attr(feature = "ssr", derive(Serialize, Deserialize, Debug, Clone, FromRow))]
#[cfg_attr(feature = "hydrate", derive(Serialize, Deserialize, Debug, Clone))]
pub struct Post {
    pub id: String,
    pub dt: DateTime<Utc>,
    pub image_url: String,
    pub title: String,
    pub text: String,
}

impl Post {
    pub fn new_empty() -> Post {
        Post {
            id: "".to_string(),
            dt: Utc::now(),
            image_url: "".to_string(),
            title: "".to_string(),
            text: "".to_string(),
        }
    }
}
