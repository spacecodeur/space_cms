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
            tagline: "Explorez l'actualité IT, apprenez par la pratique et échangez avec nos experts passionnés !".to_string(),
            description: "Une plateforme de gestion de contenu permettant de partager des connaissances et d'offrir des services de consultation".to_string(),
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