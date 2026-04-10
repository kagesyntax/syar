use dioxus::prelude::*;
use dioxus_free_icons::icons::hi_outline_icons::HiHeart;
use dioxus_free_icons::icons::hi_solid_icons::HiHeart as HiHeartSolid;
use dioxus_free_icons::Icon;

#[derive(Props, Clone, PartialEq)]
pub struct WishlistButtonProps {
    #[props(default)]
    pub is_wished: bool,
    pub on_click: EventHandler<MouseEvent>,
    #[props(default)]
    pub count: i32,
    #[props(default = "w-9 h-9".to_string())]
    pub size: String,
}

#[component]
pub fn WishlistButton(props: WishlistButtonProps) -> Element {
    let icon_color = if props.is_wished {
        "text-red-500"
    } else {
        "text-[#1D1D1F]/40 dark:text-[#F5F5F7]/40"
    };

    rsx! {
        div { class: "relative inline-flex",
            button {
                class: "inline-flex items-center justify-center {props.size} bg-white/90 dark:bg-[#1C1C1E]/90 backdrop-blur border border-[#E5E5E7] dark:border-[#2C2C2E] transition-all duration-200 hover:-translate-y-0.5 hover:shadow-xl active:scale-[0.98] {icon_color}",
                onclick: move |e| props.on_click.call(e),
                if props.is_wished {
                    Icon { width: 20, height: 20, fill: "currentColor", icon: HiHeartSolid }
                } else {
                    Icon { width: 20, height: 20, fill: "none", icon: HiHeart }
                }
            }
            // Sharp square count badge
            if props.count > 0 {
                span {
                    class: "absolute -top-1.5 -right-1.5 flex items-center justify-center min-w-[18px] h-[18px] px-1 bg-[#0071E3] dark:bg-[#0A84FF] text-white text-[10px] font-semibold leading-none",
                    "{props.count}"
                }
            }
        }
    }
}
