use amplify_derive::{From, Wrapper};

#[derive(Default, From, Wrapper)]
pub struct Color {
    inner: u32,
}

impl Color {
    pub const WHITE: Color = Color { inner: 0xffffff };

    pub const fn hex(hex: u32) -> Color {
        Color { inner: hex }
    }
}

// TODO
// impl From<(u8, u8, u8)> for Color {
//     fn from(color: (u8, u8, u8)) -> Color {
//         let (r, g, b) = color;

//         Color {
//             inner: (r as f64, g as f64, b as f64),
//         }
//     }
// }

impl Into<rpt::Color> for Color {
    fn into(self) -> rpt::Color {
        rpt::hex_color(self.inner)
    }
}
