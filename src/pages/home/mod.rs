mod features;
mod hero;

use crate::components::{FilterPill, ProductCard};
use crate::models::{Product, PRODUCTS};
use dioxus::prelude::*;
use features::Features;
use hero::Hero;

#[component]
pub fn Home() -> Element {
    let mut active_filter = use_signal(|| "All".to_string());
    let search_q = use_context::<Signal<String>>();

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

    let filtered: Memo<Vec<Product>> = use_memo(move || {
        let filter = active_filter();
        let query = search_q().to_lowercase();

        PRODUCTS
            .iter()
            .filter(|p| {
                // Category filter
                let category_match = filter == "All" || p.category == filter;
                // Search filter
                let search_match = query.is_empty()
                    || p.name.to_lowercase().contains(&query)
                    || p.category.to_lowercase().contains(&query)
                    || p.blurb.to_lowercase().contains(&query);
                category_match && search_match
            })
            .cloned()
            .collect()
    });

    rsx! {
        Hero {}

        // ── Products Section ───────────────────────────────
        section { id: "products", class: "bg-white dark:bg-[#0A0A0A]",
            div { class: "mx-auto max-w-7xl px-6 pb-24 pt-12",
                // Section header
                div { class: "mb-8 flex items-end justify-between",
                    h2 {
                        class: "text-2xl font-semibold tracking-tight text-[#1D1D1F] dark:text-[#F5F5F7]",
                        "Featured"
                    }
                    span {
                        class: "text-sm text-[#1D1D1F]/50 dark:text-[#F5F5F7]/50",
                        "{filtered.len()} products"
                    }
                }

                // Filter pills
                div { class: "mb-8 flex flex-wrap gap-2",
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

                // Product grid
                div { class: "grid grid-cols-2 gap-4 sm:gap-5 md:grid-cols-3 lg:grid-cols-4",
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
                        p {
                            class: "text-lg font-medium text-[#1D1D1F]/40 dark:text-[#F5F5F7]/40",
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

        Features {}
    }
}
