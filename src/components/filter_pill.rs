use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct FilterPillProps {
    pub label: String,
    #[props(default)]
    pub active: bool,
    pub on_click: EventHandler<MouseEvent>,
}

#[component]
pub fn FilterPill(props: FilterPillProps) -> Element {
    let base = "inline-flex items-center justify-center px-4 py-2 text-sm font-medium transition-all duration-200 cursor-pointer select-none border";

    let state_classes = if props.active {
        // Active state — accent background
        "bg-[#0071E3] dark:bg-[#0A84FF] text-white border-[#0071E3] dark:border-[#0A84FF] hover:shadow-xl hover:-translate-y-0.5"
    } else {
        // Inactive state — surface with border
        "bg-white dark:bg-[#1C1C1E] text-[#1D1D1F] dark:text-[#F5F5F7] border-[#E5E5E7] dark:border-[#2C2C2E] hover:border-[#1D1D1F]/30 dark:hover:border-[#F5F5F7]/30 hover:-translate-y-0.5"
    };

    rsx! {
        button {
            class: "{base} {state_classes}",
            onclick: move |e| props.on_click.call(e),
            {props.label}
        }
    }
}
