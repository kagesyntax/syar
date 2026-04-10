use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
#[allow(dead_code)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Outline,
    Ghost,
    Whatsapp,
}

impl Default for ButtonVariant {
    fn default() -> Self {
        Self::Primary
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct ButtonProps {
    pub children: Element,
    #[props(default)]
    pub variant: ButtonVariant,
    #[props(default)]
    pub class: String,
    #[props(default)]
    pub disabled: bool,
    #[props(default)]
    pub full_width: bool,
    pub onclick: EventHandler<MouseEvent>,
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
    let base = "inline-flex items-center justify-center font-medium text-sm transition-all duration-200 hover:-translate-y-1 hover:shadow-xl active:scale-[0.98] disabled:opacity-50 disabled:pointer-events-none disabled:transform-none disabled:shadow-none";

    let variant_classes = match props.variant {
        ButtonVariant::Primary => {
            "bg-[#0071E3] dark:bg-[#0A84FF] text-white px-6 h-12"
        }
        ButtonVariant::Secondary => {
            "bg-[#1D1D1F] dark:bg-[#F5F5F7] text-white dark:text-[#0A0A0A] px-6 h-12"
        }
        ButtonVariant::Outline => {
            "border border-[#E5E5E7] dark:border-[#2C2C2E] text-[#1D1D1F] dark:text-[#F5F5F7] px-6 h-12 hover:border-[#1D1D1F] dark:hover:border-[#F5F5F7]"
        }
        ButtonVariant::Ghost => {
            "text-[#1D1D1F] dark:text-[#F5F5F7] px-4 h-10 hover:bg-[#F5F5F7] dark:hover:bg-[#1C1C1E]"
        }
        ButtonVariant::Whatsapp => {
            "bg-[#25D366] dark:bg-[#30D158] text-white px-6 h-12"
        }
    };

    let width = if props.full_width { "w-full" } else { "" };

    rsx! {
        button {
            class: "{base} {variant_classes} {width} {props.class}",
            disabled: props.disabled,
            onclick: move |e| props.onclick.call(e),
            {props.children}
        }
    }
}
