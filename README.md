# actix-gen

Generates `api`, `model`, and `ops` Actix boilerplate for a REST API.

Set the `TEMPLATES_PATH` environment variable to the templates directory path (where `api.txt`, `model.txt`, and `ops.txt` are held).

### Build
`cargo build --release`

## Usage
`actix-gen all <model> <table>` (e.g., `actix-gen all status statuses`)