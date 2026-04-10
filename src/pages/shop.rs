use crate::components::{FilterPill, ProductCard};
use crate::models::PRODUCTS;
use dioxus::prelude::*;

#[component]
pub fn Shop() -> Element {
    let mut active_filter = use_signal(|| "All".to_string());

    let categories = use_memo(move || {
        let mut cats: Vec<String> = vec!["All".to_string()];
        for p in PRODUCTS.iter() {
            let c = p.category.to_string();
            if !cats.contains(&c) {
                cats.push(c);
            }
        }
        cats
    });

    let filtered = use_memo(move || {
        let filter = active_filter();
        if filter == "All" {
            PRODUCTS.to_vec()
        } else {
            PRODUCTS
                .iter()
                .filter(|p| p.category == filter)
                .cloned()
                .collect()
        }
    });

    rsx! {
        div { class: "bg-white dark:bg-[#0A0A0A]",
            div { class: "mx-auto max-w-7xl px-6 py-12",
                // Header
                div { class: "flex items-baseline justify-between",
                    h1 { class: "text-2xl font-semibold tracking-tight text-[#1D1D1F] dark:text-[#F5F5F7]",
                        "Shop"
                    }
                    span { class: "text-sm text-[#1D1D1F]/50 dark:text-[#F5F5F7]/50",
                        "{filtered.len()} products"
                    }
                }

                // Filters
                div { class: "mt-6 flex flex-wrap gap-2",
                    for cat in categories.read().iter() {
                        FilterPill {
                            label: cat.clone(),
                            active: active_filter() == *cat,
                            on_click: {
                                let cat = cat.clone();
                                move |_| active_filter.set(cat.clone())
                            },
                        }
                    }
                }

                // Grid
                div { class: "mt-8 grid grid-cols-2 gap-4 sm:gap-5 md:grid-cols-3 lg:grid-cols-4",
                    for product in filtered.read().iter() {
                        ProductCard {
                            key: "{product.id}",
                            id: product.id.to_string(),
                            name: product.name.to_string(),
                            price_ngn: product.price_ngn,
                            image_url: product.image_url.to_string(),
                            in_stock: product.in_stock,
                        }
                    }
                }

                // Empty state
                if filtered.read().is_empty() {
                    div { class: "flex flex-col items-center justify-center py-20",
                        p { class: "text-lg font-medium text-[#1D1D1F]/40 dark:text-[#F5F5F7]/40",
                            "No products in this category"
                        }
                        button {
                            class: "mt-4 text-sm font-medium text-[#0071E3] dark:text-[#0A84FF] hover:underline",
                            onclick: move |_| active_filter.set("All".to_string()),
                            "View all products"
                        }
                    }
                }
            }
        }
    }
}
