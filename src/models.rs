/// Theme context wrapper to avoid Signal<bool> type collision
#[derive(Clone, Copy, PartialEq)]
pub struct Theme(pub bool);

/// Search visibility context wrapper to avoid Signal<bool> type collision
#[derive(Clone, Copy, PartialEq)]
pub struct SearchOpen(pub bool);

#[derive(Clone, PartialEq, Debug)]
pub struct Product {
    pub id: &'static str,
    pub name: &'static str,
    pub price_ngn: f64,
    pub image_url: &'static str,
    pub blurb: &'static str,
    pub category: &'static str,
    pub in_stock: bool,
}

#[derive(Clone, PartialEq, Debug)]
pub struct WishlistItem {
    pub id: String,
    pub name: String,
    pub price_ngn: f64,
    pub image_url: String,
    pub in_stock: bool,
}

pub const PRODUCTS: &[Product] = &[
    Product {
        id: "aurora-headphones",
        name: "Aurora Headphones",
        price_ngn: 285_000.0,
        image_url: "https://images.unsplash.com/photo-1505740420928-5e560c06d30e?q=80&w=1200",
        blurb: "Titanium drivers, memory-foam cushions, 40-hour battery with fast charge. Studio-grade ANC with transparency mode.",
        category: "Audio",
        in_stock: true,
    },
    Product {
        id: "arduino-uno",
        name: "Arduino Uno",
        price_ngn: 40_000.0,
        image_url: "https://res.cloudinary.com/dqga5yu1w/image/upload/v1771604720/Arduino_Uno_cmk7jg.jpg",
        blurb: "Industry-standard ATmega328P microcontroller board. 14 digital I/O pins, 6 analog inputs, USB-B interface. Perfect for robotics, IoT, and electronics prototyping.",
        category: "Microcontroller",
        in_stock: true,
    },
    Product {
        id: "noir-chronograph",
        name: "Noir Chronograph",
        price_ngn: 475_000.0,
        image_url: "https://images.unsplash.com/photo-1523170335258-f5ed11844a49?q=80&w=1200",
        blurb: "Sapphire crystal, Swiss automatic movement, 100m water resistance. Exhibition caseback.",
        category: "Watches",
        in_stock: true,
    },
    Product {
        id: "terra-loafer",
        name: "Terra Loafer",
        price_ngn: 185_000.0,
        image_url: "https://images.unsplash.com/photo-1542291026-7eec264c27ff?q=80&w=1200",
        blurb: "Vegetable-tanned Italian leather, Blake stitched, fully resoleable. Breaks in beautifully.",
        category: "Footwear",
        in_stock: true,
    },
    Product {
        id: "lumen-lamp",
        name: "Lumen Lamp",
        price_ngn: 155_000.0,
        image_url: "https://images.unsplash.com/photo-1507473885765-e6ed057f782c?q=80&w=1200",
        blurb: "Hand-blown opal glass, solid brass fittings, warm dim-to-glow LED. Touch-sensitive base.",
        category: "Home",
        in_stock: true,
    },
    Product {
        id: "onyx-speaker",
        name: "Onyx Speaker",
        price_ngn: 210_000.0,
        image_url: "https://images.unsplash.com/photo-1608043152269-423dbba4e7e1?q=80&w=1200",
        blurb: "360-degree spatial audio, machined aluminium body, 18-hour battery. Pairs in seconds.",
        category: "Audio",
        in_stock: false,
    },
    Product {
        id: "slim-cardholder",
        name: "Slim Cardholder",
        price_ngn: 45_000.0,
        image_url: "https://images.unsplash.com/photo-1627123424574-724758594e93?q=80&w=1200",
        blurb: "Full-grain calfskin, RFID-blocking, 6-card capacity with central cash slot. Slimmer than your phone.",
        category: "Accessories",
        in_stock: true,
    },
    Product {
        id: "ceramic-diffuser",
        name: "Ceramic Diffuser",
        price_ngn: 68_000.0,
        image_url: "https://images.unsplash.com/photo-1602928321679-560bb453f190?q=80&w=1200",
        blurb: "Hand-thrown stoneware, ultrasonic mist, ambient warm glow. Turns any room into a sanctuary.",
        category: "Home",
        in_stock: true,
    },
    Product {
        id: "field-jacket",
        name: "Field Jacket",
        price_ngn: 320_000.0,
        image_url: "https://images.unsplash.com/photo-1591047139829-d91aecb6caea?q=80&w=1200",
        blurb: "Waxed cotton shell, brass hardware, storm flap. Built for Lagos rain and harmattan dust alike.",
        category: "Apparel",
        in_stock: false,
    },
];

/// Format price in NGN with comma separators: 285000.0 → "₦285,000"
pub fn format_ngn(price: f64) -> String {
    let whole = price.round() as u64;
    let s = whole.to_string();
    let mut result = String::new();
    let digits = s.chars().count();
    for (i, c) in s.chars().enumerate() {
        if i > 0 && (digits - i) % 3 == 0 {
            result.push(',');
        }
        result.push(c);
    }
    format!("₦{result}")
}
