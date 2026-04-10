use dioxus::prelude::*;
use dioxus_free_icons::icons::hi_outline_icons::{HiChevronDown};
use dioxus_free_icons::Icon;

#[component]
pub fn SpecsGrid() -> Element {
    rsx! {
        div { class: "mt-10 grid grid-cols-3 gap-px border border-[#E5E5E7] dark:border-[#2C2C2E] bg-[#E5E5E7] dark:bg-[#2C2C2E]",
            div { class: "bg-white dark:bg-[#0A0A0A] p-4 text-center",
                div { class: "text-xs font-semibold uppercase tracking-wider text-[#1D1D1F] dark:text-[#F5F5F7]", "Delivery" }
                div { class: "mt-1 text-xs text-[#1D1D1F]/50 dark:text-[#F5F5F7]/50", "2–5 business days" }
            }
            div { class: "bg-white dark:bg-[#0A0A0A] p-4 text-center",
                div { class: "text-xs font-semibold uppercase tracking-wider text-[#1D1D1F] dark:text-[#F5F5F7]", "Returns" }
                div { class: "mt-1 text-xs text-[#1D1D1F]/50 dark:text-[#F5F5F7]/50", "7-day window" }
            }
            div { class: "bg-white dark:bg-[#0A0A0A] p-4 text-center",
                div { class: "text-xs font-semibold uppercase tracking-wider text-[#1D1D1F] dark:text-[#F5F5F7]", "Authentic" }
                div { class: "mt-1 text-xs text-[#1D1D1F]/50 dark:text-[#F5F5F7]/50", "100% guaranteed" }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct AccordionItemProps {
    pub title: String,
    pub children: Element,
}

#[component]
pub fn AccordionItem(props: AccordionItemProps) -> Element {
    rsx! {
        details { class: "group",
            summary { class: "flex items-center justify-between py-4 cursor-pointer text-sm font-medium text-[#1D1D1F] dark:text-[#F5F5F7] hover:text-[#0071E3] dark:hover:text-[#0A84FF] transition-colors duration-200",
                "{props.title}"
                Icon { width: 16, height: 16, fill: "none", icon: HiChevronDown,
                    class: "transition-transform duration-200 group-open:rotate-180 text-[#1D1D1F]/40 dark:text-[#F5F5F7]/40"
                }
            }
            div { class: "pb-4 text-sm leading-relaxed text-[#1D1D1F]/60 dark:text-[#F5F5F7]/60",
                {props.children}
            }
        }
    }
}
