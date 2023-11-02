use actix_web::{ HttpRequest, Result as ActixResult, get };
use maud::{ Markup, html };

use crate::{ pages::index::index, components::collection_card::collection_card };

#[get("/collections")]
pub async fn page(req: HttpRequest) -> ActixResult<Markup> {
    let title = "NFT Collections";
    let desc = "NFT Collections";

    let content =
        html! {
            div class = "container flex flex-col w-full h-full gap-10 px-4 py-5 bg-red-300" {
                h1 {
                    div class = "flex gap-4" {
                        span {"Explore"}
                        span {"All Collections"}
                    }
                }

                div class = "flex gap-4" {
                    input type = "text" placeholder = "Search Collections" {}
                    div class = "flex" {
                        img src="filter.jpg"
                        label {"Filter"}
                    }
                }

                div class ="grid w-full h-full grid-cols-2 gap-4 p-2 mx-auto bg-blue-300 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5" {
                    // Collection Cards

                    // TODO: Replace with actual collection cards
                    (collection_card())
                    (collection_card())
                    (collection_card())
                    (collection_card())
                    (collection_card())
                    (collection_card())
                    (collection_card())
                    (collection_card())
                    (collection_card())
                    (collection_card())
                    (collection_card())
                    (collection_card())
                    (collection_card())
                    (collection_card())
                    (collection_card())
                    (collection_card())
                    (collection_card())
                    (collection_card())
                    (collection_card())
                }
            }
        };

    Ok(index(req, content, title, desc).await)
}
