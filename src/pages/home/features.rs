use dioxus::prelude::*;
use dioxus_free_icons::icons::hi_outline_icons::{HiChatAlt, HiRefresh, HiShieldCheck, HiTruck};
use dioxus_free_icons::Icon;

#[component]
pub fn Features() -> Element {
    rsx! {
        section { class: "border-t border-[#E5E5E7] dark:border-[#2C2C2E] bg-white dark:bg-[#0A0A0A]",
            div { class: "mx-auto grid max-w-7xl grid-cols-2 gap-px px-6 py-16 lg:grid-cols-4",

                // Nationwide Delivery
                div { class: "flex flex-col items-center text-center px-4 py-6",
                    div { class: "flex h-12 w-12 items-center justify-center bg-[#0071E3]/10 dark:bg-[#0A84FF]/10",
                        Icon { width: 24, height: 24, fill: "none", icon: HiTruck,
                            class: "text-[#0071E3] dark:text-[#0A84FF]"
                        }
                    }
                    h3 { class: "mt-4 text-sm font-semibold text-[#1D1D1F] dark:text-[#F5F5F7]", "Nationwide Delivery" }
                    p { class: "mt-1 text-xs text-[#1D1D1F]/50 dark:text-[#F5F5F7]/50", "All 36 states covered" }
                }

                // Authentic Guarantee
                div { class: "flex flex-col items-center text-center px-4 py-6",
                    div { class: "flex h-12 w-12 items-center justify-center bg-[#0071E3]/10 dark:bg-[#0A84FF]/10",
                        Icon { width: 24, height: 24, fill: "none", icon: HiShieldCheck,
                            class: "text-[#0071E3] dark:text-[#0A84FF]"
                        }
                    }
                    h3 { class: "mt-4 text-sm font-semibold text-[#1D1D1F] dark:text-[#F5F5F7]", "Authentic Guarantee" }
                    p { class: "mt-1 text-xs text-[#1D1D1F]/50 dark:text-[#F5F5F7]/50", "100% genuine products only" }
                }

                // Easy Returns
                div { class: "flex flex-col items-center text-center px-4 py-6",
                    div { class: "flex h-12 w-12 items-center justify-center bg-[#0071E3]/10 dark:bg-[#0A84FF]/10",
                        Icon { width: 24, height: 24, fill: "none", icon: HiRefresh,
                            class: "text-[#0071E3] dark:text-[#0A84FF]"
                        }
                    }
                    h3 { class: "mt-4 text-sm font-semibold text-[#1D1D1F] dark:text-[#F5F5F7]", "Easy Returns" }
                    p { class: "mt-1 text-xs text-[#1D1D1F]/50 dark:text-[#F5F5F7]/50", "7-day hassle-free returns" }
                }

                // WhatsApp Support
                div { class: "flex flex-col items-center text-center px-4 py-6",
                    div { class: "flex h-12 w-12 items-center justify-center bg-[#25D366]/10 dark:bg-[#30D158]/10",
                        Icon { width: 24, height: 24, fill: "none", icon: HiChatAlt,
                            class: "text-[#25D366] dark:text-[#30D158]"
                        }
                    }
                    h3 { class: "mt-4 text-sm font-semibold text-[#1D1D1F] dark:text-[#F5F5F7]", "WhatsApp Support" }
                    p { class: "mt-1 text-xs text-[#1D1D1F]/50 dark:text-[#F5F5F7]/50", "Chat with us instantly" }
                }
            }
        }
    }
}
