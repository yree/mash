use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;

mod views {
    use maud::{html, Markup};

    /// Renders the form using Maud
    pub fn render_form() -> Markup {
        html! {
            (maud::DOCTYPE)
            html {
                head {
                    meta charset="UTF-8";
                    meta name="viewport" content="width=device-width, initial-scale=1.0";
                    link rel="stylesheet" href="https://yree.io/mold/assets/css/main.css";
                    link rel="icon" href="data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 100'><text y='.9em' font-size='90'>🥔</text></svg>";
                    link rel="preconnect" href="https://fonts.googleapis.com";
                    link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="";
                    link href="https://fonts.googleapis.com/css2?family=JetBrains+Mono:ital,wght@0,100..800;1,100..800&amp;display=swap" rel="stylesheet";
                    title { "A mash demo 🥔" }
                    script src="https://unpkg.com/htmx.org" {}
                }
                body a="auto" {
                    main class="content" aria-label="Content" {
                        div class="w" id="markdown-view" _="on load call MathJax.typeset()" {
                            h1 { "Mash 🥔" }
                            p { "A simple demo using the mash stack." }
                            h2 { "What's your name?" }
                            form hx-post="/submit" hx-target="#response" {
                                div class="grid" {
                                    input
                                        type="text"
                                        id="name"
                                        name="name"
                                        placeholder="Enter your name"
                                        required;
                                    button type="submit" { "Submit" }
                                }
                            }
                            br;
                            div id="response" {
                                p { "Hello world!" }
                            }
                        }
                    }
                }
                footer {
                    div class="w" {
                        p { a href="https://yree.io/mash" { "mash" } " 🥔 :: a " a href="https://yree.io" { "Yree" } " stack ♥" }
                    }
                }
            }
        }
    }

    /// Renders the response when a form is submitted
    pub fn render_response(name: &str) -> Markup {
        html! {
            p { "Hello, " (name) "!" }
        }
    }
}

mod handlers {
    use crate::views::{render_form, render_response};
    use axum::{
        extract::Form,
        response::{Html, IntoResponse},
    };
    use serde::Deserialize;

    // Define a form data structure using Serde
    #[derive(Deserialize)]
    pub struct NameForm {
        pub name: String,
    }

    // Handle GET request to render the form
    pub async fn get_form() -> impl IntoResponse {
        Html(render_form().into_string())
    }

    // Handle POST request when form is submitted
    pub async fn handle_submit(Form(form): Form<NameForm>) -> impl IntoResponse {
        Html(render_response(&form.name).into_string())
    }
}

#[tokio::main]
async fn main() {
    // Define routes
    let app = Router::new()
        .route("/", get(handlers::get_form))
        .route("/submit", post(handlers::handle_submit));

    // Set up the server
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("Listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
