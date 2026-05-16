use maud::{Markup, PreEscaped, html};

use crate::i18n::Messages;
use crate::state::AppState;

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
                    a href=(format!("/{locale}")) class="inline-flex items-center -ml-1 p-1.5 rounded-md"
                      aria-label="Niklas Tat" {
                        img src="/tat.webp" alt="Niklas Tat" width="28" height="28" loading="eager" decoding="async";
                    }

                    div class="hidden sm:flex items-center gap-1 ml-6" {
                        @for (label, p) in nav {
                            (nav_link(locale, path, label, p))
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

pub fn footer(_state: &AppState, _locale: &str, _m: &Messages) -> Markup {
    html! {}
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
                @let label = locale_label(locale);
                a href=(href)
                  aria-pressed=(active.to_string())
                  aria-label=(format!("Switch language to {label}"))
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

fn locale_label(locale: &str) -> &'static str {
    match locale {
        "de-DE" => "DE",
        "en-US" => "EN",
        _ => "EN",
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
