use crate::models::{format_ngn, WishlistItem};
use crate::router::Route;
use dioxus::prelude::*;
use dioxus_free_icons::icons::hi_outline_icons::HiHeart;
use dioxus_free_icons::icons::hi_solid_icons::HiHeart as HiHeartSolid;
use dioxus_free_icons::Icon;

#[derive(Props, Clone, PartialEq)]
pub struct ProductCardProps {
    pub id: String,
    pub name: String,
    pub price_ngn: f64,
    pub image_url: String,
    #[props(default = true)]
    pub in_stock: bool,
}

#[component]
pub fn ProductCard(props: ProductCardProps) -> Element {
    let mut wishlist = use_context::<Signal<Vec<WishlistItem>>>();
    let nav = use_navigator();
    let is_wished = wishlist.read().iter().any(|w| w.id == props.id);
    let price_formatted = format_ngn(props.price_ngn);
    let product_id = props.id.clone();
    let product_id_for_wish = props.id.clone();

    rsx! {
        div {
            class: "group bg-white dark:bg-[#1C1C1E] border border-[#E5E5E7] dark:border-[#2C2C2E] hover:shadow-2xl hover:-translate-y-1 transition-all duration-200 cursor-pointer overflow-hidden",
            onclick: move |_| {
                nav.push(Route::ProductDetail { id: product_id.clone() });
            },

            // ── Image Area ───────────────────────────────
            div { class: "relative aspect-square overflow-hidden bg-[#F5F5F7] dark:bg-[#0A0A0A]",
                img {
                    src: "{props.image_url}",
                    alt: "{props.name}",
                    class: "w-full h-full object-cover group-hover:scale-105 transition-transform duration-300",
                    loading: "lazy",
                }

                // Wishlist heart button — sharp square
                button {
                    class: "absolute top-3 right-3 w-9 h-9 bg-white/90 dark:bg-black/70 backdrop-blur flex items-center justify-center hover:bg-white dark:hover:bg-black transition-colors duration-200 z-10",
                    onclick: move |e| {
                        e.stop_propagation();
                        let mut w = wishlist.write();
                        if let Some(pos) = w.iter().position(|item| item.id == product_id_for_wish) {
                            w.remove(pos);
                        } else {
                            w.push(WishlistItem {
                                id: product_id_for_wish.clone(),
                                name: props.name.clone(),
                                price_ngn: props.price_ngn,
                                image_url: props.image_url.clone(),
                                in_stock: props.in_stock,
                            });
                        }
                    },
                    if is_wished {
                        span { class: "text-red-500",
                            Icon { width: 20, height: 20, fill: "currentColor", icon: HiHeartSolid }
                        }
                    } else {
                        span { class: "text-[#1D1D1F]/40 dark:text-[#F5F5F7]/40",
                            Icon { width: 20, height: 20, fill: "none", icon: HiHeart }
                        }
                    }
                }

                // SOLD badge — sharp rectangle
                if !props.in_stock {
                    div {
                        class: "absolute top-3 left-3 px-2 py-1 bg-[#1D1D1F] dark:bg-[#F5F5F7] text-white dark:text-[#0A0A0A] text-xs font-medium tracking-wide z-10",
                        "SOLD"
                    }
                }

                // Subtle bottom gradient on hover
                div { class: "pointer-events-none absolute inset-x-0 bottom-0 h-20 bg-gradient-to-t from-black/10 to-transparent opacity-0 group-hover:opacity-100 transition-opacity duration-300" }
            }

            // ── Info Area ────────────────────────────────
            div { class: "p-4",
                h3 {
                    class: "text-sm text-[#1D1D1F] dark:text-[#F5F5F7] font-medium truncate",
                    "{props.name}"
                }
                div { class: "flex items-center justify-between mt-3",
                    p {
                        class: "text-xl font-semibold text-[#1D1D1F] dark:text-white",
                        "{price_formatted}"
                    }
                    if props.in_stock {
                        // Sharp green dot for in-stock
                        div { class: "w-2 h-2 bg-[#25D366] dark:bg-[#30D158]" }
                    } else {
                        // Muted dot for out-of-stock
                        div { class: "w-2 h-2 bg-[#1D1D1F]/20 dark:bg-[#F5F5F7]/20" }
                    }
                }
            }
        }
    }
}
