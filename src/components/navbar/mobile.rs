use crate::models::{SearchOpen, Theme};
use crate::router::Route;
use dioxus::prelude::*;
use dioxus_free_icons::icons::hi_outline_icons::{HiHeart, HiHome, HiMoon, HiSearch, HiSun, HiX};
use dioxus_free_icons::Icon;

#[derive(Props, Clone, PartialEq)]
pub struct MobileSearchProps {
    pub search_q: Signal<String>,
    pub search_open: Signal<SearchOpen>,
}

#[component]
pub fn MobileSearch(props: MobileSearchProps) -> Element {
    let mut search_q = props.search_q;
    let mut search_open = props.search_open;

    if !search_open().0 {
        return rsx! {};
    }

    rsx! {
        div {
            class: "md:hidden border-b border-[#E5E5E7] dark:border-[#2C2C2E] bg-white dark:bg-[#0A0A0A] px-4 py-3",

            div { class: "relative",
                div {
                    class: "absolute left-3 top-1/2 -translate-y-1/2 pointer-events-none text-[#1D1D1F]/30 dark:text-[#F5F5F7]/30",
                    Icon { width: 16, height: 16, fill: "none", icon: HiSearch }
                }
                input {
                    class: "w-full h-10 pl-10 pr-10 text-sm bg-[#F5F5F7] dark:bg-[#1C1C1E] border border-[#E5E5E7] dark:border-[#2C2C2E] text-[#1D1D1F] dark:text-[#F5F5F7] placeholder:text-[#1D1D1F]/30 dark:placeholder:text-[#F5F5F7]/30 outline-none focus:border-[#0071E3] dark:focus:border-[#0A84FF] transition-colors duration-200",
                    placeholder: "Search products...",
                    value: "{search_q}",
                    autofocus: true,
                    oninput: move |e| search_q.set(e.value()),
                    onkeydown: move |e| {
                        if e.key() == Key::Escape {
                            search_q.set(String::new());
                            search_open.set(SearchOpen(false));
                        }
                    },
                }
                button {
                    class: "absolute right-3 top-1/2 -translate-y-1/2 text-[#1D1D1F]/40 dark:text-[#F5F5F7]/40 hover:text-[#1D1D1F] dark:hover:text-[#F5F5F7] transition-colors duration-200",
                    onclick: move |_| {
                        search_q.set(String::new());
                        search_open.set(SearchOpen(false));
                    },
                    Icon { width: 16, height: 16, fill: "none", icon: HiX }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct MobileBottomNavProps {
    pub wishlist_count: usize,
    pub theme: Signal<Theme>,
    pub toggle_dark: EventHandler<()>,
}

#[component]
pub fn MobileBottomNav(props: MobileBottomNavProps) -> Element {
    rsx! {
        nav {
            class: "md:hidden fixed bottom-0 inset-x-0 z-50 border-t border-[#E5E5E7] dark:border-[#2C2C2E] bg-white/95 dark:bg-[#0A0A0A]/95 backdrop-blur-xl",

            div { class: "grid grid-cols-4 h-16",

                // Home
                Link {
                    to: Route::Home {},
                    class: "flex flex-col items-center justify-center gap-1 text-[#1D1D1F]/40 dark:text-[#F5F5F7]/40 hover:text-[#1D1D1F] dark:hover:text-[#F5F5F7] transition-colors duration-200 active:scale-[0.95]",
                    Icon { width: 20, height: 20, fill: "none", icon: HiHome }
                    span { class: "text-[10px] font-medium", "Home" }
                }

                // Shop
                Link {
                    to: Route::Shop {},
                    class: "flex flex-col items-center justify-center gap-1 text-[#1D1D1F]/40 dark:text-[#F5F5F7]/40 hover:text-[#1D1D1F] dark:hover:text-[#F5F5F7] transition-colors duration-200 active:scale-[0.95]",
                    Icon { width: 20, height: 20, fill: "none", icon: HiSearch }
                    span { class: "text-[10px] font-medium", "Shop" }
                }

                // Wishlist
                Link {
                    to: Route::Wishlist {},
                    class: "flex flex-col items-center justify-center gap-1 text-[#1D1D1F]/40 dark:text-[#F5F5F7]/40 hover:text-[#1D1D1F] dark:hover:text-[#F5F5F7] transition-colors duration-200 active:scale-[0.95]",
                    div { class: "relative",
                        Icon { width: 20, height: 20, fill: "none", icon: HiHeart }
                        if props.wishlist_count > 0 {
                            span {
                                class: "absolute -top-1.5 -right-2 flex items-center justify-center min-w-[14px] h-[14px] px-0.5 bg-[#0071E3] dark:bg-[#0A84FF] text-white text-[8px] font-bold leading-none",
                                "{props.wishlist_count}",
                            }
                        }
                    }
                    span { class: "text-[10px] font-medium", "Wishlist" }
                }

                // Dark mode toggle (mobile)
                button {
                    class: "flex flex-col items-center justify-center gap-1 text-[#1D1D1F]/40 dark:text-[#F5F5F7]/40 hover:text-[#1D1D1F] dark:hover:text-[#F5F5F7] transition-colors duration-200 active:scale-[0.95]",
                    onclick: move |_| props.toggle_dark.call(()),
                    if (*props.theme)().0 {
                        Icon { width: 20, height: 20, fill: "none", icon: HiSun }
                    } else {
                        Icon { width: 20, height: 20, fill: "none", icon: HiMoon }
                    }
                    if (*props.theme)().0 {
                        span { class: "text-[10px] font-medium", "Light" }
                    } else {
                        span { class: "text-[10px] font-medium", "Dark" }
                    }
                }
            }
        }
    }
}
