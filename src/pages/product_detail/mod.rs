mod components;

use crate::models::{format_ngn, Product, WishlistItem, PRODUCTS};
use components::{AccordionItem, SpecsGrid};
use dioxus::prelude::*;
use dioxus_free_icons::icons::hi_outline_icons::{HiArrowLeft, HiChatAlt, HiHeart};
use dioxus_free_icons::icons::hi_solid_icons::HiHeart as HiHeartSolid;
use dioxus_free_icons::Icon;

#[component]
pub fn ProductDetail(id: String) -> Element {
    let mut wishlist = use_context::<Signal<Vec<WishlistItem>>>();
    let nav = use_navigator();

    let product = PRODUCTS
        .iter()
        .find(|p| p.id == id)
        .cloned()
        .unwrap_or_else(|| PRODUCTS[0].clone());

    let is_wished = wishlist.read().iter().any(|w| w.id == product.id);
    let price_formatted = format_ngn(product.price_ngn);
    let product_id_for_wish = product.id.to_string();
    let product_name_for_link = product.name.to_string();

    // Related products: same category, different id
    let related: Vec<Product> = PRODUCTS
        .iter()
        .filter(|p| p.category == product.category && p.id != product.id)
        .cloned()
        .collect();

    rsx! {
        div { class: "bg-white dark:bg-[#0A0A0A] min-h-screen pb-16 lg:pb-0",

            // ── Breadcrumb / Back ──────────────────────────
            div { class: "mx-auto max-w-7xl px-6 pt-6 pb-2",
                button {
                    class: "inline-flex items-center gap-2 text-sm text-[#1D1D1F]/50 dark:text-[#F5F5F7]/50 hover:text-[#1D1D1F] dark:hover:text-[#F5F5F7] transition-colors duration-200",
                    onclick: move |_| { nav.go_back(); },
                    Icon { width: 16, height: 16, fill: "none", icon: HiArrowLeft }
                    "Back"
                }
            }

            // ── Product Layout ────────────────────────────
            div { class: "mx-auto max-w-7xl px-6 pb-16",
                div { class: "grid grid-cols-1 lg:grid-cols-2 gap-8 lg:gap-12",

                    // ── Left: Image ─────────────────────────
                    div {
                        class: "relative aspect-square overflow-hidden bg-[#F5F5F7] dark:bg-[#0A0A0A] border border-[#E5E5E7] dark:border-[#2C2C2E]",
                        img {
                            src: "{product.image_url}",
                            alt: "{product.name}",
                            class: "w-full h-full object-cover",
                        }
                        // SOLD badge overlay
                        if !product.in_stock {
                            div {
                                class: "absolute top-6 left-6 px-4 py-2 bg-[#1D1D1F] dark:bg-[#F5F5F7] text-white dark:text-[#0A0A0A] text-sm font-semibold tracking-wide",
                                "SOLD OUT"
                            }
                        }
                    }

                    // ── Right: Info ──────────────────────────
                    div { class: "flex flex-col lg:py-4",

                        // Category label
                        span {
                            class: "text-xs font-medium uppercase tracking-[0.15em] text-[#0071E3] dark:text-[#0A84FF]",
                            "{product.category}"
                        }

                        // Product name
                        h1 {
                            class: "mt-3 text-3xl lg:text-4xl font-semibold tracking-tight text-[#1D1D1F] dark:text-[#F5F5F7] leading-[1.1]",
                            "{product.name}"
                        }

                        // Price + Stock row
                        div { class: "mt-6 flex items-center gap-4",
                            span {
                                class: "text-2xl lg:text-3xl font-semibold text-[#1D1D1F] dark:text-white tracking-tight",
                                "{price_formatted}"
                            }
                            div { class: "flex items-center gap-2",
                                if product.in_stock {
                                    div { class: "w-2.5 h-2.5 bg-[#25D366] dark:bg-[#30D158]" }
                                    span { class: "text-sm font-medium text-[#25D366] dark:text-[#30D158]", "In Stock" }
                                } else {
                                    div { class: "w-2.5 h-2.5 bg-red-500" }
                                    span { class: "text-sm font-medium text-red-500", "Out of Stock" }
                                }
                            }
                        }

                        // Divider
                        div { class: "mt-6 h-px bg-[#E5E5E7] dark:bg-[#2C2C2E]" }

                        // Description
                        p {
                            class: "mt-6 text-base leading-relaxed text-[#1D1D1F]/60 dark:text-[#F5F5F7]/60",
                            "{product.blurb}"
                        }

                        // ── Action Buttons ────────────────────
                        div { class: "mt-8 flex flex-col gap-3 sm:flex-row sm:items-center",
                            // WhatsApp CTA
                            a {
                                href: "https://wa.me/2348000000000?text=Hi%2C%20I'm%20interested%20in%20the%20{product_name_for_link}",
                                target: "_blank",
                                rel: "noopener noreferrer",
                                class: "inline-flex items-center justify-center gap-2 bg-[#25D366] dark:bg-[#30D158] text-white font-medium text-sm h-12 px-8 transition-all duration-200 hover:-translate-y-1 hover:shadow-xl active:scale-[0.98] w-full sm:w-auto",
                                Icon { width: 20, height: 20, fill: "none", icon: HiChatAlt }
                                "Order on WhatsApp"
                            }

                            // Wishlist toggle
                            button {
                                class: "inline-flex items-center justify-center gap-2 border border-[#E5E5E7] dark:border-[#2C2C2E] text-[#1D1D1F] dark:text-[#F5F5F7] font-medium text-sm h-12 px-8 transition-all duration-200 hover:-translate-y-1 hover:shadow-xl active:scale-[0.98] w-full sm:w-auto",
                                onclick: move |_| {
                                    let mut w = wishlist.write();
                                    if let Some(pos) = w.iter().position(|item| item.id == product_id_for_wish) {
                                        w.remove(pos);
                                    } else {
                                        w.push(WishlistItem {
                                            id: product_id_for_wish.clone(),
                                            name: product.name.to_string(),
                                            price_ngn: product.price_ngn,
                                            image_url: product.image_url.to_string(),
                                            in_stock: product.in_stock,
                                        });
                                    }
                                },
                                if is_wished {
                                    Icon { width: 20, height: 20, fill: "currentColor", icon: HiHeartSolid,
                                        class: "text-red-500"
                                    }
                                } else {
                                    Icon { width: 20, height: 20, fill: "none", icon: HiHeart }
                                }
                                if is_wished { "Saved" } else { "Save to Wishlist" }
                            }
                        }

                        SpecsGrid {}

                        // ── Details Accordion ───────────────────
                        div { class: "mt-8 divide-y divide-[#E5E5E7] dark:divide-[#2C2C2E]",
                            AccordionItem { title: "Product Details",
                                "{product.blurb}"
                            }
                            AccordionItem { title: "Shipping & Returns",
                                "Free delivery on orders above ₦50,000. Standard delivery takes 2–5 business days across all 36 states. Express delivery available in Lagos (24 hours). Returns accepted within 7 days of delivery in original packaging."
                            }
                            AccordionItem { title: "Contact Us",
                                "Have questions? Reach us on WhatsApp for instant support. We respond within minutes during business hours (Mon–Sat, 9am–6pm WAT)."
                            }
                        }
                    }
                }
            }

            // ── Related Products ───────────────────────────
            if !related.is_empty() {
                section { class: "border-t border-[#E5E5E7] dark:border-[#2C2C2E]",
                    div { class: "mx-auto max-w-7xl px-6 py-16",
                        h2 { class: "text-xl font-semibold tracking-tight text-[#1D1D1F] dark:text-[#F5F5F7]",
                            "You might also like"
                        }
                        div { class: "mt-8 grid grid-cols-2 gap-4 sm:gap-5 md:grid-cols-3 lg:grid-cols-4",
                            for p in related.iter() {
                                crate::components::ProductCard {
                                    key: "{p.id}",
                                    id: p.id.to_string(),
                                    name: p.name.to_string(),
                                    price_ngn: p.price_ngn,
                                    image_url: p.image_url.to_string(),
                                    in_stock: p.in_stock,
                                }
                            }
                        }
                    }
                }
            }
        }

        // ── Mobile Sticky CTA ─────────────────────────
        if product.in_stock {
            div { class: "lg:hidden fixed inset-x-0 bottom-0 z-40 border-t border-[#E5E5E7] dark:border-[#2C2C2E] bg-white/95 dark:bg-[#0A0A0A]/95 backdrop-blur-xl p-4",
                a {
                    href: "https://wa.me/2348000000000?text=Hi%2C%20I'm%20interested%20in%20the%20{product_name_for_link}",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    class: "flex h-12 w-full items-center justify-center gap-2 bg-[#25D366] dark:bg-[#30D158] text-white font-medium text-sm transition-all duration-200 active:scale-[0.98]",
                    Icon { width: 20, height: 20, fill: "none", icon: HiChatAlt }
                    "Order on WhatsApp — {price_formatted}"
                }
            }
        }
    }
}
