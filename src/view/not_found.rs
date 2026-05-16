use maud::{Markup, html};

use crate::i18n::Messages;

pub fn body(locale: &str, m: &Messages) -> Markup {
    html! {
        section class="relative min-h-[70vh] flex items-center justify-center px-4" {
            div class="relative z-[2] max-w-md mx-auto text-center" {
                p class="t-caption mb-6" { "404" }
                h1 class="t-h1 mb-4" { span class="text-aurora" { (m.notfound.title) } }
                p class="t-lead mb-10" { (m.notfound.message) }
                a href=(format!("/{locale}")) class="btn btn-primary inline-flex" {
                    (m.notfound.back)
                }
            }
        }
    }
}
