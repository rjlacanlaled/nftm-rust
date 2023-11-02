use actix_web::HttpRequest;
use maud::{ Markup, html };

use crate::components::navbar::navbar;

use super::base::page;

pub async fn index(req: HttpRequest, child_content: Markup, title: &str, desc: &str) -> Markup {
    let host = format!("{}", req.uri());
    let lang = "en";
    // TODO: Add your site or application content here.
    let content =
        html! {
        #content {
            (navbar())
            (child_content)

            // TODO: Add Footer Here
        }
    };
    page(&host, title, desc, lang, content)
}
