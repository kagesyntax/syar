use dioxus::document::eval;
use dioxus::prelude::*;

mod components;
mod models;
mod pages;
mod router;

use models::WishlistItem;
use router::Route;

// ─── Design System Assets ───────────────────────────────────
const TAILWIND_CSS: Asset = asset!("assets/tailwind.css");

// ─── Entry Point ────────────────────────────────────────────

fn main() {
    dioxus::launch(App);
}

// ─── App Root ───────────────────────────────────────────────

#[component]
fn App() -> Element {
    let mut dark = use_signal(|| false);

    // Read saved theme preference from localStorage on mount so Rust state is correct
    use_effect(move || {
        spawn(async move {
            let result = eval(r#"
                (localStorage.getItem("theme") === "dark" ||
                (!localStorage.getItem("theme") && window.matchMedia("(prefers-color-scheme: dark)").matches)) ? "true" : "false"
            "#).await;

            if let Ok(val) = result {
                if val.as_str() == Some("true") {
                    dark.set(true);
                }
            }
        });
    });

    use_context_provider(|| dark);
    use_context_provider(|| Signal::new(Vec::<WishlistItem>::new()));

    rsx! {
        document::Stylesheet { href: TAILWIND_CSS }
        document::Link { rel: "preconnect", href: "https://fonts.googleapis.com" }
        document::Link { rel: "preconnect", href: "https://fonts.gstatic.com", crossorigin: "anonymous" }
        document::Link {
            href: "https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&display=swap",
            rel: "stylesheet",
        }
        Router::<Route> {}
    }
}
