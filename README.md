# Rust Backend
Simple example rust backend using
* Actix as web framework
* Diesel as ORM (using embedded migrations)
* Deadpool for connection pooling
* Serde

# Metaprogramming (TODO)
the `/meta` folder is dedicated to metaprogramming.

The following should be checked for suitability
* Proc macros
    * Attribute-like macros
    * Function-like macros
    * Custom `#[derive]` macros
* Declarative macros

For intuitive usage the `/meta` Rust projects are structured as followed:

* `/meta` bundle trait/macros
* `/meta/trait` defining the traits which are implemented by macro
* `/meta/derive` containing macro implementations