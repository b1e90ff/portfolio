use maud::{Markup, PreEscaped, html};

use crate::i18n::Messages;
use crate::state::AppState;
use crate::view::layout::asset;

pub fn header(state: &AppState, locale: &str, path: &str, m: &Messages) -> Markup {
    let nav = [
        (m.navigation.home.as_str(), ""),
        (m.navigation.projects.as_str(), "/projects"),
        (m.navigation.about.as_str(), "/about"),
        (m.navigation.contact.as_str(), "/contact"),
    ];

    html! {
        nav class="fixed top-0 left-0 right-0 z-50 backdrop-blur-xl border-b border-[var(--border-subtle)]"
            style="background-color: var(--nav-bg);" {
            div class="max-w-6xl mx-auto px-4 sm:px-6" {
                div class="flex justify-between items-center h-14" {
                    div class="flex items-center gap-6" {
                        a href=(format!("/{locale}")) class="inline-flex items-center -ml-1 p-1.5 rounded-md"
                          aria-label="Niklas Tat" {
                            img src=(asset("/tat.webp")) alt="Niklas Tat" width="28" height="28" loading="eager" decoding="async";
                        }

                        div class="hidden sm:flex items-center gap-1" {
                            @for (label, p) in nav {
                                (nav_link(locale, path, label, p))
                            }
                        }
                    }

                    div class="flex items-center gap-1.5" {
                        (theme_toggle(m))
                        (language_switcher(state, locale, path))
                        button type="button"
                               class="sm:hidden inline-flex items-center justify-center w-11 h-11 rounded-md text-[var(--text-secondary)] hover:text-[var(--foreground)] transition-colors"
                               data-mobile-menu-toggle
                               aria-controls="mobile-menu"
                               aria-expanded="false"
                               aria-label=(m.navigation.open_menu) {
                            (icon_menu())
                        }
                    }
                }
            }

            div #mobile-menu
                class="sm:hidden hidden border-t border-[var(--border-subtle)]"
                data-mobile-menu {
                div class="px-4 py-3 space-y-1" {
                    @for (label, p) in nav {
                        (mobile_nav_link(locale, path, label, p))
                    }
                }
            }
        }
    }
}

pub fn footer(_state: &AppState, locale: &str, m: &Messages) -> Markup {
    let year = chrono::Utc::now().format("%Y").to_string();
    html! {
        footer class="relative z-10 mt-24 border-t border-[var(--border-subtle)]" {
            div class="max-w-6xl mx-auto px-4 sm:px-6 py-12" {
                div class="flex flex-col sm:flex-row justify-between items-center gap-8" {
                    div class="flex flex-wrap justify-center gap-6 text-sm" {
                        a href=(format!("/{locale}/contact"))
                          class="text-[var(--text-secondary)] hover:text-[var(--foreground)] transition-colors" {
                            (m.footer.contact)
                        }
                        a href=(format!("/{locale}/impressum"))
                          rel="nofollow"
                          class="text-[var(--text-secondary)] hover:text-[var(--foreground)] transition-colors" {
                            (m.footer.impressum)
                        }
                        a href=(format!("/{locale}/privacy"))
                          rel="nofollow"
                          class="text-[var(--text-secondary)] hover:text-[var(--foreground)] transition-colors" {
                            (m.footer.privacy)
                        }
                    }

                    div class="flex items-center gap-2" {
                        (social_link(&m.social.email.href, &m.social.email.label, icon_mail(), false))
                        (social_link(&m.social.imessage.href, &m.social.imessage.label, icon_message(), false))
                        (social_link(&m.social.github.href, &m.social.github.label, icon_github(), true))
                        (social_link(&m.social.linkedin.href, &m.social.linkedin.label, icon_linkedin(), true))
                    }
                }

                div class="mt-8 pt-6 border-t border-[var(--border-subtle)] flex flex-col sm:flex-row items-center justify-center gap-1 sm:gap-3" {
                    p class="text-xs text-[var(--text-tertiary)]" {
                        "© " (year) " " (m.footer.copyright)
                    }
                    span class="hidden sm:inline text-[var(--text-tertiary)]" aria-hidden="true" { "|" }
                    p class="text-xs text-[var(--text-tertiary)]" {
                        "v" (env!("CARGO_PKG_VERSION"))
                    }
                }
            }
        }
        (back_to_top())
    }
}

fn social_link(href: &str, label: &str, icon: Markup, external: bool) -> Markup {
    html! {
        a href=(href)
          aria-label=(label)
          target=[external.then_some("_blank")]
          rel=[external.then_some("noopener noreferrer")]
          class="inline-flex items-center justify-center w-10 h-10 rounded-full border border-transparent text-[var(--text-secondary)] hover:text-[var(--foreground)] hover:border-[var(--border-glass)] transition-colors" {
            (icon)
        }
    }
}

fn back_to_top() -> Markup {
    html! {
        button type="button"
               data-back-to-top
               aria-label="Back to top"
               class="fixed bottom-6 right-6 z-50 p-3 rounded-xl backdrop-blur-xl border border-[var(--border-glass)] text-[var(--text-secondary)] hover:text-[var(--accent-warm)] hover:border-[var(--border-glass-hover)] transition-all duration-300 opacity-0 translate-y-4 pointer-events-none"
               style="background-color: var(--background-glass);" {
            (PreEscaped(r#"<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><line x1="12" y1="19" x2="12" y2="5"/><polyline points="5 12 12 5 19 12"/></svg>"#))
        }
    }
}

fn icon_mail() -> Markup {
    html! { (PreEscaped(r#"<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M4 4h16c1.1 0 2 .9 2 2v12c0 1.1-.9 2-2 2H4c-1.1 0-2-.9-2-2V6c0-1.1.9-2 2-2z"/><polyline points="22,6 12,13 2,6"/></svg>"#)) }
}

fn icon_message() -> Markup {
    html! { (PreEscaped(r#"<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z"/></svg>"#)) }
}

fn icon_github() -> Markup {
    html! { (PreEscaped(r#"<svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true"><path d="M12 .5C5.65.5.5 5.65.5 12c0 5.08 3.29 9.38 7.86 10.9.58.1.79-.25.79-.55v-2.1c-3.2.7-3.87-1.36-3.87-1.36-.52-1.33-1.28-1.68-1.28-1.68-1.05-.71.08-.7.08-.7 1.16.08 1.78 1.2 1.78 1.2 1.03 1.76 2.7 1.25 3.36.95.1-.75.4-1.25.73-1.54-2.55-.29-5.24-1.27-5.24-5.65 0-1.25.45-2.27 1.18-3.07-.12-.29-.51-1.46.11-3.04 0 0 .96-.31 3.15 1.17.92-.25 1.9-.38 2.88-.39.98.01 1.96.14 2.88.39 2.19-1.48 3.15-1.17 3.15-1.17.62 1.58.23 2.75.11 3.04.73.8 1.18 1.82 1.18 3.07 0 4.39-2.69 5.35-5.25 5.64.41.36.78 1.06.78 2.15v3.19c0 .31.21.66.8.55C20.21 21.38 23.5 17.07 23.5 12 23.5 5.65 18.35.5 12 .5z"/></svg>"#)) }
}

fn icon_linkedin() -> Markup {
    html! { (PreEscaped(r#"<svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true"><path d="M20.45 20.45h-3.55v-5.57c0-1.33-.03-3.05-1.86-3.05-1.86 0-2.14 1.45-2.14 2.95v5.67H9.34V9h3.41v1.56h.05c.47-.9 1.63-1.85 3.36-1.85 3.6 0 4.26 2.37 4.26 5.45v6.29zM5.34 7.43a2.06 2.06 0 1 1 0-4.12 2.06 2.06 0 0 1 0 4.12zM7.12 20.45H3.56V9h3.56v11.45z"/></svg>"#)) }
}

fn nav_link(locale: &str, current: &str, label: &str, path: &str) -> Markup {
    let active = current == path;
    let href = format!("/{locale}{path}");
    html! {
        a href=(href)
          aria-current=[active.then_some("page")]
          class="relative px-3 py-2 text-sm font-medium text-[var(--text-secondary)] hover:text-[var(--foreground)] transition-colors" {
            span { (label) }
            @if active {
                span class="absolute left-3 right-3 -bottom-px h-px"
                     style="background-color: var(--accent-warm);" {}
            }
        }
    }
}

fn mobile_nav_link(locale: &str, current: &str, label: &str, path: &str) -> Markup {
    let active = current == path;
    let href = format!("/{locale}{path}");
    html! {
        a href=(href)
          aria-current=[active.then_some("page")]
          class="flex items-center justify-between min-h-11 px-3 py-2.5 rounded-md text-[var(--text-secondary)] hover:text-[var(--foreground)] hover:bg-[var(--surface-1)] transition-colors"
          data-mobile-link {
            span { (label) }
            @if active {
                span class="w-1.5 h-1.5 rounded-full"
                     style="background-color: var(--accent-warm);" {}
            }
        }
    }
}

fn theme_toggle(m: &Messages) -> Markup {
    html! {
        button type="button"
               class="inline-flex items-center justify-center w-11 h-11 rounded-full border border-[var(--border-glass)] text-[var(--text-secondary)] hover:text-[var(--foreground)] hover:border-[var(--border-glass-hover)] transition-colors"
               data-theme-toggle
               aria-label=(m.navigation.toggle_theme) {
            span data-theme-icon="dark" class="hidden" { (icon_sun()) }
            span data-theme-icon="light" { (icon_moon()) }
        }
    }
}

fn language_switcher(state: &AppState, current: &str, path: &str) -> Markup {
    html! {
        div class="hidden sm:flex items-center rounded-lg border border-[var(--border-glass)] overflow-hidden" {
            @for locale in state.i18n.locales() {
                @let active = locale == current;
                @let href = format!("/{locale}{path}");
                @let target = state.i18n.get(locale);
                @let label = target.navigation.language_short.clone();
                a href=(href)
                  hreflang=(locale)
                  aria-pressed=(active.to_string())
                  aria-label=(target.navigation.toggle_theme)
                  class={
                      "min-w-11 h-11 inline-flex items-center justify-center px-3 text-xs font-medium transition-colors "
                      @if active { "bg-[var(--accent-warm)] text-[#09090b]" }
                      @else { "text-[var(--text-secondary)] hover:text-[var(--foreground)] hover:bg-[var(--surface-1)]" }
                  } {
                    (label)
                }
            }
        }
    }
}

fn icon_menu() -> Markup {
    html! {
        (PreEscaped(r#"<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" data-menu-icon="closed"><line x1="3" y1="6" x2="21" y2="6"/><line x1="3" y1="12" x2="21" y2="12"/><line x1="3" y1="18" x2="21" y2="18"/></svg>"#))
        (PreEscaped(r#"<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" class="hidden" data-menu-icon="open"><line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/></svg>"#))
    }
}

fn icon_sun() -> Markup {
    html! {
        (PreEscaped(r#"<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><circle cx="12" cy="12" r="5"/><line x1="12" y1="1" x2="12" y2="3"/><line x1="12" y1="21" x2="12" y2="23"/><line x1="4.22" y1="4.22" x2="5.64" y2="5.64"/><line x1="18.36" y1="18.36" x2="19.78" y2="19.78"/><line x1="1" y1="12" x2="3" y2="12"/><line x1="21" y1="12" x2="23" y2="12"/><line x1="4.22" y1="19.78" x2="5.64" y2="18.36"/><line x1="18.36" y1="5.64" x2="19.78" y2="4.22"/></svg>"#))
    }
}

fn icon_moon() -> Markup {
    html! {
        (PreEscaped(r#"<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"/></svg>"#))
    }
}
