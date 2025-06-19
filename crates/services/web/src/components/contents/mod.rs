use leptos::prelude::*;
use chrono::{DateTime, Utc};

#[derive(Clone, Debug, PartialEq)]
pub enum ContentType {
    Latest,
    Popular,
}

#[derive(Clone, Debug)]
pub struct Article {
    pub id: u32,
    pub title: String,
    pub teaser_image: String,
    pub author_avatar: String,
    pub author_name: String,
    pub date: DateTime<Utc>,
    pub views: u32,
}

impl PartialEq for Article {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id 
            && self.title == other.title 
            && self.teaser_image == other.teaser_image
            && self.author_avatar == other.author_avatar
            && self.author_name == other.author_name
            && self.date.timestamp() == other.date.timestamp()
            && self.views == other.views
    }
}

#[component]
pub fn Contents() -> impl IntoView {
    let selected_type = RwSignal::new(ContentType::Latest);
    
    // Hardcoded articles for now
    let articles = vec![
        Article {
            id: 1,
            title: "Introduction à Rust pour les développeurs web".to_string(),
            teaser_image: "/images/rust-intro.jpg".to_string(),
            author_avatar: "/images/avatar1.jpg".to_string(),
            author_name: "Alice Martin".to_string(),
            date: DateTime::parse_from_rfc3339("2024-01-15T10:00:00Z").unwrap().with_timezone(&Utc),
            views: 1250,
        },
        Article {
            id: 2,
            title: "Créer une API REST avec Axum et Leptos".to_string(),
            teaser_image: "/images/axum-api.jpg".to_string(),
            author_avatar: "/images/avatar2.jpg".to_string(),
            author_name: "Bob Dupont".to_string(),
            date: DateTime::parse_from_rfc3339("2024-01-14T14:30:00Z").unwrap().with_timezone(&Utc),
            views: 3200,
        },
        Article {
            id: 3,
            title: "Les meilleures pratiques pour structurer une application Leptos".to_string(),
            teaser_image: "/images/leptos-structure.jpg".to_string(),
            author_avatar: "/images/avatar3.jpg".to_string(),
            author_name: "Claire Bernard".to_string(),
            date: DateTime::parse_from_rfc3339("2024-01-13T09:15:00Z").unwrap().with_timezone(&Utc),
            views: 2100,
        },
    ];

    let filtered_articles = Memo::new(move |_| {
        let mut sorted_articles = articles.clone();
        match selected_type.get() {
            ContentType::Latest => {
                sorted_articles.sort_by(|a, b| b.date.cmp(&a.date));
            }
            ContentType::Popular => {
                sorted_articles.sort_by(|a, b| b.views.cmp(&a.views));
            }
        }
        sorted_articles
    });

    view! {
        <section class="contents">
            <TypeSelector selected_type=selected_type />
            <ArticlesList articles=Signal::from(filtered_articles) />
        </section>
    }
}

#[component]
fn TypeSelector(
    selected_type: RwSignal<ContentType>,
) -> impl IntoView {
    view! {
        <div class="type-selector">
            <button 
                class="type-selector__button"
                class:active=move || selected_type.get() == ContentType::Latest
                on:click=move |_| selected_type.set(ContentType::Latest)
            >
                "Nouveautés"
            </button>
            <button 
                class="type-selector__button"
                class:active=move || selected_type.get() == ContentType::Popular
                on:click=move |_| selected_type.set(ContentType::Popular)
            >
                "Populaires"
            </button>
        </div>
    }
}

#[component]
fn ArticlesList(
    #[prop(into)]
    articles: Signal<Vec<Article>>
) -> impl IntoView {
    view! {
        <div class="articles-list">
            <For
                each=move || articles.get()
                key=|article| article.id
                children=move |article| {
                    view! {
                        <ArticleCard article=article />
                    }
                }
            />
        </div>
    }
}

#[component]
fn ArticleCard(article: Article) -> impl IntoView {
    let formatted_date = article.date.format("%d/%m/%Y").to_string();
    let title = article.title.clone();
    let author_name = article.author_name.clone();
    
    view! {
        <article class="article-card">
            <img 
                class="article-card__image" 
                src=article.teaser_image 
                alt=title.clone()
            />
            <div class="article-card__content">
                <h3 class="article-card__title">{title}</h3>
                <div class="article-card__meta">
                    <img 
                        class="article-card__avatar" 
                        src=article.author_avatar 
                        alt=author_name.clone()
                    />
                    <span class="article-card__date">{formatted_date}</span>
                </div>
            </div>
        </article>
    }
}