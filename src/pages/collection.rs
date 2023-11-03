use actix_web::{ HttpRequest, Result as ActixResult, get, web };
use maud::{ Markup, html };

use crate::{
    pages::{ index::index, dummy_data::{ get_dummy_collections, get_dummy_assets } },
    components::{
        collection_card::{ collection_card, self },
        asset_card::asset_card,
        ui::button::button_primary,
    },
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
    let collections = get_dummy_collections();
    let collection = collections
        .iter()
        .find(|x| x.id == path.0)
        .unwrap();

    let assets = get_dummy_assets();
    let assets = assets
        .iter()
        .filter(|x| x.collection_id == path.0)
        .collect::<Vec<_>>();

    let has_assets = assets.len() > 0;

    // TODO: Update metadata to match fetched data
    let title = path.0.to_string();
    let desc = "NFT Collections";

    let content =
        html! {
            div {
                div class="relative flex flex-col items-center h-40 overflow-hidden md:h-44 lg:h-64 bg-background-popout" {
                    img src="https://placekitten.com/500/150" class="absolute z-0 object-cover w-full h-full t-0 l-0" {}
                }
                main class="container mx-auto" {
                    div class="px-4 lg:px-16"{
                        section class="box-border relative flex flex-col w-full px-0 -mt-16 bg-red-500 border border-solid rounded-lg border-border-primary bg-bg-popout" {
                            div class="p-4" {
                                p {(collection.name)}
                                p {(collection.volume)}
                                p {(collection.id)}
                                p {"Collection Cards"}
                            }
                        }
                    }

                    div class = "grid grid-cols-2 gap-3 mt-3" {
                        div class="col-span-2" {
                            (button_primary("Buy all"))
                        }

                        div class="col-span-1" {
                            (button_primary("Sweep Assets"))
                        }
                        
                        div class="col-span-1" {
                            (button_primary("Sell Asset"))
                        }

                        div class="flex col-span-2" {
                            input type = "text" placeholder = "Search Assets" class="flex-1" {}
                            div class = "flex" {
                                img src="filter.jpg"
                                label {"Filter"}
                            }
                        }
                    }

                    div class ="grid w-full h-full grid-cols-2 gap-4 p-2 mx-auto mt-3 bg-blue-300 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5" { 
                        // TODO: Replace with actual collection cards
                        @if !has_assets {
                            // TODO: Replace with better looking no assets found ui
                            p {"No assets found"}
                        } @else {
                            @for asset in assets {
                                a href = (format!("/asset/{}", asset.id)) {
                                    (asset_card(&asset))
                                }
                            }
                        }
                    }
                }
            }
        };

    Ok(index(req, content, &title, desc).await)
}
