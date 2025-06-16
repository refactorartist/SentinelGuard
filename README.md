# SentinelGuard

## How to Use

### Prerequisites

- **Rust** (latest stable)
- **Devbox** (for reproducible development environments)
- **PostgreSQL** (running instance, for database operations)
- **[Optional] Docker** (for isolated PostgreSQL/testing)

### Setup

1. **Clone the Repository**
   ```sh
   git clone https://github.com/RunicEngines/SentinelGuard.git sentinal-guard
   cd sentinal-guard
   ```

2. **Install Devbox & Project Dependencies**
   ```sh
   devbox shell
   devbox run cargo fetch
   ```

3. **Configure Environment**
   - Copy `.env.example` to `.env` and fill in your configuration (especially database URI and secrets).
   - All secrets and sensitive data must be set via environment variables.

4. **Prepare the Database**
   - Ensure PostgreSQL is running and accessible.
   - The app will automatically create collections as needed.

### Running the Application

- **Start the App**
  ```sh
  devbox run app
  ```
  The server will start using the host and port specified in your `.env` file (see `.env.sample` for required variables).
  Be sure to set `BURAQ_HOST`, `BURAQ_PORT` and any other required variables in your environment file.

  The `DATABASE_URL` environment variable is by sqlx and rust-analyzer to check database queries.

### Development Workflow

- **Build:**  
  ```sh
  devbox cargo build
  ```
- **Run Tests:**  
  ```sh
  devbox run tests
  ```
- **Lint:**  
  ```sh
  devbox run cargo clippy
  ```
- **Format:**  
  ```sh
  devbox run cargo fmt
  ```

### Testing

- **Unit tests** are located alongside source code in `mod tests { ... }` blocks.
- **Integration tests** are in `tests/integration/`.
- All tests use async patterns and isolated test schemas (see `/tests/integration/repositories/fixtures/` for DB fixtures).
- To run all tests:
  ```sh
  devbox run tests
  ```

### API Usage

- The REST API is documented via OpenAPI/Swagger (see `/docs` endpoint when the server is running).
- **Swagger UI is available at [`http://localhost:8080/docs`](http://localhost:8080/docs) when the server is running.**
- All endpoints are async, return JSON, and require proper input validation.

### Logging & Configuration

- Logging is provided via the `tracing` crate.
- All configuration is handled by the `config` crate and environment variables.

### Security

- Never hardcode secrets—use environment variables.
- Use secure database URIs.
- Enforce HTTPS in production.
