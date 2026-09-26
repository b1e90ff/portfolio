use maud::{Markup, PreEscaped, html};
use serde_json::Value;

use crate::i18n::Messages;
use crate::state::AppState;
use crate::view::world::{self, Island, PanelHead};
use crate::view::{icons, schema};

pub fn body(locale: &str, m: &Messages) -> Markup {
    let head = PanelHead {
        id: Island::Contact.key(),
        eyebrow: Island::Contact.label(m),
        title: &m.contact.heading,
        ..PanelHead::default()
    };
    let social = &m.social;
    let content = html! {
        p class="lead" { (m.contact.intro) }
        (form(m))
        h2 class="section-title" { (m.contact.channel_heading) }
        ul class="channels" {
            (channel(&social.email.label, &social.email.href, icons::MAIL, false))
            (channel(&social.imessage.label, &social.imessage.href, icons::MESSAGE, false))
            (channel(&social.github.label, &social.github.href, icons::GITHUB, true))
            (channel(&social.linkedin.label, &social.linkedin.href, icons::LINKEDIN, true))
        }
    };
    world::panel(locale, m, head, content)
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

fn channel(label: &str, href: &str, icon: &'static str, external: bool) -> Markup {
    html! {
        li {
            a class="card channel" href=(href)
              target=[external.then_some("_blank")]
              rel=[external.then_some("noopener noreferrer")] {
                (PreEscaped(icon))
                span { (label) }
            }
        }
    }
}

fn form(m: &Messages) -> Markup {
    let f = &m.contact.form;
    html! {
        form class="form" novalidate action="/api/contact" method="post" data-contact-form {
            div class="honeypot" aria-hidden="true" {
                label for="_honeypot" { "Do not fill" }
                input type="text" id="_honeypot" name="_honeypot" tabindex="-1" autocomplete="off";
            }
            div class="form-row" {
                (field("name", "text", &f.name, "name", 2, 100))
                (field("email", "email", &f.email, "email", 5, 254))
            }
            (field("subject", "text", &f.subject, "off", 3, 200))
            div class="field" {
                label for="message" { (f.message) }
                textarea id="message" name="message" required rows="5" minlength="10" maxlength="5000" {}
            }
            p class="notice notice-error" role="alert" hidden data-contact-error { (f.error) }
            p class="notice notice-success" role="status" hidden data-contact-success { (f.success) }
            button type="submit" class="btn" data-contact-submit {
                span data-contact-label-idle { (f.send) }
                span hidden data-contact-label-sending { (f.sending) }
            }
        }
    }
}

fn field(
    name: &str,
    kind: &str,
    label: &str,
    autocomplete: &str,
    min_len: u32,
    max_len: u32,
) -> Markup {
    html! {
        div class="field" {
            label for=(name) { (label) }
            input type=(kind) id=(name) name=(name) required
                  autocomplete=(autocomplete) minlength=(min_len) maxlength=(max_len);
        }
    }
}
