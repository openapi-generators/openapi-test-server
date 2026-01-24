## 0.2.2 (2026-01-24)

### Fixes

- update rust crate tracing-subscriber to v0.3.20 (#22)
- update rust crate time to v0.3.43 (#24)
- update rust crate time to v0.3.44 (#27)
- update rust crate serde to v1.0.226 (#25)

## 0.2.1 (2025-06-03)

### Features

- Add vec of scalars and JSON-serialized objects to multipart test

## 0.2.0 (2025-05-31)

### Breaking Changes

- Revamp test server to use Actix, update CI stuff, add multi-files to multipart test

### Features

- Tests for headers
- Slim down final Docker image
- Create a server which can test multipart/form uploads
