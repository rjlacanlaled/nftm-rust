use actix_web::{ HttpRequest, Result as ActixResult, get };
use maud::{ Markup, html };

use crate::pages::index::index;

#[get("/")]
pub async fn page(req: HttpRequest) -> ActixResult<Markup> {
    let title = "Home";
    let desc = "NFT Marketplace Home";

    let content = html! {
            h1 {"Home"}
        };

    Ok(index(req, content, title, desc).await)
}
