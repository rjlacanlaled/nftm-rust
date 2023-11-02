use maud::{ Markup, html };

use crate::components::ui::button::button_primary;

pub fn navbar() -> Markup {
    html! {
        nav ."flex justify-between px-10 py-2 bg-blue-100" {
                // Logo
                div class="flex gap-2" {
                    img src="/logo.png" class="w-10 h-10" {}
                    p class="hidden lg:block" {"Slogan"}
                }
                
                // Mobile
                div class="lg:hidden" {
                    img src="../static/icons/hamburger-menu.svg" class="w-10 h-10" _ = "on click remove .hidden from #mobile-menu" {}
                }

                div class = "hidden" id = "mobile-menu" {
                    (mobile_menu())
                }

                // Desktop

                // Nav routes
                div class="hidden gap-2 lg:flex" {
                    a href="/" {"Home"}
                    a href="/collections" {"Collections"}
                }

                // Auth / User profile
                div class="hidden gap-2 lg:flex" {
                    (button_primary("Login"))
                    (button_primary("Sign up"))
                }
            }
        
    }
}

fn mobile_menu() -> Markup {
    html! {
        div ."flex flex-col gap-2 bg-blue-100 absolute h-full w-full top-0 left-0 px-10 py-2 lg:hidden"
        {
            button _ = "on click add .hidden to #mobile-menu" {"X"}
            a href = "/" { "Home" }
            a href = "/collection" { "Collections" }
            (button_primary("Login"))
            (button_primary("Sign up"))
        }
    }
}
