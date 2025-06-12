use leptos::prelude::*;
use crate::pages::{HomePage, NotFoundPage};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppRoutes {
    Home,
    NotFound,
}

impl AppRoutes {
    pub fn component(&self) -> Box<dyn IntoView> {
        match self {
            AppRoutes::Home => Box::new(view! { <HomePage /> }),
            AppRoutes::NotFound => Box::new(view! { <NotFoundPage /> }),
        }
    }
}