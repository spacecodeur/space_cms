# Space CMS

A modular, server-side rendered CMS built with Leptos and Rust, designed to serve as a foundation for multiple projects.

## Overview

Space CMS is a modern content management system that leverages the power of Rust and the Leptos framework to provide a fast, type-safe, and modular architecture. The project is structured as a workspace with clear separation between frontend and backend concerns.

## Architecture

The project follows a clean architecture pattern with three main crates:

- **`app/`** - The main SSR application that orchestrates frontend and backend
- **`crates/frontend/`** - Pure UI components and pages
- **`crates/backend/`** - Server-side logic, API routes, and services  
- **`crates/shared/`** - Shared types and models used by both frontend and backend

## Prerequisites

- Rust 1.75+ with nightly toolchain
- cargo-leptos: `cargo install cargo-leptos --locked`
- sass/dart-sass for SCSS compilation

## Getting Started

### Development

```bash
# Clone the repository
git clone <your-repo-url>
cd space_cms

# Configure git hooks
git config core.hooksPath .git-hooks

# Install dependencies and build
cargo build

# Run the development server
cd app
cargo leptos serve
```

The application will be available at `http://127.0.0.1:3000`

### Building for Production

```bash
cd app
cargo leptos build --release
```

## Project Structure

```
space_cms/
├── app/                    # Main SSR application
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs        # Server entry point
│       ├── lib.rs         # WASM hydration
│       └── app.rs         # App shell and routing
├── crates/
│   ├── frontend/          # Frontend components
│   │   └── src/
│   │       ├── components/
│   │       └── pages/
│   ├── backend/           # Backend services
│   │   └── src/
│   │       ├── api/       # API routes
│   │       └── services/  # Business logic
│   └── shared/            # Shared types
│       └── src/
│           └── models.rs  # Data models
├── style/                 # SCSS styles
└── public/                # Static assets
```

## Features

- **Server-Side Rendering (SSR)** - Full SSR support with hydration
- **Modular Architecture** - Clear separation of concerns
- **Type Safety** - Leverages Rust's type system
- **Hot Reloading** - Development server with automatic reload
- **API Integration** - Built-in API routes alongside SSR pages

## API Endpoints

- `GET /api/health` - Health check endpoint

## Extending the CMS

### Adding a New Page

1. Create a new component in `crates/frontend/src/pages/`
2. Export it from the pages module
3. Add a route in `app/src/app.rs`

### Adding an API Endpoint

1. Create a new route handler in `crates/backend/src/api/`
2. Add it to the router in `routes.rs`

### Adding Shared Types

1. Define new types in `crates/shared/src/models.rs`
2. They'll be available in both frontend and backend crates

## Technology Stack

- **Leptos 0.8.0** - Full-stack Rust framework
- **Axum** - Web server framework
- **Tokio** - Async runtime
- **WASM** - Client-side interactivity

## Testing

```bash
cargo leptos end-to-end
```

Tests are located in the `end2end/tests` directory.

## Deployment

After building for production with `cargo leptos build --release`:

1. Copy the server binary from `target/server/release`
2. Copy the `site` directory from `target/site`
3. Set the required environment variables:
   ```sh
   export LEPTOS_OUTPUT_NAME="space_cms"
   export LEPTOS_SITE_ROOT="site"
   export LEPTOS_SITE_PKG_DIR="pkg"
   export LEPTOS_SITE_ADDR="127.0.0.1:3000"
   export LEPTOS_RELOAD_PORT="3001"
   ```
4. Run the server binary

## License

This project is released under the Unlicense. You should replace the LICENSE for your own application with an appropriate license if you plan to release it publicly.
