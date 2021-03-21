# Rust + Svelte CRUD App

## Quick Start

Requires [Node.js](https://nodejs.org) and [Rust](https://www.rust-lang.org/tools/install)

Build client and dependencies

```
cd client
npm install
npm run build
cd ..
```

Compile and run the server

```
cargo run --release
```

## Tech Stack

* [Rust](https://www.rust-lang.org/) Back-end
  * [warp](https://github.com/seanmonstar/warp)
  * [SQLx](https://github.com/launchbadge/sqlx)
  * [SQLite](https://www.sqlite.org/index.html)
* [Svelte](https://svelte.dev/) Front-end
  * [Snowpack](https://www.snowpack.dev/)
  * [TypeScript](https://www.typescriptlang.org/)

## Dev

In one terminal instance, run the client in dev-mode

```
cd client
npm start
```

In another, run the server

```
cargo run
```