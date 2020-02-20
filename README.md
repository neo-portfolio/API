# API

# Clone

    git clone git@github.com:neo-portfolio/API.git

# Installation

# Install Rust (+ cargo)

Follow instructions on [the official website.](https://www.rust-lang.org/tools/install)

    
## Compile

### Debug

    cargo run --color=always --package api --bin api
    
### Production
    
    cargo build --release
    
    
# Run

    cargo run
    
# .env

    NEO4J_URL=
    NEO4J_PORT=
    NEO4J_USER=
    NEO4J_PASSWORD=
    NEO4J_DATABASE=
    ALLOWED_ORIGIN=