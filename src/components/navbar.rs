use maud::{ Markup, html };

use crate::components::ui::button::button;

pub fn navbar() -> Markup {
    html! {
        nav ."flex justify-between px-10 py-2 bg-blue-100" {
            // Nav routes
            div ."flex gap-4 bg-red-200 bg-red-500" {
                a href="/" {"Home"}
                a href="/collection" {"Collections"}
            }

            // Auth / User profile
            div ."flex gap-4" {
                (button("Login", "primary"))
            }
        }
    }
}
