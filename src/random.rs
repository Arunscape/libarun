pub fn flip_a_coin() -> u128 {
    let coin = {
        let mut rng = rand::rng();
        rng.random::<bool>()
    };

    if coin {
        "heads"
    } else {
        "tails"
    }
}

pub fn random_number() -> u128 {
    let mut rng = rand::rng();
    rng.random::<u128>().to_string()
}

pub fn random_colour() -> RgbA {
    let mut rng = rand::rng();

    let r: u8 = rng.random();
    let g: u8 = rng.random();
    let b: u8 = rng.random();
    let a: u8 = rng.random();
    let a_div = (a as f32 / 255.0 * 100.0).round() / 100.0;

    let rgba_str = format!("rgba({r}, {g}, {b}, {a_div})");
    let hex = format!("#{r:02x}{g:02x}{b:02x}{a:02x}");

    let res = json!({
        "rgba": {"r": r, "g": g, "b": b, "a": a_div.to_string()},
        "rgba_str": rgba_str,
        "hex": hex,
    });

    Json(res)
}
