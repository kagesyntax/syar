mod desktop;
mod mobile;

use desktop::DesktopNavbar;
use dioxus::document::eval;
use dioxus::prelude::*;
use mobile::{MobileBottomNav, MobileSearch};

#[derive(Props, Clone, PartialEq)]
pub struct NavbarProps {
    pub wishlist_count: usize,
}

#[component]
pub fn Navbar(props: NavbarProps) -> Element {
    let dark = use_context::<Signal<bool>>();
    let search_q = use_context::<Signal<String>>();
    let search_open = use_context::<Signal<bool>>();

    // Toggle dark mode and persist to localStorage
    let toggle_dark = move |_| {
        let next = !dark();
        let mut dark = dark;
        dark.set(next);
        spawn(async move {
            let _ = eval(if next {
                "localStorage.setItem('theme','dark')"
            } else {
                "localStorage.setItem('theme','light')"
            })
            .await;
        });
    };

    rsx! {
        DesktopNavbar {
            wishlist_count: props.wishlist_count,
            dark,
            toggle_dark,
            search_q,
            search_open,
        }
        MobileSearch { search_q, search_open }
        MobileBottomNav {
            wishlist_count: props.wishlist_count,
            dark,
            toggle_dark,
        }
    }
}
