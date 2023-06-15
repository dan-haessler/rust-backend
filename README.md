# Rust Backend
This project is made for pure practice purposes, it represents a backend with database connectivity, migrations and a Rest API, with the possibility to virtualize it using Docker.

There is no application purpose for this repository, it is just to use the Rust macros in a real context and to identify and solve the difficulties.

With Actix as the web framework and Diesel as the ORM providing a DSL for the data model in SQL, we have a powerful stack with Deadpool taking care of connection pooling. Database access takes place synchronously. There is also diesel-async, but it is cumbersome to work with and moreover the sychnronicity of diesel is not a performance problem. At least not for most services, even Crates.io uses a full sync stack including diesel under the hood.

Together with Serde as a serialization library, the web server should easily manage several thousand requests/transactions per second in a "normal" usecase on "normal" hardware.

# Start
You can build and start the application with db like this:
```
docker-compose up --build
```

**NOTE** Docker may not recognize the name of your architecture correctly, so there is an additional argument that can be specified, e.g. for Windows it is necessary to specify the following:

```
build:
  context: .
  args:
    - ARCH=x86_64
```

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