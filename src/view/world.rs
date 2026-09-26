use chrono::Utc;
use maud::{Markup, PreEscaped, html};

use crate::i18n::{IslandCopy, Messages};
use crate::view::icons;
use crate::view::layout::asset;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Island {
    Overview,
    Basecamp,
    Projects,
    About,
    Experience,
    Contact,
}

impl Island {
    pub const ALL: [Island; 6] = [
        Island::Overview,
        Island::Basecamp,
        Island::Projects,
        Island::About,
        Island::Experience,
        Island::Contact,
    ];

    pub fn key(self) -> &'static str {
        match self {
            Island::Overview => "overview",
            Island::Basecamp => "basecamp",
            Island::Projects => "projects",
            Island::About => "about",
            Island::Experience => "experience",
            Island::Contact => "contact",
        }
    }

    pub fn path(self) -> &'static str {
        match self {
            Island::Overview => "",
            Island::Basecamp => "/basecamp",
            Island::Projects => "/projects",
            Island::About => "/about",
            Island::Experience => "/experience",
            Island::Contact => "/contact",
        }
    }

    pub fn icon(self) -> &'static str {
        match self {
            Island::Overview => icons::OVERVIEW,
            Island::Basecamp => icons::BASECAMP,
            Island::Projects => icons::PROJECTS,
            Island::About => icons::ABOUT,
            Island::Experience => icons::EXPERIENCE,
            Island::Contact => icons::MAIL,
        }
    }

    pub fn copy(self, m: &Messages) -> Option<&IslandCopy> {
        let islands = &m.world.islands;
        match self {
            Island::Overview => None,
            Island::Basecamp => Some(&islands.basecamp),
            Island::Projects => Some(&islands.projects),
            Island::About => Some(&islands.about),
            Island::Experience => Some(&islands.experience),
            Island::Contact => Some(&islands.contact),
        }
    }

    pub fn label(self, m: &Messages) -> &str {
        self.copy(m)
            .map_or(m.world.overview.as_str(), |c| c.label.as_str())
    }

    pub fn meta(self, m: &Messages) -> String {
        let count = match self {
            Island::Projects => m.projects.items.len(),
            Island::Experience => m.experience.items.len(),
            _ => 0,
        };
        self.copy(m)
            .map(|c| c.meta.replace("{count}", &count.to_string()))
            .unwrap_or_default()
    }

    pub fn href(self, locale: &str) -> String {
        format!("/{locale}{}", self.path())
    }
}

#[derive(Default)]
pub struct PanelHead<'a> {
    pub id: &'a str,
    pub eyebrow: &'a str,
    pub title: &'a str,
    pub behind_modal: bool,
}

pub fn panel(locale: &str, m: &Messages, head: PanelHead<'_>, content: Markup) -> Markup {
    html! {
        section class="panel glass" aria-labelledby="panel-title" data-panel=(head.id) inert[head.behind_modal] {
            header class="panel-head" {
                div {
                    p class="eyebrow" { (head.eyebrow) }
                    @if head.behind_modal {
                        h2 #panel-title { (head.title) }
                    } @else {
                        h1 #panel-title tabindex="-1" { (head.title) }
                    }
                }
                div class="panel-actions" {
                    button type="button" class="chip glass panel-toggle" data-panel-toggle
                           aria-expanded="true" aria-controls="panel-scroll"
                           aria-label=(m.world.collapse) title=(m.world.collapse)
                           data-label-collapse=(m.world.collapse) data-label-expand=(m.world.expand) {
                        (PreEscaped(icons::CHEVRON))
                    }
                    a class="chip glass" href=(Island::Overview.href(locale)) data-nav {
                        (PreEscaped(icons::ARROW_LEFT))
                        span { (m.world.overview) }
                    }
                }
            }
            div #panel-scroll class="panel-scroll" {
                div class="panel-body" { (content) }
                (footer(locale, m))
            }
        }
    }
}

pub fn footer(locale: &str, m: &Messages) -> Markup {
    let year = Utc::now().format("%Y").to_string();
    html! {
        footer class="site-foot" {
            nav class="site-foot-links" {
                a href=(format!("/{locale}/impressum")) rel="nofollow" data-nav { (m.footer.impressum) }
                a href=(format!("/{locale}/privacy")) rel="nofollow" data-nav { (m.footer.privacy) }
            }
            p { "© " (year) " " (m.footer.copyright) " · v" (env!("CARGO_PKG_VERSION")) }
        }
    }
}

pub fn profile(m: &Messages, lead: &str) -> Markup {
    html! {
        div class="profile" {
            img class="avatar" src=(asset(&m.about.avatar.image)) alt=(m.about.avatar.alt)
                width="72" height="72" decoding="async";
            p class="lead" { (lead) }
        }
    }
}

pub fn signposts(m: &Messages) -> String {
    let labels: Vec<String> = m
        .experience
        .items
        .iter()
        .map(|item| {
            let year = item.period.get(..4).unwrap_or(&item.period);
            let company = item.company.split_whitespace().next().unwrap_or_default();
            format!("{year} {company}")
        })
        .collect();
    serde_json::to_string(&labels).unwrap_or_else(|_| "[]".into())
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::sync::Arc;

    use super::*;
    use crate::i18n::I18n;

    fn messages() -> Arc<Messages> {
        I18n::load(&["de-DE".to_string()], "de-DE")
            .unwrap()
            .get("de-DE")
    }

    #[test]
    fn island_meta_fills_in_counts() {
        let m = messages();
        assert_eq!(
            Island::Projects.meta(&m),
            format!("{} Projekte", m.projects.items.len())
        );
        assert_eq!(Island::Overview.meta(&m), "");
    }

    #[test]
    fn signposts_pair_start_year_with_company() {
        let m = messages();
        let labels: Vec<String> = serde_json::from_str(&signposts(&m)).unwrap();
        assert_eq!(labels.len(), m.experience.items.len());
        assert_eq!(labels[0], "2023 Eventfrog");
    }

    #[test]
    fn island_keys_and_paths_are_unique() {
        let keys: HashSet<_> = Island::ALL.iter().map(|i| i.key()).collect();
        let paths: HashSet<_> = Island::ALL.iter().map(|i| i.path()).collect();
        assert_eq!(keys.len(), Island::ALL.len());
        assert_eq!(paths.len(), Island::ALL.len());
    }
}
