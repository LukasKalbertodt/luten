use maud::{Markup};

use template::Page;
use super::html;


#[get("/login")]
fn without_user() -> Markup {
    Page::empty()
        .with_title("Login")
        .with_content(html::content())
        .render()
}
