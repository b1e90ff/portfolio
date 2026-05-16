use maud::{Markup, PreEscaped, html};
use serde_json::Value;

use crate::i18n::Messages;
use crate::state::AppState;
use crate::view::schema;

pub fn body(_state: &AppState, _locale: &str, m: &Messages) -> Markup {
    html! {
        section class="container mx-auto px-4 pt-32 sm:pt-40 pb-16" {
            div class="max-w-2xl mx-auto" {
                header class="mb-12" {
                    h1 class="t-h1 mb-4" { span class="text-aurora" { (m.contact.title) } }
                    p class="t-lead max-w-xl" { (m.contact.intro) }
                }

                section class="border-t border-[var(--border-subtle)] pt-8" {
                    h2 class="t-caption mb-5" { (m.contact.channel_heading) }
                    ul class="grid grid-cols-1 sm:grid-cols-2 gap-2" {
                        (channel_link(&m.social.email.label, &m.social.email.href, icon_mail(), false))
                        (channel_link(&m.social.imessage.label, &m.social.imessage.href, icon_message(), false))
                        (channel_link(&m.social.github.label, &m.social.github.href, icon_github(), true))
                        (channel_link(&m.social.linkedin.label, &m.social.linkedin.href, icon_linkedin(), true))
                        (channel_toggle(&m.contact.form_channel))
                    }

                    div class="hidden mt-6 rounded-xl border border-[var(--border-subtle)] p-6"
                        style="background-color: var(--surface-1);"
                        data-contact-form-panel {
                        p class="t-small mb-6" data-contact-hint { (m.contact.form_channel_hint) }
                        (form(m))
                    }
                }
            }
        }
    }
}

pub fn extra_schemas(state: &AppState, locale: &str, m: &Messages) -> Vec<Value> {
    vec![
        schema::web_page(
            state,
            locale,
            "/contact",
            &m.contact.title,
            &m.contact.description,
            "ContactPage",
            &m.structured_data.person.name,
        ),
        schema::breadcrumb(
            state,
            locale,
            &[
                (m.navigation.home.as_str(), ""),
                (m.contact.title.as_str(), "/contact"),
            ],
        ),
    ]
}

fn channel_toggle(label: &str) -> Markup {
    html! {
        li {
            button type="button"
                   data-contact-form-toggle
                   aria-expanded="false"
                   aria-controls="contact-form-panel"
                   class="w-full flex items-center gap-3 rounded-lg border border-transparent px-4 py-3 hover:border-[var(--border-glass)] hover:bg-[var(--surface-2)] transition-colors text-left text-[var(--text-secondary)] hover:text-[var(--foreground)] cursor-pointer" {
                span class="inline-flex w-8 h-8 items-center justify-center rounded-md"
                     style="color: var(--accent-warm); background-color: var(--primary-accent-soft);" {
                    (icon_form())
                }
                span class="font-medium" { (label) }
            }
        }
    }
}

fn icon_form() -> Markup { html! { (PreEscaped(r#"<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><rect x="3" y="4" width="18" height="16" rx="2"/><path d="M7 9h10"/><path d="M7 13h7"/><path d="M7 17h4"/></svg>"#)) } }

fn channel_link(label: &str, href: &str, icon: Markup, external: bool) -> Markup {
    html! {
        li {
            a href=(href)
              target=[external.then_some("_blank")]
              rel=[external.then_some("noopener noreferrer")]
              class="flex items-center gap-3 rounded-lg border border-transparent px-4 py-3 hover:border-[var(--border-glass)] hover:bg-[var(--surface-2)] transition-colors text-[var(--text-secondary)] hover:text-[var(--foreground)]" {
                span class="inline-flex w-8 h-8 items-center justify-center rounded-md"
                     style="color: var(--accent-warm); background-color: var(--primary-accent-soft);" {
                    (icon)
                }
                span class="font-medium" { (label) }
            }
        }
    }
}

fn form(m: &Messages) -> Markup {
    let f = &m.contact.form;
    html! {
        form id="contact-form-panel" class="space-y-5" novalidate action="/api/contact" method="post" data-contact-form {
            div class="absolute opacity-0 h-0 w-0 overflow-hidden" aria-hidden="true" {
                label for="_honeypot" { "Do not fill" }
                input type="text" id="_honeypot" name="_honeypot" tabindex="-1" autocomplete="off";
            }

            (text_field("name", &f.name, true, "given-name", 2, 100, false))
            (text_field("email", &f.email, true, "email", 5, 254, true))
            (text_field("subject", &f.subject, true, "off", 3, 200, false))
            (textarea_field("message", &f.message, true, 10, 5000))

            div class="hidden p-4 rounded-xl text-sm"
                style="background-color: rgba(248,113,113,0.10); border:1px solid rgba(248,113,113,0.25); color: var(--error);"
                data-contact-error {
                (f.error)
            }
            div class="hidden p-4 rounded-xl text-sm"
                style="background-color: rgba(52,211,153,0.10); border:1px solid rgba(52,211,153,0.25); color: var(--success);"
                data-contact-success {
                (f.success)
            }

            button type="submit" class="btn btn-primary w-full" data-contact-submit {
                span data-contact-label-idle { (f.send) }
                span class="hidden" data-contact-label-sending { (f.sending) }
            }
        }
    }
}

fn text_field(
    name: &str,
    label: &str,
    required: bool,
    autocomplete: &str,
    min_len: u32,
    max_len: u32,
    is_email: bool,
) -> Markup {
    let input_type = if is_email { "email" } else { "text" };
    html! {
        div {
            label for=(name) class="block t-caption mb-2" {
                (label)
                @if required { span style="color: var(--accent-warm);" { " *" } }
            }
            input type=(input_type)
                  id=(name)
                  name=(name)
                  required[required]
                  autocomplete=(autocomplete)
                  minlength=(min_len)
                  maxlength=(max_len)
                  class="w-full px-4 py-2.5 rounded-lg text-sm border border-[var(--border-subtle)] focus:outline-none focus:border-[var(--accent-warm)] transition-colors"
                  style="background-color: var(--surface-1); color: var(--foreground);";
        }
    }
}

fn textarea_field(name: &str, label: &str, required: bool, min_len: u32, max_len: u32) -> Markup {
    html! {
        div {
            label for=(name) class="block t-caption mb-2" {
                (label)
                @if required { span style="color: var(--accent-warm);" { " *" } }
            }
            textarea id=(name)
                     name=(name)
                     required[required]
                     rows="6"
                     minlength=(min_len)
                     maxlength=(max_len)
                     class="w-full px-4 py-3 rounded-lg text-sm border border-[var(--border-subtle)] focus:outline-none focus:border-[var(--accent-warm)] transition-colors resize-y"
                     style="background-color: var(--surface-1); color: var(--foreground);" {}
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
