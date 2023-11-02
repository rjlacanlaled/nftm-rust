use maud::{ Markup, html };

use crate::pages::asset::Asset;

pub fn asset_card(asset: &Asset) -> Markup {
    html! {
        div ."flex flex-col h-full bg-white rounded-md shadow-md rounded-md"
        {
            img src = (asset.img_url) class = "w-full rounded-t-md flex-4/6" {}
            div class = "flex flex-col p-4 flex-2/6" {
                p class="text-xs" {(asset.name)}
                p class="text-xs" {(format!("PHP {} volume", asset.price))}
                p class="text-xs" {(format!("CollectionId#{}", asset.collection_id))}
            }
        }
    }
}
