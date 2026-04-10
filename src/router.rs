use dioxus::prelude::*;
use crate::pages::{Home, ProductDetail, Wishlist, Shop};
use crate::components::layout::{Shell, NotFound};

#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[layout(Shell)]
    #[route("/")]
    Home {},
    #[route("/shop")]
    Shop {},
    #[route("/product/:id")]
    ProductDetail { id: String },
    #[route("/wishlist")]
    Wishlist {},

    #[route("/:..segments")]
    NotFound { segments: Vec<String> },
}
