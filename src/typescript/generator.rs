use ts_rs::TS;

pub fn generate_typescript_bindings() {
    println!("cargo:rerun-if-changed=src/schema");
}

#[macro_export]
macro_rules! derive_ts {
    ($($t:ty),*) => {
        $(
            const _: () = assert_ts!($t);
        )*
    };
}

pub trait ToTypeScript {
    fn to_typescript() -> String;
}

impl<T: TS + 'static> ToTypeScript for T {
    fn to_typescript() -> String {
        T::inline()
    }
}
