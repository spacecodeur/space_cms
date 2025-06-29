# Database Configuration

Space CMS uses SQLx with SQLite for data persistence.

## Setup

1. Copy the environment file:
   ```bash
   cp .env.example .env
   ```

2. The default configuration uses SQLite with a local database file:
   ```
   DATABASE_URL=sqlite:space_cms.db
   ```

## Migrations

Migrations are automatically run when the server starts. To manage migrations manually:

### Install SQLx CLI
```bash
cargo install sqlx-cli --no-default-features --features sqlite
```

### Create a new migration
```bash
sqlx migrate add <migration_name>
```

### Run migrations manually
```bash
sqlx migrate run
```

### Revert migrations
```bash
sqlx migrate revert
```

## Database Schema

The initial schema includes:

- **users** - User accounts with id, username, email, and timestamps
- **posts** - Content posts with id, title, content, author_id, and timestamps

Both tables use UUID-like text IDs and include automatic timestamp updates.