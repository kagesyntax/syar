mod item_card;

use crate::models::WishlistItem;
use crate::router::Route;
use dioxus::prelude::*;
use dioxus_free_icons::icons::hi_outline_icons::{HiHeart, HiTrash};
use dioxus_free_icons::Icon;
use item_card::WishlistItemCard;

#[component]
pub fn Wishlist() -> Element {
    let mut wishlist = use_context::<Signal<Vec<WishlistItem>>>();
    let items = wishlist.read().clone();
    let item_count = items.len();
    let count_label = if item_count == 1 {
        format!("{item_count} item saved")
    } else {
        format!("{item_count} items saved")
    };

    rsx! {
        div { class: "bg-white dark:bg-[#0A0A0A] min-h-screen",

            // ── Page Header ────────────────────────────────
            div { class: "mx-auto max-w-7xl px-6 pt-8 pb-4",
                div { class: "flex items-center gap-3",
                    Icon {
                        width: 28, height: 28, fill: "none", icon: HiHeart,
                        class: "text-[#1D1D1F] dark:text-[#F5F5F7]"
                    }
                    h1 {
                        class: "text-2xl font-semibold tracking-tight text-[#1D1D1F] dark:text-[#F5F5F7]",
                        "Wishlist"
                    }
                }
                if item_count > 0 {
                    p {
                        class: "mt-1 text-sm text-[#1D1D1F]/50 dark:text-[#F5F5F7]/50",
                        "{count_label}"
                    }
                }
            }

            // ── Divider ─────────────────────────────────────
            div { class: "h-px bg-[#E5E5E7] dark:bg-[#2C2C2E]" }

            // ── Content ─────────────────────────────────────
            div { class: "mx-auto max-w-7xl px-6 py-8",

                if items.is_empty() {
                    // ── Empty State ──────────────────────────
                    div { class: "flex flex-col items-center justify-center py-24",
                        div {
                            class: "flex h-16 w-16 items-center justify-center bg-[#F5F5F7] dark:bg-[#1C1C1E]",
                            Icon {
                                width: 32, height: 32, fill: "none", icon: HiHeart,
                                class: "text-[#1D1D1F]/20 dark:text-[#F5F5F7]/20"
                            }
                        }
                        h2 {
                            class: "mt-6 text-lg font-semibold text-[#1D1D1F] dark:text-[#F5F5F7]",
                            "Your wishlist is empty"
                        }
                        p {
                            class: "mt-2 text-sm text-[#1D1D1F]/50 dark:text-[#F5F5F7]/50 max-w-sm text-center",
                            "Save items you love by tapping the heart icon on any product."
                        }
                        Link {
                            to: Route::Shop {},
                            class: "mt-8 inline-flex h-12 items-center justify-center bg-[#0071E3] dark:bg-[#0A84FF] px-8 text-sm font-medium text-white transition-all duration-200 hover:-translate-y-1 hover:shadow-xl active:scale-[0.98]",
                            "Browse products"
                        }
                    }
                } else {
                    // ── Wishlist Grid ────────────────────────
                    div { class: "grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4",
                        for item in items.iter() {
                            WishlistItemCard {
                                key: "{item.id}",
                                item: item.clone(),
                                on_remove: move |id| {
                                    wishlist.write().retain(|w| w.id != id);
                                }
                            }
                        }
                    }

                    // ── Clear All ─────────────────────────────
                    div { class: "mt-8 flex justify-center",
                        button {
                            class: "inline-flex items-center gap-2 text-sm text-[#1D1D1F]/40 dark:text-[#F5F5F7]/40 hover:text-red-500 dark:hover:text-red-400 transition-colors duration-200",
                            onclick: move |_| { wishlist.write().clear(); },
                            Icon { width: 14, height: 14, fill: "none", icon: HiTrash }
                            "Clear all"
                        }
                    }
                }
            }
        }
    }
}
