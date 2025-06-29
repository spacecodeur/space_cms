# Authentication

Space CMS uses JWT (JSON Web Token) based authentication for secure API access.

## Features

- JWT token generation and validation
- Password hashing with bcrypt
- Login and registration endpoints
- Protected routes with middleware

## API Endpoints

### Public Endpoints

#### Register
```
POST /api/auth/register
Content-Type: application/json

{
  "username": "john_doe",
  "email": "john@example.com",
  "password": "secure_password"
}

Response:
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "user": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "username": "john_doe",
    "email": "john@example.com",
    "created_at": "2024-01-01T00:00:00Z",
    "updated_at": "2024-01-01T00:00:00Z"
  }
}
```

#### Login
```
POST /api/auth/login
Content-Type: application/json

{
  "username": "john_doe",
  "password": "secure_password"
}

Response:
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "user": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "username": "john_doe",
    "email": "john@example.com",
    "created_at": "2024-01-01T00:00:00Z",
    "updated_at": "2024-01-01T00:00:00Z"
  }
}
```

### Protected Endpoints

All protected endpoints require the Authorization header:
```
Authorization: Bearer <your-jwt-token>
```

Example:
```
GET /api/protected
Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...

Response: "This is a protected endpoint"
```

## Configuration

Set the JWT secret in your `.env` file:
```
JWT_SECRET=your-very-secure-secret-key
```

## Security Notes

1. Always use HTTPS in production
2. Use a strong, random JWT secret
3. Tokens expire after 24 hours
4. Passwords are hashed using bcrypt with default cost
5. Never expose password hashes to the client

## Implementation Notes

The authentication system is modular with separate concerns:
- `auth/jwt.rs` - JWT token creation and validation
- `auth/password.rs` - Password hashing and verification
- `auth/middleware.rs` - Request authentication middleware
- `auth/routes.rs` - Login and registration endpoints