use crate::models::{format_ngn, WishlistItem};
use crate::router::Route;
use dioxus::prelude::*;
use dioxus_free_icons::icons::hi_outline_icons::{HiShoppingBag, HiTrash};
use dioxus_free_icons::Icon;

#[derive(Props, Clone, PartialEq)]
pub struct WishlistItemCardProps {
    pub item: WishlistItem,
    pub on_remove: EventHandler<String>,
}

#[component]
pub fn WishlistItemCard(props: WishlistItemCardProps) -> Element {
    let item = props.item;
    let item_id = item.id.clone();
    let item_id_for_link = item.id.clone();
    let item_id_for_remove = item.id.clone();
    let item_name = item.name.clone();
    let item_image = item.image_url.clone();
    let item_price = item.price_ngn;
    let item_stock = item.in_stock;
    let price_formatted = format_ngn(item_price);

    rsx! {
        div {
            key: "{item_id}",
            class: "group bg-white dark:bg-[#1C1C1E] border border-[#E5E5E7] dark:border-[#2C2C2E] hover:shadow-2xl hover:-translate-y-1 transition-all duration-200 overflow-hidden",

            // ── Image ───────────────────────
            Link {
                to: Route::ProductDetail { id: item_id },
                class: "block relative aspect-square overflow-hidden bg-[#F5F5F7] dark:bg-[#0A0A0A]",
                img {
                    src: "{item_image}",
                    alt: "{item_name}",
                    class: "w-full h-full object-cover group-hover:scale-105 transition-transform duration-300",
                    loading: "lazy",
                }
                if !item_stock {
                    div {
                        class: "absolute top-3 left-3 px-2 py-1 bg-[#1D1D1F] dark:bg-[#F5F5F7] text-white dark:text-[#0A0A0A] text-xs font-medium tracking-wide z-10",
                        "SOLD"
                    }
                }
            }

            // ── Info ─────────────────────────
            div { class: "p-4",
                Link {
                    to: Route::ProductDetail { id: item_id_for_link },
                    class: "block text-sm text-[#1D1D1F] dark:text-[#F5F5F7] font-medium truncate hover:underline",
                    "{item_name}"
                }
                div { class: "flex items-center justify-between mt-3",
                    span {
                        class: "text-xl font-semibold text-[#1D1D1F] dark:text-white",
                        "{price_formatted}"
                    }
                    if item_stock {
                        div { class: "w-2 h-2 bg-[#25D366] dark:bg-[#30D158]" }
                    } else {
                        div { class: "w-2 h-2 bg-[#1D1D1F]/20 dark:bg-[#F5F5F7]/20" }
                    }
                }

                // ── Action Row ─────────────────
                div { class: "flex items-center gap-2 mt-4 pt-3 border-t border-[#E5E5E7] dark:border-[#2C2C2E]",
                    a {
                        href: "https://wa.me/2348000000000?text=Hi%2C%20I%27m%20interested%20in%20the%20{item_name}",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        class: "flex-1 inline-flex items-center justify-center gap-1.5 h-9 bg-[#25D366] dark:bg-[#30D158] text-white text-xs font-medium transition-all duration-200 hover:-translate-y-0.5 hover:shadow-lg active:scale-[0.98]",
                        Icon { width: 14, height: 14, fill: "none", icon: HiShoppingBag }
                        "Order"
                    }
                    button {
                        class: "inline-flex items-center justify-center w-9 h-9 border border-[#E5E5E7] dark:border-[#2C2C2E] text-[#1D1D1F]/40 dark:text-[#F5F5F7]/40 hover:text-red-500 hover:border-red-500 dark:hover:text-red-400 dark:hover:border-red-400 transition-all duration-200 active:scale-[0.98]",
                        onclick: move |_| {
                            props.on_remove.call(item_id_for_remove.clone());
                        },
                        Icon { width: 16, height: 16, fill: "none", icon: HiTrash }
                    }
                }
            }
        }
    }
}
