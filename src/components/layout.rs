use crate::components::Navbar;
use crate::models::{SearchOpen, Theme, WishlistItem};
use crate::router::Route;
use dioxus::document::eval;
use dioxus::prelude::*;

#[component]
pub fn Shell() -> Element {
    let theme = use_context::<Signal<Theme>>();
    let wishlist = use_context::<Signal<Vec<WishlistItem>>>();
    let wishlist_count = wishlist.read().len();
    let route: Route = use_route();

    // Search state - provided at shell level so navbar and pages can access
    let search_q = use_signal(String::new);
    let search_open = use_signal(|| SearchOpen(false));
    use_context_provider(|| search_q);
    use_context_provider(|| search_open);

    // Scroll to top on route change
    use_effect(move || {
        let _ = route.clone();
        let _ = eval("window.scrollTo({top:0,behavior:'instant'})");
    });

    // Sync dark class onto <html> so body CSS variables + base styles update too
    use_effect(move || {
        let is_dark = theme().0;
        spawn(async move {
            let _ = eval(if is_dark {
                "document.documentElement.classList.add('dark')"
            } else {
                "document.documentElement.classList.remove('dark')"
            })
            .await;
        });
    });

    rsx! {
        div { class: if theme().0 { "dark" } else { "" },
            div {
                class: "min-h-screen bg-white dark:bg-[#0A0A0A] text-[#1D1D1F] dark:text-[#F5F5F7] font-[Inter,system-ui,sans-serif] antialiased",
                Navbar { wishlist_count }
                main { class: "flex-1 pb-16 md:pb-0",
                    Outlet::<Route> {}
                }
                div { class: "hidden md:block",
                    Footer {}
                }
            }
        }
    }
}

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer {
            class: "border-t border-[#E5E5E7] dark:border-[#2C2C2E] bg-white dark:bg-[#0A0A0A] mt-auto",
            div {
                class: "mx-auto flex max-w-7xl flex-col items-center justify-between gap-4 px-6 py-10 text-xs text-[#1D1D1F]/50 dark:text-[#F5F5F7]/50 sm:flex-row",
                div { class: "flex items-center gap-2",
                    span { class: "font-semibold tracking-wide text-[#1D1D1F] dark:text-[#F5F5F7] uppercase text-[10px]", "DAD'S STORE" }
                    span { "© 2025 — Crafted in Lagos" }
                }
                nav { class: "flex items-center gap-5",
                    a { class: "transition-colors duration-200 hover:text-[#1D1D1F] dark:hover:text-[#F5F5F7]", href: "#", "Privacy" }
                    a { class: "transition-colors duration-200 hover:text-[#1D1D1F] dark:hover:text-[#F5F5F7]", href: "#", "Terms" }
                    a { class: "transition-colors duration-200 hover:text-[#1D1D1F] dark:hover:text-[#F5F5F7]", href: "#", "Contact" }
                }
            }
        }
    }
}

#[component]
pub fn NotFound(segments: Vec<String>) -> Element {
    let nav = use_navigator();
    let path = format!("/{}", segments.join("/"));

    rsx! {
        div { class: "flex min-h-[70vh] items-center justify-center px-6",
            div { class: "text-center",
                div {
                    class: "mx-auto flex h-20 w-20 items-center justify-center bg-[#F5F5F7] dark:bg-[#1C1C1E] text-[#1D1D1F] dark:text-[#F5F5F7] text-2xl font-bold",
                    "404",
                }
                h1 { class: "mt-6 text-2xl font-semibold tracking-tight text-[#1D1D1F] dark:text-[#F5F5F7]",
                    "Page not found"
                }
                p { class: "mt-2 text-sm text-[#1D1D1F]/50 dark:text-[#F5F5F7]/50",
                    "Nothing at "
                    code { class: "bg-[#F5F5F7] dark:bg-[#1C1C1E] px-1.5 py-0.5 font-mono text-xs", "{path}" }
                }
                button {
                    class: "mt-8 inline-flex h-12 items-center justify-center bg-[#0071E3] dark:bg-[#0A84FF] px-8 text-sm font-medium text-white transition-all duration-200 hover:-translate-y-1 hover:shadow-xl active:scale-[0.98]",
                    onclick: move |_| {
                        nav.push(Route::Home {});
                    },
                    "Go home"
                }
            }
        }
    }
}
