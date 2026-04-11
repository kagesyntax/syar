mod desktop;
mod mobile;

use crate::models::SearchOpen;
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
    let mut theme = use_context::<Signal<crate::models::Theme>>();
    let search_q = use_context::<Signal<String>>();
    let search_open = use_context::<Signal<SearchOpen>>();

    // Toggle dark mode and persist to localStorage
    let toggle_dark = move |_| {
        let next = !theme().0;
        theme.set(crate::models::Theme(next));
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
            theme,
            toggle_dark,
            search_q,
            search_open,
        }
        MobileSearch { search_q, search_open }
        MobileBottomNav {
            wishlist_count: props.wishlist_count,
            theme,
            toggle_dark,
        }
    }
}
