use maud::{ Markup, html };

use crate::pages::collection::Collection;

pub fn collection_card(collection: &Collection) -> Markup {
    html! {
        div ."flex flex-col h-full bg-white rounded-md shadow-md rounded-md"
        {
            img src = (collection.img_url) class = "w-full rounded-t-md flex-4/6" {}
            div class = "flex flex-col p-4 flex-2/6" {
                p class="text-xs" {(collection.name)}
                p class="text-xs" {(format!("PHP {} volume", collection.volume))}
            }
        }
    }
}
