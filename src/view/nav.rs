use maud::{Markup, PreEscaped, html};

use crate::i18n::Messages;
use crate::state::AppState;
use crate::view::world::Island;

pub const MOODS: [&str; 5] = ["dusk", "night", "aurora", "peach", "day"];
pub const DEFAULT_MOOD: &str = MOODS[0];

pub fn topbar(state: &AppState, locale: &str, path: &str, m: &Messages) -> Markup {
    let person = &m.structured_data.person;
    html! {
        header class="topbar" {
            a class="brand glass" href=(Island::Overview.href(locale)) data-nav {
                img src="/android-chrome-192x192.png" alt="" width="38" height="38" decoding="async";
                span class="brand-text" {
                    b { (person.name) }
                    small { (person.job_title) }
                }
            }
            div class="topbar-controls" {
                (moods(m))
                (language_switch(state, locale, path, m))
            }
        }
    }
}

fn moods(m: &Messages) -> Markup {
    let labelled = MOODS
        .iter()
        .filter_map(|key| m.world.moods.label(key).map(|name| (*key, name)));
    html! {
        div class="moods glass" role="group" aria-label=(m.world.mood_label) {
            span class="moods-label" aria-hidden="true" { (m.world.mood_label) }
            @for (key, name) in labelled {
                button type="button" class="mood" data-mood-option=(key)
                       aria-pressed=((key == DEFAULT_MOOD).to_string())
                       title=(name) aria-label=(name) {}
            }
        }
    }
}

pub fn language_switch(state: &AppState, current: &str, path: &str, m: &Messages) -> Markup {
    html! {
        nav class="lang glass" aria-label=(m.world.language_label) data-swap="lang" {
            @for locale in state.i18n.locales() {
                @let target = state.i18n.get(locale);
                a href=(format!("/{locale}{path}"))
                  hreflang=(locale)
                  lang=(locale)
                  aria-current=[(locale == current).then_some("true")] {
                    (target.navigation.language_short)
                }
            }
        }
    }
}

pub fn dock(locale: &str, m: &Messages, current: Option<Island>) -> Markup {
    html! {
        nav class="dock glass" aria-label=(m.world.dock_label) data-swap="dock" {
            @for island in Island::ALL {
                a href=(island.href(locale))
                  data-nav
                  data-island=(island.key())
                  aria-current=[(Some(island) == current).then_some("page")] {
                    (PreEscaped(island.icon()))
                    span { (island.label(m)) }
                }
            }
        }
    }
}

pub fn pins(locale: &str, m: &Messages) -> Markup {
    html! {
        div class="pins" data-pins {
            @for island in Island::ALL.into_iter().filter(|i| *i != Island::Overview) {
                a class="pin" href=(island.href(locale)) data-nav data-pin=(island.key()) tabindex="-1" {
                    span class="pin-chip glass" {
                        span class="pin-icon" { (PreEscaped(island.icon())) }
                        span class="pin-text" {
                            b { (island.label(m)) }
                            small { (island.meta(m)) }
                        }
                        span class="pin-go" { (m.world.land) " →" }
                    }
                    span class="pin-stem" {}
                    span class="pin-dot" {}
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::i18n::I18n;

    #[test]
    fn every_mood_has_a_label_in_every_locale() {
        let locales = ["en-US".to_string(), "de-DE".to_string()];
        let i18n = I18n::load(&locales, "en-US").unwrap();
        for locale in &locales {
            let m = i18n.get(locale);
            for key in MOODS {
                assert!(m.world.moods.label(key).is_some(), "{locale} {key}");
            }
        }
    }
}
