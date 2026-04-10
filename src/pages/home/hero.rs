use dioxus::prelude::*;

#[component]
pub fn Hero() -> Element {
    rsx! {
        section { class: "relative overflow-hidden bg-white dark:bg-[#0A0A0A]",
            div { class: "mx-auto max-w-7xl px-6 py-20 lg:py-28",
                p {
                    class: "text-sm font-medium uppercase tracking-[0.2em] text-[#0071E3] dark:text-[#0A84FF]",
                    "New Season — 2025"
                }
                h1 {
                    class: "mt-4 max-w-3xl text-[clamp(40px,7vw,80px)] font-semibold leading-[0.9] tracking-tight text-[#1D1D1F] dark:text-[#F5F5F7]",
                    "Objects made"
                    br {}
                    "to be kept."
                }
                p {
                    class: "mt-6 max-w-lg text-lg leading-relaxed text-[#1D1D1F]/60 dark:text-[#F5F5F7]/60",
                    "A considered edit of premium essentials — sourced globally, delivered across Nigeria. Designed slowly, built to last."
                }
                div { class: "mt-10 flex flex-wrap items-center gap-3",
                    a {
                        href: "#products",
                        class: "inline-flex h-12 items-center justify-center bg-[#0071E3] dark:bg-[#0A84FF] px-8 text-sm font-medium text-white transition-all duration-200 hover:-translate-y-1 hover:shadow-xl active:scale-[0.98]",
                        "Shop the edit"
                    }
                    a {
                        href: "#products",
                        class: "inline-flex h-12 items-center justify-center border border-[#E5E5E7] dark:border-[#2C2C2E] px-8 text-sm font-medium text-[#1D1D1F] dark:text-[#F5F5F7] transition-all duration-200 hover:-translate-y-1 hover:shadow-xl active:scale-[0.98]",
                        "View catalogue"
                    }
                }
            }
            // Decorative line
            div { class: "absolute bottom-0 left-0 right-0 h-px bg-[#E5E5E7] dark:bg-[#2C2C2E]" }
        }
    }
}
