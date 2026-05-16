// The i18n catalogues mirror the JSON files 1:1, so the struct fields are
// driven by serde regardless of whether each one is currently bound in a
// view. We accept the dead-code warning at module scope rather than
// stripping fields and letting the JSON contract drift from the types.
#![allow(dead_code)]

use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::Arc;

use anyhow::{Context, Result, bail};
use serde::Deserialize;

pub const I18N_DIR: &str = "i18n";

#[derive(Debug, Clone)]
pub struct I18n {
    by_locale: HashMap<String, Arc<Messages>>,
    default_locale: String,
    locales: Vec<String>,
}

impl I18n {
    pub fn load(locales: &[String], default_locale: &str) -> Result<Self> {
        let mut by_locale = HashMap::with_capacity(locales.len());
        for locale in locales {
            let path = Path::new(I18N_DIR).join(format!("{locale}.json"));
            let raw = fs::read_to_string(&path)
                .with_context(|| format!("read translation file {}", path.display()))?;
            let messages: Messages = serde_json::from_str(&raw)
                .with_context(|| format!("parse translation file {}", path.display()))?;
            by_locale.insert(locale.clone(), Arc::new(messages));
        }

        if !by_locale.contains_key(default_locale) {
            bail!("default locale {default_locale} not loaded");
        }

        Ok(Self {
            by_locale,
            default_locale: default_locale.to_string(),
            locales: locales.to_vec(),
        })
    }

    pub fn get(&self, locale: &str) -> Arc<Messages> {
        self.by_locale
            .get(locale)
            .cloned()
            .unwrap_or_else(|| self.by_locale[&self.default_locale].clone())
    }

    pub fn has(&self, locale: &str) -> bool {
        self.by_locale.contains_key(locale)
    }

    pub fn locales(&self) -> &[String] {
        &self.locales
    }

    pub fn default_locale(&self) -> &str {
        &self.default_locale
    }

    pub fn html_lang(locale: &str) -> &str {
        locale
    }

    pub fn og_locale(locale: &str) -> String {
        locale.replace('-', "_")
    }
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Messages {
    pub metadata: Metadata,
    pub navigation: Navigation,
    pub hero: Hero,
    pub about: About,
    pub skills: Skills,
    pub experience: Experience,
    pub projects: Projects,
    pub contact: Contact,
    pub footer: Footer,
    pub social: Social,
    pub impressum: Impressum,
    pub privacy: Privacy,
    #[serde(rename = "structuredData")]
    pub structured_data: StructuredData,
    pub notfound: NotFound,
}

#[derive(Debug, Deserialize)]
pub struct Metadata {
    pub title: String,
    #[serde(rename = "titleTemplate")]
    pub title_template: String,
    pub description: String,
    pub manifest: ManifestStrings,
}

#[derive(Debug, Deserialize)]
pub struct ManifestStrings {
    pub name: String,
    #[serde(rename = "shortName")]
    pub short_name: String,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct Navigation {
    pub home: String,
    pub projects: String,
    pub about: String,
    pub contact: String,
    #[serde(rename = "openMenu")]
    pub open_menu: String,
    #[serde(rename = "closeMenu")]
    pub close_menu: String,
    #[serde(rename = "skipToContent")]
    pub skip_to_content: String,
    #[serde(rename = "toggleTheme")]
    pub toggle_theme: String,
    #[serde(rename = "languageShort")]
    pub language_short: String,
}

#[derive(Debug, Deserialize)]
pub struct Hero {
    pub title: String,
    #[serde(rename = "titlePrefix")]
    pub title_prefix: String,
    #[serde(rename = "titleName")]
    pub title_name: String,
    #[serde(rename = "pageTitle")]
    pub page_title: String,
    pub description: String,
    #[serde(rename = "ctaProjects")]
    pub cta_projects: String,
    #[serde(rename = "ctaContact")]
    pub cta_contact: String,
    #[serde(rename = "currentlyAt")]
    pub currently_at: String,
    #[serde(rename = "openSource")]
    pub open_source: String,
    #[serde(rename = "currentEmployerUrl")]
    pub current_employer_url: String,
}

#[derive(Debug, Deserialize)]
pub struct About {
    #[serde(rename = "aboutTitle")]
    pub about_title: String,
    pub avatar: Avatar,
    pub bio: Bio,
    pub approach: Approach,
    pub interests: Interests,
}

#[derive(Debug, Deserialize)]
pub struct Avatar {
    pub image: String,
    pub alt: String,
}

#[derive(Debug, Deserialize)]
pub struct Bio {
    pub title: String,
    pub content: String,
    pub highlights: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Approach {
    pub title: String,
    pub content: String,
    pub principles: Vec<Principle>,
}

#[derive(Debug, Deserialize)]
pub struct Principle {
    pub title: String,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct Interests {
    pub title: String,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct Skills {
    pub title: String,
    #[serde(rename = "moreSkillsTooltip")]
    pub more_skills_tooltip: String,
    pub categories: Vec<SkillCategory>,
}

#[derive(Debug, Deserialize)]
pub struct SkillCategory {
    pub name: String,
    pub icon: String,
    pub skills: Vec<String>,
    #[serde(rename = "moreSkillsText")]
    pub more_skills_text: String,
}

#[derive(Debug, Deserialize)]
pub struct Experience {
    pub title: String,
    pub items: Vec<ExperienceItem>,
}

#[derive(Debug, Deserialize)]
pub struct ExperienceItem {
    pub title: String,
    pub company: String,
    pub period: String,
    pub description: String,
    #[serde(default)]
    pub technologies: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Projects {
    pub title: String,
    pub description: String,
    #[serde(rename = "viewLive")]
    pub view_live: String,
    #[serde(rename = "viewCode")]
    pub view_code: String,
    #[serde(rename = "viewDocs")]
    pub view_docs: String,
    #[serde(rename = "viewDetails")]
    pub view_details: String,
    #[serde(rename = "backToProjects")]
    pub back_to_projects: String,
    #[serde(rename = "searchPlaceholder")]
    pub search_placeholder: String,
    #[serde(rename = "statusAll")]
    pub status_all: String,
    #[serde(rename = "technologyAll")]
    pub technology_all: String,
    #[serde(rename = "noProjects")]
    pub no_projects: String,
    #[serde(rename = "noProjectsHint")]
    pub no_projects_hint: String,
    #[serde(rename = "resetFilters")]
    pub reset_filters: String,
    #[serde(rename = "technologiesTitle")]
    pub technologies_title: String,
    #[serde(rename = "highlightsTitle")]
    pub highlights_title: String,
    pub items: Vec<ProjectItem>,
}

#[derive(Debug, Deserialize)]
pub struct ProjectItem {
    pub id: String,
    pub title: String,
    pub description: String,
    #[serde(rename = "extendedDescription")]
    pub extended_description: String,
    #[serde(default)]
    pub technologies: Vec<String>,
    pub image: String,
    #[serde(rename = "liveUrl", default)]
    pub live_url: Option<String>,
    #[serde(rename = "codeUrl", default)]
    pub code_url: Option<String>,
    #[serde(rename = "docsUrl", default)]
    pub docs_url: Option<String>,
    pub status: String,
    #[serde(rename = "statusLabel")]
    pub status_label: String,
    pub date: String,
}

#[derive(Debug, Deserialize)]
pub struct Contact {
    pub title: String,
    pub description: String,
    pub intro: String,
    #[serde(rename = "channelHeading")]
    pub channel_heading: String,
    #[serde(rename = "formChannel")]
    pub form_channel: String,
    #[serde(rename = "formChannelHint")]
    pub form_channel_hint: String,
    pub form: ContactForm,
}

#[derive(Debug, Deserialize)]
pub struct ContactForm {
    pub name: String,
    pub email: String,
    pub subject: String,
    pub message: String,
    pub send: String,
    pub sending: String,
    pub success: String,
    pub error: String,
}

#[derive(Debug, Deserialize)]
pub struct Footer {
    pub impressum: String,
    pub privacy: String,
    pub copyright: String,
    pub contact: String,
}

#[derive(Debug, Deserialize)]
pub struct Social {
    pub email: SocialLink,
    pub imessage: SocialLink,
    pub github: SocialLink,
    pub linkedin: SocialLink,
}

#[derive(Debug, Deserialize)]
pub struct SocialLink {
    pub href: String,
    pub label: String,
}

#[derive(Debug, Deserialize)]
pub struct Impressum {
    pub title: String,
    pub sections: Vec<ImpressumSection>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ImpressumSection {
    Contact { title: String, lines: Vec<String> },
    Text { title: String, content: String },
}

#[derive(Debug, Deserialize)]
pub struct Privacy {
    pub title: String,
    pub sections: Vec<PrivacySection>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum PrivacySection {
    Subsection {
        title: String,
        subsections: Vec<PrivacySubsection>,
    },
    Text {
        title: String,
        content: String,
    },
    TextList {
        title: String,
        content: String,
        items: Vec<String>,
    },
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum PrivacySubsection {
    Text {
        title: String,
        content: String,
    },
    TextList {
        title: String,
        content: String,
        items: Vec<String>,
    },
}

#[derive(Debug, Deserialize)]
pub struct StructuredData {
    pub person: StructuredPerson,
    pub website: StructuredSite,
    pub portfolio: StructuredPortfolio,
}

#[derive(Debug, Deserialize)]
pub struct StructuredPerson {
    pub name: String,
    #[serde(rename = "jobTitle")]
    pub job_title: String,
    pub description: String,
    pub image: String,
    #[serde(rename = "sameAs", default)]
    pub same_as: Vec<String>,
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct StructuredSite {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct StructuredPortfolio {
    pub name: String,
    pub description: String,
    #[serde(rename = "dateCreated")]
    pub date_created: String,
    pub genre: String,
    #[serde(default)]
    pub about: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct NotFound {
    pub title: String,
    pub message: String,
    pub back: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_all_configured_locales() {
        let i18n = I18n::load(
            &["en-US".to_string(), "de-DE".to_string()],
            "en-US",
        )
        .expect("translations should load");
        assert_eq!(i18n.locales().len(), 2);
        assert!(i18n.has("en-US"));
        assert!(i18n.has("de-DE"));
    }

    #[test]
    fn unknown_locale_falls_back_to_default() {
        let i18n = I18n::load(&["en-US".to_string()], "en-US").unwrap();
        let m = i18n.get("xx-XX");
        assert!(!m.navigation.home.is_empty());
    }

    #[test]
    fn og_locale_uses_underscore() {
        assert_eq!(I18n::og_locale("en-US"), "en_US");
        assert_eq!(I18n::og_locale("de-DE"), "de_DE");
    }

    #[test]
    fn missing_default_locale_is_rejected() {
        let result = I18n::load(&["en-US".to_string()], "fr-FR");
        assert!(result.is_err());
    }

    #[test]
    fn en_us_catalogue_has_all_pages() {
        let i18n = I18n::load(&["en-US".to_string()], "en-US").unwrap();
        let m = i18n.get("en-US");
        assert!(!m.hero.title.is_empty());
        assert!(!m.about.bio.content.is_empty());
        assert!(!m.projects.items.is_empty());
        assert!(!m.contact.form.send.is_empty());
        assert!(!m.privacy.sections.is_empty());
        assert!(!m.impressum.sections.is_empty());
    }
}
