use std::sync::Arc;

use anyhow::{Context, Result};
use resvg::tiny_skia::{Pixmap, Transform};
use resvg::usvg::fontdb;
use resvg::usvg::{Options, Tree};

use crate::i18n::Messages;

pub const WIDTH: u32 = 1200;
pub const HEIGHT: u32 = 630;

const INTER_BOLD: &[u8] = include_bytes!("../assets/fonts/Inter-Bold.ttf");
const INTER_REGULAR: &[u8] = include_bytes!("../assets/fonts/Inter-Regular.ttf");

pub fn render_png(locale: &str, m: &Messages) -> Result<Vec<u8>> {
    let svg = build_svg(locale, m);
    let mut db = fontdb::Database::new();
    db.load_font_data(INTER_BOLD.to_vec());
    db.load_font_data(INTER_REGULAR.to_vec());

    let opts = Options {
        fontdb: Arc::new(db),
        font_family: "Inter".to_string(),
        ..Default::default()
    };

    let tree = Tree::from_str(&svg, &opts).context("parse generated svg")?;
    let mut pixmap = Pixmap::new(WIDTH, HEIGHT).context("allocate pixmap")?;
    resvg::render(&tree, Transform::default(), &mut pixmap.as_mut());
    pixmap.encode_png().context("encode png")
}

fn build_svg(locale: &str, m: &Messages) -> String {
    let job_title = xml_escape(&m.structured_data.person.job_title);
    let name = xml_escape(&m.structured_data.person.name);
    let description = xml_escape(&truncate(&m.metadata.description, 110));
    let page_title = xml_escape(&m.hero.page_title);
    let host = xml_escape(domain_from_email(&m.structured_data.person.email));
    let locale_label = xml_escape(locale);

    format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1200 630" width="1200" height="630">
  <defs>
    <linearGradient id="bg" x1="0" y1="0" x2="1" y2="1">
      <stop offset="0%" stop-color="#0a0a0f"/>
      <stop offset="50%" stop-color="#14141d"/>
      <stop offset="100%" stop-color="#1a1625"/>
    </linearGradient>
    <linearGradient id="accent" x1="0" y1="0" x2="1" y2="0">
      <stop offset="0%" stop-color="#d4a44a"/>
      <stop offset="50%" stop-color="#e9c46a"/>
      <stop offset="100%" stop-color="#fcefc8"/>
    </linearGradient>
  </defs>

  <rect width="1200" height="630" fill="url(#bg)"/>

  <g opacity="0.05" fill="#e9c46a">
    <circle cx="0" cy="0" r="320"/>
    <circle cx="1200" cy="630" r="280"/>
  </g>

  <g font-family="Inter, sans-serif" fill="#fafafa">
    <text x="64" y="92" font-size="22" font-weight="700" letter-spacing="0.5">tat.systems</text>

    <text x="64" y="280" font-size="22" font-weight="600" letter-spacing="3" fill="#e9c46a" text-transform="uppercase">{job_title}</text>

    <text x="60" y="400" font-size="92" font-weight="700" letter-spacing="-3" fill="url(#accent)">{name}</text>

    <text x="64" y="468" font-size="26" font-weight="400" fill="rgba(245,245,247,0.74)">
      <tspan>{description}</tspan>
    </text>

    <text x="64" y="572" font-size="18" font-weight="500" fill="rgba(245,245,247,0.50)" letter-spacing="0.5">{page_title}</text>
    <text x="1136" y="572" font-size="18" font-weight="500" fill="rgba(245,245,247,0.50)" text-anchor="end" letter-spacing="0.5">{host} · {locale_label}</text>
  </g>
</svg>"##
    )
}

fn xml_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '&' => out.push_str("&amp;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&apos;"),
            _ => out.push(c),
        }
    }
    out
}

fn truncate(s: &str, max_chars: usize) -> String {
    let mut iter = s.chars();
    let head: String = iter.by_ref().take(max_chars).collect();
    if iter.next().is_some() {
        let mut head = head;
        // trim back to a word boundary
        while !head.is_empty() && !head.ends_with(' ') {
            head.pop();
        }
        head.trim_end().to_string() + "…"
    } else {
        head
    }
}

fn domain_from_email(email: &str) -> &str {
    email.rsplit_once('@').map(|(_, d)| d).unwrap_or(email)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Settings;
    use crate::i18n::I18n;

    fn fixture_messages() -> Arc<Messages> {
        let i18n = I18n::load(&["en-US".to_string()], "en-US").unwrap();
        i18n.get("en-US")
    }

    #[test]
    fn renders_png_with_expected_dimensions() {
        let m = fixture_messages();
        let bytes = render_png("en-US", &m).expect("render must succeed");
        assert!(bytes.starts_with(b"\x89PNG"));
        let png = image_dimensions_from_png(&bytes);
        assert_eq!(png, (WIDTH, HEIGHT));
    }

    fn image_dimensions_from_png(bytes: &[u8]) -> (u32, u32) {
        let w = u32::from_be_bytes([bytes[16], bytes[17], bytes[18], bytes[19]]);
        let h = u32::from_be_bytes([bytes[20], bytes[21], bytes[22], bytes[23]]);
        (w, h)
    }

    #[test]
    fn xml_escape_handles_specials() {
        assert_eq!(xml_escape("a & b <c>"), "a &amp; b &lt;c&gt;");
    }

    #[test]
    fn truncate_word_boundary() {
        let s = "hello world how are you";
        assert_eq!(truncate(s, 11), "hello…");
    }

    #[test]
    fn domain_extracted() {
        assert_eq!(domain_from_email("a@example.com"), "example.com");
        assert_eq!(domain_from_email("invalid"), "invalid");
    }

    // SAFETY: serialise env mutation across the test binary.
    fn _settings_unused() -> Settings {
        Settings {
            bind: "0.0.0.0:0".into(),
            base_url: "https://example.test".into(),
            default_locale: "en-US".into(),
            locales: vec!["en-US".into()],
            smtp: None,
        }
    }
}
