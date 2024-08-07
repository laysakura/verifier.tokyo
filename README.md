# verifier.tokyo

Decode and verify Verifiable Credentials, like jwt.io

## Implementation Details

This project is a static web site that decodes and verifies Verifiable Credentials. The initial version fetches content from `https://example.com/` and displays it in a `<div>`.

### Tools

- Static web site
  - GitHub Pages
  - WASM

- Core logics
  - Rust
  - Cargo

- Frontend
  - TypeScript
  - HTML / CSS

### Web design

- Developer-friendly
- Easy to maintain
- Interactive (use Fetch API)

## Usage Instructions

### Prerequisites

- Install [Rust](https://www.rust-lang.org/tools/install)
- Install [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

### Building the Project

1. Clone the repository:

   ```sh
   git clone https://github.com/laysakura/verifier.tokyo.git
   cd verifier.tokyo
   ```

2. Build the Rust project:

   ```sh
   wasm-pack build
   ```

3. Serve the static files (you can use any static file server, here we use `http-server`):

   ```sh
   npm install -g http-server
   http-server
   ```

4. Open your browser and navigate to `http://localhost:8080` to see the site in action.
