use actix_web::{ HttpRequest, Result as ActixResult, get };
use maud::{ Markup, html };

use crate::pages::index::index;

#[get("/collections")]
pub async fn page(req: HttpRequest) -> ActixResult<Markup> {
    let title = "NFT Collections";
    let desc = "NFT Collections";

    let content = html! {
            h1 {"NFT Collections"}
        };

    Ok(index(req, content, title, desc).await)
}
