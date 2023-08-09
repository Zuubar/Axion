#[macro_export]
macro_rules! impl_axion_json_for_builtins {
    ($($ty:ty),*) => {
        $(impl AxionJson for $ty {
            fn to_json(&self) -> String {
                format!("{:?}", self)
            }
        })*
    };
}

#[macro_export]
macro_rules! impl_axion_json_for_collections {
    ($($ty:ty),*) => {
        $(impl<T: AxionJson> AxionJson for $ty {
            fn to_json(&self) -> String {
                format!("[{}]", self.iter().map(|i| i.to_json()).collect::<Vec<_>>().join(","))
            }
        })*
    };
}
