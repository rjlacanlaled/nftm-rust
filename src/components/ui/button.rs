use maud::{ Markup, html };

pub fn button(title: &str, theme: &str) -> Markup {
    let class = format!("text-white p-2 font-semibold text-md bg-{} rounded-md", theme);

    html! {
        button class=(class) {(title)}
    }
}
