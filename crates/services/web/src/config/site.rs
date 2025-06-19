#[derive(Debug, Clone)]
pub struct SiteConfig {
    pub name: String,
    pub title: String,
    pub tagline: String,
    pub description: String,
    pub locale: String,
    pub date_format: String,
    pub ui_strings: UIStrings,
}

#[derive(Debug, Clone)]
pub struct UIStrings {
    pub search_placeholder: String,
    pub content_type_latest: String,
    pub content_type_popular: String,
    pub page_not_found: String,
}

impl Default for SiteConfig {
    fn default() -> Self {
        Self {
            name: "Kunkodio".to_string(),
            title: "Kunkodio - Learn IT with Practice".to_string(),
            tagline: "Learn IT with Practice".to_string(),
            description: "A platform for IT learning through practical workshops and articles".to_string(),
            locale: "fr".to_string(),
            date_format: "%d/%m/%Y".to_string(),
            ui_strings: UIStrings::default(),
        }
    }
}

impl Default for UIStrings {
    fn default() -> Self {
        Self {
            search_placeholder: "Rechercher...".to_string(),
            content_type_latest: "Nouveautés".to_string(),
            content_type_popular: "Populaires".to_string(),
            page_not_found: "Page not found.".to_string(),
        }
    }
}

impl SiteConfig {
    pub fn new() -> Self {
        Self::default()
    }
}