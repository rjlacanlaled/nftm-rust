use actix_web::{ HttpRequest, Result as ActixResult, get };
use maud::{ Markup, html };

use crate::{
    pages::{ index::index, dummy_data::get_dummy_assets },
    components::asset_card::asset_card,
};

#[get("/assets")]
pub async fn page(req: HttpRequest) -> ActixResult<Markup> {
    // TODO: Fetch collections data from database
    let assets = get_dummy_assets();

    let title = "Assets";
    let desc = "List of assets";

    let content =
        html! {
            div class = "flex flex-col w-full h-full gap-10 py-5 bg-red-300" {
                h1 {
                    div class = "flex gap-4" {
                        span {"Explore"}
                        span {"All Assets"}
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
                    @for asset in assets {
                        a href = (format!("/asset/{}", asset.id)) {
                            (asset_card(&asset))
                        }
                    }
                }
            }
        };

    Ok(index(req, content, title, desc).await)
}
