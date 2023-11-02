use maud::{ Markup, html };

pub fn button_primary(title: &str) -> Markup {
    html! {
        input class = "w-full p-2 font-semibold text-white rounded-md text-md bg-primary" type="button" value=(title);
    }
}
