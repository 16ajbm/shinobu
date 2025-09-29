pub trait Clampable: Copy {
    fn clamp_from_f32(val: f32) -> Self;
}

impl Clampable for u8 {
    fn clamp_from_f32(val: f32) -> Self {
        val.round().clamp(0.0, 255.0) as u8
    }
}

impl Clampable for u16 {
    fn clamp_from_f32(val: f32) -> Self {
        val.round().clamp(0.0, 65535.0) as u16
    }
}

impl Clampable for f32 {
    fn clamp_from_f32(val: f32) -> Self {
        val.clamp(0.0, 1.0)
    }
}
