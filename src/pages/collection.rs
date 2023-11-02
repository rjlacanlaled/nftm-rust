use actix_web::{ HttpRequest, Result as ActixResult, get, web };
use maud::{ Markup, html };

use crate::{
    pages::{ index::index, dummy_data::get_dummy_collections },
    components::collection_card::{ collection_card, self },
};

pub struct Collection {
    pub id: u32,
    pub img_url: String,
    pub name: String,
    pub volume: f64,
}

#[get("/collection/{id}")]
pub async fn page(req: HttpRequest, path: web::Path<(u32,)>) -> ActixResult<Markup> {
    // TODO: Fetch collection data from database

    // TODO: Update metadata to match fetched data
    let title = path.0.to_string();
    let desc = "NFT Collections";

    let content =
        html! {
            div class = "flex flex-col w-full h-full gap-10 py-5 bg-green-400" {
                div class="relative" {
                    img src="https://placekitten.com/500/150" class="w-full bg-red-700 rounded-t-lg" {}
                    img src="https://placekitten.com/150/15" class="absolute bottom-0 w-24 h-24 transform -translate-x-1/2 translate-y-1/2 border-4 border-white rounded-full left-2/4" {}
                }

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
                    
                }
            }
        };

    Ok(index(req, content, &title, desc).await)
}
