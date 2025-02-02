# mash 🥔

**as simple as potatoes**

Welcome to **MASH** - a minimalist web development stack that combines the power of **m**aud, **a**xum, **s**qlx, and **h**tmx.

## Why mash? 🤔

- **Maud**: A blazing-fast Rust-based templating engine that lets you write HTML directly in Rust, ensuring type safety and avoiding common HTML errors.
- **Axum**: A powerful web framework built on Tokio that handles routing and request management with ease and speed.
- **SQLx**: An async, pure Rust SQL crate for interacting with databases, offering compile-time query verification, which ensures you avoid runtime SQL errors.
- **HTMX**: A JavaScript library that enables interaction between the client and server without writing JavaScript—using HTML attributes like `hx-post` and `hx-target`.

MASH allows for rapid prototyping with simple UI elements while maintaining performance and ease of development.

## Hello World! 🌍

In this [example](https://github.com/yree/mash), we build a basic web application where the user inputs their name, and the server responds with a greeting. The app leverages HTMX to dynamically update the page without needing a full reload, Maud for HTML generation, Axum for request routing, and SQLx for database interaction.

<button onclick="window.open('https://mash.yree.io', '_blank')">See mash demo 🥔</button>

The example includes:
- `handlers.rs`: Contains the logic for handling HTTP requests. In this example, it renders the form and handles form submissions.
- `main.rs`: The entry point for the Axum web server, routing requests to the handlers.
- `views.rs`: Defines the HTML templates using Maud for both the form and response.

To continuously build and run the app use `cargo-watch`:

```bash
cargo watch -x "run --example hello_world"
```

Access the app at `http://localhost:8080`.

### Breaking it down 🔍

#### **Axum** - The Web Framework 🌐
Axum is a web framework that uses an asynchronous model, ideal for handling web traffic efficiently. In this example, **Axum** provides:
- **Routing**: Handling `GET` and `POST` requests to serve the form and process the submission.
- **Extractors**: Handling form data extraction using Serde's `Form`.

```rust
.route("/", get(handlers::get_form))
.route("/submit", post(handlers::handle_submit))
```

#### **Maud** - HTML Templating in Rust 🦀
**Maud** allows you to write HTML using Rust's type-safe syntax. This eliminates any risk of invalid HTML while keeping your code minimal and clean. Maud renders the form and the dynamic greeting.

```rust
html! {
    div class="w" {
        h1 { "What's your name!" }
        form hx-post="/submit" hx-target="#response" {
            input type="text" id="name" name="name" required;
            button { "Submit" }
        }
    }
}
```

#### **HTMX** - Simple Dynamic Interactions ⚡
**HTMX** simplifies AJAX-like requests by using HTML attributes. In this example, `hx-post` sends a POST request when the form is submitted, and `hx-target` updates the content of a specific DOM element (in this case, the `#response` div) without needing custom JavaScript.

```html
<form hx-post="/submit" hx-target="#response">
```

#### **SQLx** - Interacting with Databases 🗂
**SQLx** is used to handle database queries in an async, safe manner. This example demonstrates how you can use SQLx to interact with your database to store and retrieve data.

```rust
use sqlx::PgPool;

async fn handle_submit(pool: PgPool, form: FormData) -> Result<impl IntoResponse, AppError> {
    sqlx::query("INSERT INTO greetings (name) VALUES ($1)")
        .bind(form.name)
        .execute(&pool)
        .await?;
    Ok("Form submitted!")
}
```

## Why you'll love MASH ❤

- **Minimalist**: The stack is designed to be lightweight and efficient. Maud ensures your HTML is clean and type-safe, while HTMX eliminates the need for complex JavaScript.
- **Rapid Prototyping**: Thanks to Axum’s easy routing, HTMX’s seamless client-server interaction, and SQLx’s database access, you can quickly prototype interactive web applications.
- **Safe Database Interaction**: SQLx brings compile-time query verification and asynchronous database queries to your project, ensuring your database interactions are both efficient and error-free.
- **Performance**: By using Rust and asynchronous programming via Tokio (used by Axum), the app remains highly performant even under load.

It's plain, versatile, and gets the job done effectively.

With the MASH stack, you get a straightforward, efficient, and minimalist approach to web development. Perfect for fast prototyping and small-scale applications, it's **as simple as potatoes**! 🥔

