#[macro_export]
macro_rules! impl_new
{
    ($to:ty,$($v:ident: $t:ty),*)  => {

        impl $to {
            pub fn new($($v: $t),*) -> $to
            {
                Self {
                    $($v),*
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_default_new {
    ($to:ty) => {
        impl Default for $to {
            fn default() -> Self {
                Self::new()
            }
        }
    };
}

#[macro_export]
macro_rules! impl_default_new_with_params {
    ($to:ty, $($t:literal),*)  => {
        impl Default for $to {
            fn default() -> Self { Self::new($($t),*) }
        }
    };

}
