# Web Service - Space CMS

## Architecture

Cette application web utilise une architecture modulaire suivant les meilleures pratiques de Leptos et du développement web moderne (2025).

### Structure des dossiers

```
src/
├── main.rs              # Point d'entrée du serveur Axum
├── lib.rs              # Exports publics de la bibliothèque
├── app.rs              # Composant App principal
├── components/         # Composants réutilisables
│   ├── mod.rs
│   ├── layout/         # Composants de layout
│   │   ├── mod.rs
│   │   ├── header.rs   # En-tête du site
│   │   ├── footer.rs   # Pied de page
│   │   └── shell.rs    # Shell HTML principal
│   └── home/           # Composants spécifiques à l'accueil
│       ├── mod.rs
│       └── hero.rs     # Section hero
├── pages/              # Pages/Routes
│   ├── mod.rs
│   ├── home.rs         # Page d'accueil
│   └── not_found.rs    # Page 404
└── server/             # Configuration serveur
    ├── mod.rs
    └── config.rs       # Configuration du serveur

style/                  # Assets CSS
└── main.css           # Styles principaux (CSS moderne avec variables)

public/                 # Assets statiques (futur)
└── favicon.ico
```

## Principes appliqués

### Séparation des responsabilités
- **Composants** : Logique d'affichage réutilisable
- **Pages** : Assemblage de composants pour des routes spécifiques
- **Server** : Configuration et logique serveur
- **Styles** : CSS séparé avec variables modernes

### Standards modernes (2025)
- **CSS Variables** : Système de design cohérent
- **Composants modulaires** : Facilite la maintenance
- **Structure Leptos** : Suit les conventions de la communauté
- **TypeScript-like organization** : Structure claire et prévisible

## Technologies

- **Leptos 0.8.2** : Framework web réactif Rust
- **Axum 0.8** : Serveur web asynchrone
- **CSS moderne** : Variables, Flexbox, Grid
- **Docker** : Conteneurisation

## Développement

```bash
# Compiler et lancer
cargo run

# Le serveur démarre sur http://0.0.0.0:3000
```

## Prochaines étapes

1. **Routage complet** : Intégrer leptos_router pour la navigation
2. **Hydratation** : Activer le SSR complet avec hydratation côté client
3. **Base de données** : Intégrer avec le service PostgreSQL
4. **API** : Développer les endpoints pour les articles et rendez-vous
5. **Tests** : Ajouter des tests unitaires et d'intégration

Cette structure est extensible et permet d'ajouter facilement de nouvelles fonctionnalités tout en maintenant la séparation des responsabilités.