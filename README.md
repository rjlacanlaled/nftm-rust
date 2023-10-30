<div align="center">
        <h2 align="center" style="border-bottom: none">NFT Marketplace</h2>
        <img alt="Rust" src="https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white" />
        <img alt="Rust" src="https://img.shields.io/badge/tailwindcss-%23000000.svg?style=for-the-badge&logo=tailwindcss&logoColor=blue" />
                <img alt="Rust" src="https://img.shields.io/badge/htmx-%23000000.svg?style=for-the-badge&logo=htmx&logoColor=blue" />
</div>

 <hr />

## About
Recreating a Nextjs project using Rust, HTMX, Tailwindcss. 

## Quick Start (watch)
```sh
# Terminal 1 - To run server.
cargo watch -q -c -w src/ -w .cargo/ -x "run"

# Terminal 2 - To run examples
cargo watch -q -c -w examples/ -x "run --example quick_dev"
```

## Starting the DB
```sh
# Start postgesql server docker image:
docker run --rm --name pg -p 5432:5432 -e POSTGRES_PASSWORD=welcome postgres:15

# (optional) Run a psql terminal on pg. Open a new terminal tab:
docker exec -it -u postgres pg psql
```

## Unit Test (watch)
```sh
cargo watch -q -c -x "test -- --nocapture"

# Filter tests
cargo watch -q -c -x "test::model::task::tests::test_create_ok()"
```

## Status
WIP