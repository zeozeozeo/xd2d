use crate::Real;

#[inline]
fn hue_to_rgb(p: f32, q: f32, t: f32) -> f32 {
    let t = if t < 0.0 {
        t + 1.0
    } else if t > 1.0 {
        t - 1.0
    } else {
        t
    };

    if t < 1.0 / 6.0 {
        p + (q - p) * 6.0 * t
    } else if t < 1.0 / 2.0 {
        q
    } else if t < 2.0 / 3.0 {
        p + (q - p) * (2.0 / 3.0 - t) * 6.0
    } else {
        p
    }
}

/// Represents an 8-bit color.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct Color {
    /// Red component.
    pub r: u8,
    /// Green component.
    pub g: u8,
    /// Blue component.
    pub b: u8,
    /// Alpha component.
    pub a: u8,
}

impl Color {
    // CSS colors

    /// Invisible: 0, 0, 0, 0
    pub const INVISIBLE: Self = rgba(0, 0, 0, 0);
    /// Aliceblue: https://rgb.to/aliceblue
    pub const ALICE_BLUE: Self = rgba(240, 248, 255, 255);
    /// Antiquewhite: https://rgb.to/antiquewhite
    pub const ANTIQUE_WHITE: Self = rgba(250, 235, 215, 255);
    /// Aqua: https://rgb.to/aqua
    pub const AQUA: Self = rgba(0, 255, 255, 255);
    /// Aquamarine: https://rgb.to/aquamarine
    pub const AQUAMARINE: Self = rgba(127, 255, 212, 255);
    /// Azure: https://rgb.to/azure
    pub const AZURE: Self = rgba(240, 255, 255, 255);
    /// Beige: https://rgb.to/beige
    pub const BEIGE: Self = rgba(245, 245, 220, 255);
    /// Bisque: https://rgb.to/bisque
    pub const BISQUE: Self = rgba(255, 228, 196, 255);
    /// Black: https://rgb.to/black
    pub const BLACK: Self = rgba(0, 0, 0, 255);
    /// Blanchedalmond: https://rgb.to/blanchedalmond
    pub const BLANCHED_ALMOND: Self = rgba(255, 235, 205, 255);
    /// Blue: https://rgb.to/blue
    pub const BLUE: Self = rgba(0, 0, 255, 255);
    /// Blueviolet: https://rgb.to/blueviolet
    pub const BLUE_VIOLET: Self = rgba(138, 43, 226, 255);
    /// Brown: https://rgb.to/brown
    pub const BROWN: Self = rgba(165, 42, 42, 255);
    /// Burlywood: https://rgb.to/burlywood
    pub const BURLY_WOOD: Self = rgba(222, 184, 135, 255);
    /// Cadetblue: https://rgb.to/cadetblue
    pub const CADET_BLUE: Self = rgba(95, 158, 160, 255);
    /// Chartreuse: https://rgb.to/chartreuse
    pub const CHARTREUSE: Self = rgba(127, 255, 0, 255);
    /// Chocolate: https://rgb.to/chocolate
    pub const CHOCOLATE: Self = rgba(210, 105, 30, 255);
    /// Coral: https://rgb.to/coral
    pub const CORAL: Self = rgba(255, 127, 80, 255);
    /// Cornflowerblue: https://rgb.to/cornflowerblue
    pub const CORNFLOWER_BLUE: Self = rgba(100, 149, 237, 255);
    /// Cornsilk: https://rgb.to/cornsilk
    pub const CORNSILK: Self = rgba(255, 248, 220, 255);
    /// Crimson: https://rgb.to/crimson
    pub const CRIMSON: Self = rgba(220, 20, 60, 255);
    /// Cyan: https://rgb.to/cyan
    pub const CYAN: Self = rgba(0, 255, 255, 255);
    /// Darkblue: https://rgb.to/darkblue
    pub const DARK_BLUE: Self = rgba(0, 0, 139, 255);
    /// Darkcyan: https://rgb.to/darkcyan
    pub const DARK_CYAN: Self = rgba(0, 139, 139, 255);
    /// Darkgoldenrod: https://rgb.to/darkgoldenrod
    pub const DARK_GOLDEN_ROD: Self = rgba(184, 134, 11, 255);
    /// Darkgray: https://rgb.to/darkgray
    pub const DARK_GRAY: Self = rgba(169, 169, 169, 255);
    /// Darkgreen: https://rgb.to/darkgreen
    pub const DARK_GREEN: Self = rgba(0, 100, 0, 255);
    /// Darkgrey: https://rgb.to/darkgrey
    pub const DARK_GREY: Self = rgba(169, 169, 169, 255);
    /// Darkkhaki: https://rgb.to/darkkhaki
    pub const DARK_KHAKI: Self = rgba(189, 183, 107, 255);
    /// Darkmagenta: https://rgb.to/darkmagenta
    pub const DARK_MAGENTA: Self = rgba(139, 0, 139, 255);
    /// Darkolivegreen: https://rgb.to/darkolivegreen
    pub const DARK_OLIVE_GREEN: Self = rgba(85, 107, 47, 255);
    /// Darkorange: https://rgb.to/darkorange
    pub const DARKORANGE: Self = rgba(255, 140, 0, 255);
    /// Darkorchid: https://rgb.to/darkorchid
    pub const DARK_ORCHID: Self = rgba(153, 50, 204, 255);
    /// Darkred: https://rgb.to/darkred
    pub const DARK_RED: Self = rgba(139, 0, 0, 255);
    /// Darksalmon: https://rgb.to/darksalmon
    pub const DARK_SALMON: Self = rgba(233, 150, 122, 255);
    /// Darkseagreen: https://rgb.to/darkseagreen
    pub const DARK_SEA_GREEN: Self = rgba(143, 188, 143, 255);
    /// Darkslateblue: https://rgb.to/darkslateblue
    pub const DARK_SLATE_BLUE: Self = rgba(72, 61, 139, 255);
    /// Darkslategray: https://rgb.to/darkslategray
    pub const DARK_SLATE_GRAY: Self = rgba(47, 79, 79, 255);
    /// Darkslategrey: https://rgb.to/darkslategrey
    pub const DARK_SLATE_GREY: Self = rgba(47, 79, 79, 255);
    /// Darkturquoise: https://rgb.to/darkturquoise
    pub const DARK_TURQUOISE: Self = rgba(0, 206, 209, 255);
    /// Darkviolet: https://rgb.to/darkviolet
    pub const DARK_VIOLET: Self = rgba(148, 0, 211, 255);
    /// Deeppink: https://rgb.to/deeppink
    pub const DEEP_PINK: Self = rgba(255, 20, 147, 255);
    /// Deepskyblue: https://rgb.to/deepskyblue
    pub const DEEP_SKY_BLUE: Self = rgba(0, 191, 255, 255);
    /// Dimgray: https://rgb.to/dimgray
    pub const DIM_GRAY: Self = rgba(105, 105, 105, 255);
    /// Dimgrey: https://rgb.to/dimgrey
    pub const DIM_GREY: Self = rgba(105, 105, 105, 255);
    /// Dodgerblue: https://rgb.to/dodgerblue
    pub const DODGER_BLUE: Self = rgba(30, 144, 255, 255);
    /// Firebrick: https://rgb.to/firebrick
    pub const FIRE_BRICK: Self = rgba(178, 34, 34, 255);
    /// Floralwhite: https://rgb.to/floralwhite
    pub const FLORAL_WHITE: Self = rgba(255, 250, 240, 255);
    /// Forestgreen: https://rgb.to/forestgreen
    pub const FOREST_GREEN: Self = rgba(34, 139, 34, 255);
    /// Fuchsia: https://rgb.to/fuchsia
    pub const FUCHSIA: Self = rgba(255, 0, 255, 255);
    /// Gainsboro: https://rgb.to/gainsboro
    pub const GAINSBORO: Self = rgba(220, 220, 220, 255);
    /// Ghostwhite: https://rgb.to/ghostwhite
    pub const GHOST_WHITE: Self = rgba(248, 248, 255, 255);
    /// Gold: https://rgb.to/gold
    pub const GOLD: Self = rgba(255, 215, 0, 255);
    /// Goldenrod: https://rgb.to/goldenrod
    pub const GOLDEN_ROD: Self = rgba(218, 165, 32, 255);
    /// Gray: https://rgb.to/gray
    pub const GRAY: Self = rgba(128, 128, 128, 255);
    /// Green: https://rgb.to/green
    pub const GREEN: Self = rgba(0, 128, 0, 255);
    /// Greenyellow: https://rgb.to/greenyellow
    pub const GREEN_YELLOW: Self = rgba(173, 255, 47, 255);
    /// Grey: https://rgb.to/grey
    pub const GREY: Self = rgba(128, 128, 128, 255);
    /// Honeydew: https://rgb.to/honeydew
    pub const HONEY_DEW: Self = rgba(240, 255, 240, 255);
    /// Hotpink: https://rgb.to/hotpink
    pub const HOT_PINK: Self = rgba(255, 105, 180, 255);
    /// Indianred: https://rgb.to/indianred
    pub const INDIAN_RED: Self = rgba(205, 92, 92, 255);
    /// Indigo: https://rgb.to/indigo
    pub const INDIGO: Self = rgba(75, 0, 130, 255);
    /// Ivory: https://rgb.to/ivory
    pub const IVORY: Self = rgba(255, 255, 240, 255);
    /// Khaki: https://rgb.to/khaki
    pub const KHAKI: Self = rgba(240, 230, 140, 255);
    /// Lavender: https://rgb.to/lavender
    pub const LAVENDER: Self = rgba(230, 230, 250, 255);
    /// Lavenderblush: https://rgb.to/lavenderblush
    pub const LAVENDER_BLUSH: Self = rgba(255, 240, 245, 255);
    /// Lawngreen: https://rgb.to/lawngreen
    pub const LAWN_GREEN: Self = rgba(124, 252, 0, 255);
    /// Lemonchiffon: https://rgb.to/lemonchiffon
    pub const LEMON_CHIFFON: Self = rgba(255, 250, 205, 255);
    /// Lightblue: https://rgb.to/lightblue
    pub const LIGHT_BLUE: Self = rgba(173, 216, 230, 255);
    /// Lightcoral: https://rgb.to/lightcoral
    pub const LIGHT_CORAL: Self = rgba(240, 128, 128, 255);
    /// Lightcyan: https://rgb.to/lightcyan
    pub const LIGHT_CYAN: Self = rgba(224, 255, 255, 255);
    /// Lightgoldenrodyellow: https://rgb.to/lightgoldenrodyellow
    pub const LIGHT_GOLDEN_ROD_YELLOW: Self = rgba(250, 250, 210, 255);
    /// Lightgray: https://rgb.to/lightgray
    pub const LIGHT_GRAY: Self = rgba(211, 211, 211, 255);
    /// Lightgreen: https://rgb.to/lightgreen
    pub const LIGHT_GREEN: Self = rgba(144, 238, 144, 255);
    /// Lightgrey: https://rgb.to/lightgrey
    pub const LIGHT_GREY: Self = rgba(211, 211, 211, 255);
    /// Lightpink: https://rgb.to/lightpink
    pub const LIGHT_PINK: Self = rgba(255, 182, 193, 255);
    /// Lightsalmon: https://rgb.to/lightsalmon
    pub const LIGHT_SALMON: Self = rgba(255, 160, 122, 255);
    /// Lightseagreen: https://rgb.to/lightseagreen
    pub const LIGHT_SEA_GREEN: Self = rgba(32, 178, 170, 255);
    /// Lightskyblue: https://rgb.to/lightskyblue
    pub const LIGHT_SKY_BLUE: Self = rgba(135, 206, 250, 255);
    /// Lightslategray: https://rgb.to/lightslategray
    pub const LIGHT_SLATE_GRAY: Self = rgba(119, 136, 153, 255);
    /// Lightslategrey: https://rgb.to/lightslategrey
    pub const LIGHT_SLATE_GREY: Self = rgba(119, 136, 153, 255);
    /// Lightsteelblue: https://rgb.to/lightsteelblue
    pub const LIGHT_STEEL_BLUE: Self = rgba(176, 196, 222, 255);
    /// Lightyellow: https://rgb.to/lightyellow
    pub const LIGHT_YELLOW: Self = rgba(255, 255, 224, 255);
    /// Lime: https://rgb.to/lime
    pub const LIME: Self = rgba(0, 255, 0, 255);
    /// Limegreen: https://rgb.to/limegreen
    pub const LIME_GREEN: Self = rgba(50, 205, 50, 255);
    /// Linen: https://rgb.to/linen
    pub const LINEN: Self = rgba(250, 240, 230, 255);
    /// Magenta: https://rgb.to/magenta
    pub const MAGENTA: Self = rgba(255, 0, 255, 255);
    /// Maroon: https://rgb.to/maroon
    pub const MAROON: Self = rgba(128, 0, 0, 255);
    /// Mediumaquamarine: https://rgb.to/mediumaquamarine
    pub const MEDIUM_AQUA_MARINE: Self = rgba(102, 205, 170, 255);
    /// Mediumblue: https://rgb.to/mediumblue
    pub const MEDIUM_BLUE: Self = rgba(0, 0, 205, 255);
    /// Mediumorchid: https://rgb.to/mediumorchid
    pub const MEDIUM_ORCHID: Self = rgba(186, 85, 211, 255);
    /// Mediumpurple: https://rgb.to/mediumpurple
    pub const MEDIUM_PURPLE: Self = rgba(147, 112, 219, 255);
    /// Mediumseagreen: https://rgb.to/mediumseagreen
    pub const MEDIUM_SEA_GREEN: Self = rgba(60, 179, 113, 255);
    /// Mediumslateblue: https://rgb.to/mediumslateblue
    pub const MEDIUM_SLATE_BLUE: Self = rgba(123, 104, 238, 255);
    /// Mediumspringgreen: https://rgb.to/mediumspringgreen
    pub const MEDIUM_SPRING_GREEN: Self = rgba(0, 250, 154, 255);
    /// Mediumturquoise: https://rgb.to/mediumturquoise
    pub const MEDIUM_TURQUOISE: Self = rgba(72, 209, 204, 255);
    /// Mediumvioletred: https://rgb.to/mediumvioletred
    pub const MEDIUM_VIOLET_RED: Self = rgba(199, 21, 133, 255);
    /// Midnightblue: https://rgb.to/midnightblue
    pub const MIDNIGHT_BLUE: Self = rgba(25, 25, 112, 255);
    /// Mintcream: https://rgb.to/mintcream
    pub const MINT_CREAM: Self = rgba(245, 255, 250, 255);
    /// Mistyrose: https://rgb.to/mistyrose
    pub const MISTY_ROSE: Self = rgba(255, 228, 225, 255);
    /// Moccasin: https://rgb.to/moccasin
    pub const MOCCASIN: Self = rgba(255, 228, 181, 255);
    /// Navajowhite: https://rgb.to/navajowhite
    pub const NAVAJO_WHITE: Self = rgba(255, 222, 173, 255);
    /// Navy: https://rgb.to/navy
    pub const NAVY: Self = rgba(0, 0, 128, 255);
    /// Oldlace: https://rgb.to/oldlace
    pub const OLD_LACE: Self = rgba(253, 245, 230, 255);
    /// Olive: https://rgb.to/olive
    pub const OLIVE: Self = rgba(128, 128, 0, 255);
    /// Olivedrab: https://rgb.to/olivedrab
    pub const OLIVE_DRAB: Self = rgba(107, 142, 35, 255);
    /// Orange: https://rgb.to/orange
    pub const ORANGE: Self = rgba(255, 165, 0, 255);
    /// Orangered: https://rgb.to/orangered
    pub const ORANGE_RED: Self = rgba(255, 69, 0, 255);
    /// Orchid: https://rgb.to/orchid
    pub const ORCHID: Self = rgba(218, 112, 214, 255);
    /// Palegoldenrod: https://rgb.to/palegoldenrod
    pub const PALE_GOLDEN_ROD: Self = rgba(238, 232, 170, 255);
    /// Palegreen: https://rgb.to/palegreen
    pub const PALE_GREEN: Self = rgba(152, 251, 152, 255);
    /// Paleturquoise: https://rgb.to/paleturquoise
    pub const PALE_TURQUOISE: Self = rgba(175, 238, 238, 255);
    /// Palevioletred: https://rgb.to/palevioletred
    pub const PALE_VIOLET_RED: Self = rgba(219, 112, 147, 255);
    /// Papayawhip: https://rgb.to/papayawhip
    pub const PAPAYA_WHIP: Self = rgba(255, 239, 213, 255);
    /// Peachpuff: https://rgb.to/peachpuff
    pub const PEACH_PUFF: Self = rgba(255, 218, 185, 255);
    /// Peru: https://rgb.to/peru
    pub const PERU: Self = rgba(205, 133, 63, 255);
    /// Pink: https://rgb.to/pink
    pub const PINK: Self = rgba(255, 192, 203, 255);
    /// Plum: https://rgb.to/plum
    pub const PLUM: Self = rgba(221, 160, 221, 255);
    /// Powderblue: https://rgb.to/powderblue
    pub const POWDER_BLUE: Self = rgba(176, 224, 230, 255);
    /// Purple: https://rgb.to/purple
    pub const PURPLE: Self = rgba(128, 0, 128, 255);
    /// Rebeccapurple: https://rgb.to/rebeccapurple
    pub const REBECCA_PURPLE: Self = rgba(102, 51, 153, 255);
    /// Red: https://rgb.to/red
    pub const RED: Self = rgba(255, 0, 0, 255);
    /// Rosybrown: https://rgb.to/rosybrown
    pub const ROSY_BROWN: Self = rgba(188, 143, 143, 255);
    /// Royalblue: https://rgb.to/royalblue
    pub const ROYAL_BLUE: Self = rgba(65, 105, 225, 255);
    /// Saddlebrown: https://rgb.to/saddlebrown
    pub const SADDLE_BROWN: Self = rgba(139, 69, 19, 255);
    /// Salmon: https://rgb.to/salmon
    pub const SALMON: Self = rgba(250, 128, 114, 255);
    /// Sandybrown: https://rgb.to/sandybrown
    pub const SANDY_BROWN: Self = rgba(244, 164, 96, 255);
    /// Seagreen: https://rgb.to/seagreen
    pub const SEA_GREEN: Self = rgba(46, 139, 87, 255);
    /// Seashell: https://rgb.to/seashell
    pub const SEA_SHELL: Self = rgba(255, 245, 238, 255);
    /// Sienna: https://rgb.to/sienna
    pub const SIENNA: Self = rgba(160, 82, 45, 255);
    /// Silver: https://rgb.to/silver
    pub const SILVER: Self = rgba(192, 192, 192, 255);
    /// Skyblue: https://rgb.to/skyblue
    pub const SKY_BLUE: Self = rgba(135, 206, 235, 255);
    /// Slateblue: https://rgb.to/slateblue
    pub const SLATE_BLUE: Self = rgba(106, 90, 205, 255);
    /// Slategray: https://rgb.to/slategray
    pub const SLATE_GRAY: Self = rgba(112, 128, 144, 255);
    /// Slategrey: https://rgb.to/slategrey
    pub const SLATE_GREY: Self = rgba(112, 128, 144, 255);
    /// Snow: https://rgb.to/snow
    pub const SNOW: Self = rgba(255, 250, 250, 255);
    /// Springgreen: https://rgb.to/springgreen
    pub const SPRING_GREEN: Self = rgba(0, 255, 127, 255);
    /// Steelblue: https://rgb.to/steelblue
    pub const STEEL_BLUE: Self = rgba(70, 130, 180, 255);
    /// Tan: https://rgb.to/tan
    pub const TAN: Self = rgba(210, 180, 140, 255);
    /// Teal: https://rgb.to/teal
    pub const TEAL: Self = rgba(0, 128, 128, 255);
    /// Thistle: https://rgb.to/thistle
    pub const THISTLE: Self = rgba(216, 191, 216, 255);
    /// Tomato: https://rgb.to/tomato
    pub const TOMATO: Self = rgba(255, 99, 71, 255);
    /// Turquoise: https://rgb.to/turquoise
    pub const TURQUOISE: Self = rgba(64, 224, 208, 255);
    /// Violet: https://rgb.to/violet
    pub const VIOLET: Self = rgba(238, 130, 238, 255);
    /// Wheat: https://rgb.to/wheat
    pub const WHEAT: Self = rgba(245, 222, 179, 255);
    /// White: https://rgb.to/white
    pub const WHITE: Self = rgba(255, 255, 255, 255);
    /// Whitesmoke: https://rgb.to/whitesmoke
    pub const WHITE_SMOKE: Self = rgba(245, 245, 245, 255);
    /// Yellow: https://rgb.to/yellow
    pub const YELLOW: Self = rgba(255, 255, 0, 255);
    /// Yellowgreen: https://rgb.to/yellowgreen
    pub const YELLOW_GREEN: Self = rgba(154, 205, 5, 255);

    /// Create a color from the red, green, blue and alpha channel values.
    #[inline]
    pub const fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Create a color from the red, green and blue channel values.
    ///
    /// The alpha value is always 255.
    #[inline]
    pub const fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    /// Create a color from a [u32] packed color value.
    #[inline]
    pub fn from_u32(rgba: u32) -> Self {
        rgba.into()
    }

    /// Create a color from the alpha, red, green and blue channel values.
    #[inline]
    pub const fn from_argb(a: u8, r: u8, g: u8, b: u8) -> Self {
        Self::from_rgba(r, g, b, a)
    }

    /// Create a color from the blue, green, red and alpha channel values.
    #[inline]
    pub const fn from_bgra(b: u8, g: u8, r: u8, a: u8) -> Self {
        Self::from_rgba(r, g, b, a)
    }

    #[inline]
    pub fn from_rgba_float<R: Real>(r: R, g: R, b: R, a: R) -> Self {
        Self {
            r: (r.to_f32() * 255.0).round() as u8,
            g: (g.to_f32() * 255.0).round() as u8,
            b: (b.to_f32() * 255.0).round() as u8,
            a: (a.to_f32() * 255.0).round() as u8,
        }
    }

    /// Create a color from the red, green and blue channel values.
    ///
    /// The alpha value is always 1.0.
    #[inline]
    pub fn from_rgb_float<R: Real>(r: R, g: R, b: R) -> Self {
        Self::from_rgba_float(r, g, b, R::from_f32(1.0))
    }

    #[inline]
    pub fn from_argb_float<R: Real>(a: R, r: R, g: R, b: R) -> Self {
        Self::from_rgba_float(r, g, b, a)
    }

    #[inline]
    pub fn from_bgra_float<R: Real>(b: R, g: R, r: R, a: R) -> Self {
        Self::from_rgba_float(r, g, b, a)
    }

    pub fn from_hsl<R: Real>(h: R, s: R, l: R) -> Self {
        let h = h.to_f32();
        let s = s.to_f32();
        let l = l.to_f32();

        #[inline]
        fn percent_to_byte(percent: f32) -> u8 {
            (percent * 255.0).round() as u8
        }

        if s == 0.0 {
            // Achromatic, i.e., grey.
            let l = percent_to_byte(l);
            return Self::from_rgba(l, l, l, 255);
        }

        let h = h / 360.0; // treat this as 0..1 instead of degrees

        let q = if l < 0.5 {
            l * (1.0 + s)
        } else {
            l + s - (l * s)
        };
        let p = 2.0 * l - q;

        Self {
            r: percent_to_byte(hue_to_rgb(p, q, h + 1.0 / 3.0)),
            g: percent_to_byte(hue_to_rgb(p, q, h)),
            b: percent_to_byte(hue_to_rgb(p, q, h - 1.0 / 3.0)),
            a: 255,
        }
    }

    pub fn to_hsl(&self) -> (f32, f32, f32) {
        use std::cmp::{max, min};

        let mut h: f32;
        let (r, g, b) = (self.r, self.g, self.b);

        let max = max(max(r, g), b);
        let min = min(min(r, g), b);

        // divide everything by 255 to get percentages of colors
        let (r, g, b) = (r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0);
        let (min, max) = (min as f32 / 255.0, max as f32 / 255.0);

        // luminosity is the average of the max and min rgb color intensities
        let l: f32 = (max + min) / 2.0;

        // saturation
        let delta: f32 = max - min;
        if delta == 0.0 {
            // it's gray
            return (0.0, 0.0, l);
        }

        // it's not gray
        let s = if l < 0.5 {
            delta / (max + min)
        } else {
            delta / (2.0 - max - min)
        };

        // hue
        let r2 = (((max - r) / 6.0) + (delta / 2.0)) / delta;
        let g2 = (((max - g) / 6.0) + (delta / 2.0)) / delta;
        let b2 = (((max - b) / 6.0) + (delta / 2.0)) / delta;

        h = match max {
            x if x == r => b2 - g2,
            x if x == g => (1.0 / 3.0) + r2 - b2,
            _ => (2.0 / 3.0) + g2 - r2,
        };

        // fix wraparounds
        if h < 0 as f32 {
            h += 1.0;
        } else if h > 1.0 {
            h -= 1.0;
        }

        ((h * 360.0 * 100.0).round() / 100.0, s, l)
    }

    #[inline]
    pub fn set_r(&mut self, r: u8) -> &mut Self {
        self.r = r;
        self
    }

    #[inline]
    pub fn set_g(&mut self, g: u8) -> &mut Self {
        self.g = g;
        self
    }

    #[inline]
    pub fn set_b(&mut self, b: u8) -> &mut Self {
        self.b = b;
        self
    }

    #[inline]
    pub fn set_a(&mut self, a: u8) -> &mut Self {
        self.a = a;
        self
    }
}

// Convinience

impl From<Color> for u32 {
    #[inline]
    fn from(col: Color) -> Self {
        ((col.r as u32) << 24) | ((col.g as u32) << 16) | ((col.b as u32) << 8) | (col.a as u32)
    }
}

impl From<u32> for Color {
    #[inline]
    fn from(packed: u32) -> Self {
        Self {
            r: ((packed >> 24) & 0xFF) as u8,
            g: ((packed >> 16) & 0xFF) as u8,
            b: ((packed >> 8) & 0xFF) as u8,
            a: (packed & 0xFF) as u8,
        }
    }
}

impl From<Color> for (u8, u8, u8, u8) {
    #[inline]
    fn from(col: Color) -> Self {
        (col.r, col.g, col.b, col.a)
    }
}

impl From<Color> for [u8; 4] {
    #[inline]
    fn from(col: Color) -> Self {
        [col.r, col.g, col.b, col.a]
    }
}

impl From<Color> for (u8, u8, u8) {
    #[inline]
    fn from(col: Color) -> Self {
        (col.r, col.g, col.b)
    }
}

impl From<Color> for [u8; 3] {
    #[inline]
    fn from(col: Color) -> Self {
        [col.r, col.g, col.b]
    }
}

impl std::ops::Index<usize> for Color {
    type Output = u8;

    #[inline]
    fn index(&self, index: usize) -> &u8 {
        match index {
            0 => &self.r,
            1 => &self.g,
            2 => &self.b,
            3 => &self.a,
            _ => panic!("Color index out of bounds: {index}"),
        }
    }
}

impl std::ops::IndexMut<usize> for Color {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut u8 {
        match index {
            0 => &mut self.r,
            1 => &mut self.g,
            2 => &mut self.b,
            3 => &mut self.a,
            _ => panic!("Color index out of bounds: {index}"),
        }
    }
}

/// Shorthand for `Color::from_rgba(r, g, b, a)`.
#[inline(always)]
pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
    Color::from_rgba(r, g, b, a)
}

/// Shorthand for `Color::from_rgb(r, g, b)`.
#[inline(always)]
pub const fn rgb(r: u8, g: u8, b: u8) -> Color {
    Color::from_rgb(r, g, b)
}

/// Shorthand for `Color::from_argb(a, r, g, b)`.
#[inline(always)]
pub const fn argb(a: u8, r: u8, g: u8, b: u8) -> Color {
    Color::from_argb(a, r, g, b)
}

/// Shorthand for `Color::from_bgra(b, g, r, a)`.
#[inline(always)]
pub const fn bgra(b: u8, g: u8, r: u8, a: u8) -> Color {
    Color::from_bgra(b, g, r, a)
}

#[cfg(test)]
mod tests {
    use crate::rgba;

    #[test]
    fn packing_and_unpacking() {
        use super::*;
        let color = rgba(255, 254, 253, 252);
        let packed: u32 = color.into();
        let unpacked: Color = packed.into();
        assert_eq!(color, unpacked);
    }

    #[test]
    fn color_from_hsl() {
        // hsl: 39°, 96%, 49%
        use super::*;
        let from_hsl = Color::from_hsl(39.0, 0.96, 0.49);
        assert_eq!(from_hsl, rgba(245, 161, 5, 255));
    }

    #[test]
    fn convinience_tests() {
        use super::*;
        assert_eq!(rgba(1, 2, 3, 4), Color::from_rgba(1, 2, 3, 4));
        assert_eq!(
            Color::from_rgba(1, 2, 3, 4),
            Color {
                r: 1,
                g: 2,
                b: 3,
                a: 4,
            }
        );

        assert_eq!(rgb(1, 2, 3), Color::from_rgb(1, 2, 3));
        assert_eq!(
            Color::from_rgb(1, 2, 3),
            Color {
                r: 1,
                g: 2,
                b: 3,
                a: 255,
            }
        );

        assert_eq!(argb(1, 2, 3, 4), Color::from_argb(1, 2, 3, 4));
        assert_eq!(
            Color::from_argb(1, 2, 3, 4),
            Color {
                r: 2,
                g: 3,
                b: 4,
                a: 1,
            }
        );

        assert_eq!(bgra(1, 2, 3, 4), Color::from_bgra(1, 2, 3, 4));
        assert_eq!(
            Color::from_bgra(1, 2, 3, 4),
            Color {
                r: 3,
                g: 2,
                b: 1,
                a: 4,
            }
        );
    }

    #[test]
    fn index_test() {
        let col = rgba(0xde, 0xad, 0xbe, 0xef);
        assert_eq!(col[0], 0xde);
        assert_eq!(col[1], 0xad);
        assert_eq!(col[2], 0xbe);
        assert_eq!(col[3], 0xef);
    }
}
