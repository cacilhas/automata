pub trait FromF64: Copy {
    fn from_f64(v: f64) -> Self;
}

macro_rules! impl_from_f64 {
    ($($ty:ty)*) => {
        $(
            impl FromF64 for $ty {
                #[inline]
                fn from_f64(v: f64) -> $ty {
                    v as $ty
                }
            }
        )*
    }
}

impl_from_f64!(i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 usize);
