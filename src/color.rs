pub fn hsl_to_rgb(h: f64, s: f64, l: f64) -> (u8, u8, u8) {
    let (r, g, b) = if s == 0. {
        (l, l, l)
    } else {
        let q = if l < 0.5 { l * (1. + s) } else { l + s - l * s };
        let p = 2. * l - q;
        (
            hue_to_rgb(p, q, h + 1.0 / 3.0),
            hue_to_rgb(p, q, h),
            hue_to_rgb(p, q, h - 1.0 / 3.0),
        )
    };

    ((r * 255.) as u8, (g * 255.) as u8, (b * 255.) as u8)
}

fn hue_to_rgb(p: f64, q: f64, mut t: f64) -> f64 {
    if t < 0. {
        t += 1.0;
    }
    if t > 1.0 {
        t -= 1.0;
    }

    if t < (1.0 / 6.0) {
        p + (q - p) * 6.0 * t
    } else if t < (1.0 / 2.0) {
        q
    } else if t < (2.0 / 3.0) {
        p + (q - p) * (2.0 / 3.0 - t) * 6.0
    } else {
        p
    }
}
