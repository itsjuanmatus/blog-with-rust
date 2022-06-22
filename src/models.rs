use serde::{Deserialize, Serialize};

use super::schema::posts;

#[derive(Queryable, Debug, Deserialize, Serialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub body: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct NewPostHandler {
    pub title: String,
    pub body: String,
}

use diesel::prelude::*;

#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
    pub slug: &'a str,
}

impl Post {
    pub fn slugify(title: &String) -> String {
        return title.replace(" ", "-").to_lowercase();
    }

    pub fn create_post<'a>(
        conn: &PgConnection,
        post: &NewPostHandler,
    ) -> Result<Post, diesel::result::Error> {
        let slug = Post::slugify(&post.title.clone());

        let new_post = NewPost {
            title: &post.title,
            slug: &slug,
            body: &post.body,
        };

        diesel::insert_into(posts::table)
            .values(new_post)
            .get_result::<Post>(conn)
    }
}
