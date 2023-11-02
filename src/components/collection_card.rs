use maud::{ Markup, html };

pub fn collection_card() -> Markup {
    html! {
        div ."flex flex-col h-full bg-white rounded-md shadow-md rounded-md"
        {
            img src = "../static/images/nft-collection-placeholder.webp" class = "w-full rounded-t-md flex-4/6" {}
            div class = "flex flex-col p-4 flex-2/6" {
                p class="text-xs" {"Collection Name"}
                p class="text-xs" {"P 43.1m volume"}
            }
        }
    }
}
