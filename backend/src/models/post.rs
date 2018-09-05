use chrono::{NaiveDateTime};
use chrono::prelude::Utc;
use models::user::User;

table! {
    posts (id) {
        id -> Int4,
        creator -> Int4,
        created -> Timestamp,
        title -> Text,
        body -> Text,
    }
}

#[derive(Serialize, Deserialize, Insertable, Associations)]
#[belongs_to(User, foreign_key="creator")]
#[table_name = "posts"]
pub struct PostForm {
    pub creator: i32,
    pub created: NaiveDateTime,
    pub title: String,
    pub body: String,
}
//it's pretty ridiculous you need to have separate models for inserts
#[derive(Serialize, Deserialize, Queryable, Associations)]
#[belongs_to(User, foreign_key="creator")]
pub struct Post {
    pub id: i32,
    pub creator: i32,
    pub created: NaiveDateTime,
    pub title: String,
    pub body: String,
}

