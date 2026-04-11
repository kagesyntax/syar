use crate::models::{SearchOpen, Theme};
use crate::router::Route;
use dioxus::prelude::*;
use dioxus_free_icons::icons::hi_outline_icons::{HiHeart, HiMoon, HiSearch, HiSun};
use dioxus_free_icons::Icon;

#[derive(Props, Clone, PartialEq)]
pub struct DesktopNavbarProps {
    pub wishlist_count: usize,
    pub theme: Signal<Theme>,
    pub toggle_dark: EventHandler<()>,
    pub search_q: Signal<String>,
    pub search_open: Signal<crate::models::SearchOpen>,
}

#[component]
pub fn DesktopNavbar(props: DesktopNavbarProps) -> Element {
    let mut search_q = props.search_q;
    let mut search_open = props.search_open;

    rsx! {
        header {
            class: "sticky top-0 z-50 w-full border-b border-[#E5E5E7] dark:border-[#2C2C2E] bg-white/80 dark:bg-[#0A0A0A]/80 backdrop-blur-xl",

            div { class: "mx-auto flex h-16 max-w-7xl items-center justify-between px-6",

                // ── Left: Logo ────────────────────────────
                Link {
                    to: Route::Home {},
                    class: "flex items-center gap-2 shrink-0",
                    span {
                        class: "text-base font-semibold tracking-wide text-[#1D1D1F] dark:text-[#F5F5F7] uppercase",
                        "SYAR",
                    }
                }

                // ── Center: Search ────────────────────────
                div { class: "hidden md:flex flex-1 max-w-md mx-8",
                    div { class: "relative w-full",
                        // Search icon
                        div {
                            class: "absolute left-3 top-1/2 -translate-y-1/2 pointer-events-none text-[#1D1D1F]/30 dark:text-[#F5F5F7]/30",
                            Icon { width: 16, height: 16, fill: "none", icon: HiSearch }
                        }
                        input {
                            class: "w-full h-10 pl-10 pr-4 text-sm bg-[#F5F5F7] dark:bg-[#1C1C1E] border border-[#E5E5E7] dark:border-[#2C2C2E] text-[#1D1D1F] dark:text-[#F5F5F7] placeholder:text-[#1D1D1F]/30 dark:placeholder:text-[#F5F5F7]/30 outline-none focus:border-[#0071E3] dark:focus:border-[#0A84FF] transition-colors duration-200",
                            placeholder: "Search products...",
                            value: "{search_q}",
                            oninput: move |e| search_q.set(e.value()),
                            onkeydown: move |e| {
                                if e.key() == Key::Escape {
                                    search_q.set(String::new());
                                    search_open.set(SearchOpen(false));
                                }
                            },
                        }
                    }
                }

                // ── Right: Actions ───────────────────────
                div { class: "flex items-center gap-2",

                    // Mobile search trigger
                    button {
                        class: "md:hidden flex items-center justify-center w-10 h-10 text-[#1D1D1F]/60 dark:text-[#F5F5F7]/60 transition-all duration-200 hover:-translate-y-0.5 hover:text-[#1D1D1F] dark:hover:text-[#F5F5F7] active:scale-[0.98]",
                        onclick: move |_| {
                            let current = search_open().0;
                            search_open.set(SearchOpen(!current));
                        },
                        Icon { width: 20, height: 20, fill: "none", icon: HiSearch }
                    }

                    // Dark mode toggle
                    button {
                        class: "flex items-center justify-center w-10 h-10 text-[#1D1D1F]/60 dark:text-[#F5F5F7]/60 transition-all duration-200 hover:-translate-y-0.5 hover:text-[#1D1D1F] dark:hover:text-[#F5F5F7] active:scale-[0.98]",
                        onclick: move |_| props.toggle_dark.call(()),
                        if (*props.theme)().0 {
                            Icon { width: 20, height: 20, fill: "none", icon: HiSun }
                        } else {
                            Icon { width: 20, height: 20, fill: "none", icon: HiMoon }
                        }
                    }

                    // Wishlist with sharp square count badge
                    Link {
                        to: Route::Wishlist {},
                        class: "relative flex items-center justify-center w-10 h-10 text-[#1D1D1F]/60 dark:text-[#F5F5F7]/60 transition-all duration-200 hover:-translate-y-0.5 hover:text-[#1D1D1F] dark:hover:text-[#F5F5F7] active:scale-[0.98]",
                        Icon { width: 20, height: 20, fill: "none", icon: HiHeart }
                        if props.wishlist_count > 0 {
                            span {
                                class: "absolute -top-1 -right-1 flex items-center justify-center min-w-[18px] h-[18px] px-1 bg-[#0071E3] dark:bg-[#0A84FF] text-white text-[10px] font-semibold leading-none",
                                "{props.wishlist_count}",
                            }
                        }
                    }
                }
            }
        }
    }
}
