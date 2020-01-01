# Todo Example

## Frontend

Created using [VueJs](https://vuejs.org/);

Application styling provided borrowed from the [TodoMVC Example](https://vuejs.org/v2/examples/todomvc.html) by [Evan You](http://evanyou.me/)

## Backend

This Todo application's backend has been implemented using [Rust programming language](https://rust-lang.org/). Some of the crates from [crates.io](https://crates.io/) used include:

[actix-web](https://crates.io/crates/actix-web)
[juniper](https://crates.io/crates/juniper)
[mongodb](https://crates.io/crates/mongodb)

You can find a comprehensive list in the _backend/Cargo.toml_ file.

## Goal

The goal of this example is to implement a simple Todo application that utilizes a [GraphQL API](https://graphql.org/) implemented using [Rust's](https://rust-lang.org/) [actix-web](https://crates.io/crates/actix-web) and [juniper](https://crates.io/crates/juniper) with the data being persisted to a [MongoDB](https://www.mongodb.com/) database.

The applications demonstrates:

- Setting up a [GraphQL API](https://graphql.org/) on an [actix-web](https://crates.io/crates/actix-web) application using [juniper](https://crates.io/crates/juniper).
- Handling user sessions using the [actix-session](https://crates.io/crates/actix-session) crate.
- Serving your static application using the [actix-files](https://crates.io/crates/actix-files) crate.
- Persisting your application data using the [mongodb](https://crates.io/crates/mongodb) crate.

**Disclaimer:** This repository is created for learning purposes and is by no means meant to be a fully fledged application. That said, feel free to suggest improvements to all implementations herein.
