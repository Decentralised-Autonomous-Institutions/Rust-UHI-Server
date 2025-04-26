# Universal Health Interface (UHI) Protocol Implementation

This repository contains a Rust implementation of the Universal Health Interface (UHI) Protocol, which aims to standardize healthcare interactions between patients, healthcare providers, and other stakeholders in the healthcare ecosystem.

## Project Structure

The project is organized as follows:

- **src/** - Source code directory
  - **config.rs** - Configuration module for loading settings
  - **errors.rs** - Error handling definitions
  - **handlers/** - HTTP request handlers
  - **logging.rs** - Logging configuration
  - **routes.rs** - API route definitions
  - **main.rs** - Application entry point

- **config/** - Configuration files
  - **default.toml** - Default configuration
  - **development.toml** - Development environment configuration

- **docs/** - Documentation
  - **tasks.md** - Implementation tasks
  - **status.md** - Project status

## Getting Started

### Prerequisites

- Rust toolchain (latest stable)
- PostgreSQL database

### Setup

1. Clone the repository:
   ```
   git clone https://github.com/yourusername/rust-uhi.git
   cd rust-uhi
   ```

2. Copy the example env file:
   ```
   cp env.example .env
   ```

3. Modify the `.env` file with your configuration.

4. Build the project:
   ```
   cargo build
   ```

5. Run the project:
   ```
   cargo run
   ```

## Features

- **UHI Protocol Compliance**: Full implementation of the UHI protocol specification
- **Modular Architecture**: Well-organized code with clear separation of concerns
- **Configurable**: Environment-based configuration for different deployment scenarios
- **Structured Logging**: Comprehensive logging for observability
- **Error Handling**: Robust error handling and reporting

## API Endpoints

- `/api/v1/search` & `/api/v1/on_search` - Discovery of healthcare services
- `/api/v1/select` & `/api/v1/on_select` - Selection of specific services
- `/api/v1/init` & `/api/v1/on_init` - Initialization of service booking
- `/api/v1/confirm` & `/api/v1/on_confirm` - Confirmation of service booking
- `/api/v1/status` & `/api/v1/on_status` - Status checking of booked services
- `/api/v1/networkregistry/lookup` - Network registry for provider discovery

## Development Status

This project is in active development. See [docs/status.md](docs/status.md) for current progress.

## License

This project is licensed under the MIT License - see the LICENSE file for details. 