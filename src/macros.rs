
macro_rules! decl_api {
    (
        $prefix:ident {
            $(
                $(#[$fn_meta:meta])*
                fn $fn_name:ident($($arg:ident: $arg_ty:ty),*$(,)?) -> $ret:ty;
            )*
        }
    ) => {
        #[derive(Debug)]
        pub struct API {
            __library: ::libloading::Library,

            $(
                $(#[$fn_meta])*
                pub $fn_name: unsafe extern "C" fn($($arg: $arg_ty),*) -> $ret,
            )*
        }

        impl API {
            pub unsafe fn new<P>(path: P) -> Result<Self, ::libloading::Error>
            where
                P: AsRef<::std::ffi::OsStr>,
            {
                let library = ::libloading::Library::new(path)?;
                Self::from_library(library)
            }

            pub unsafe fn from_library<L: Into<::libloading::Library>>(library: L) -> Result<Self, ::libloading::Error> {
                let library = library.into();

                $(
                    let $fn_name = library
                        .get(concat!(stringify!($prefix), "_", stringify!($fn_name), "\0").as_bytes())
                        .map(|sym| *sym)?;
                )*

                Ok(Self {
                    __library: library,
                    $(
                        $fn_name,
                    )*
                })
            }
        }
    };
}

pub(crate) use decl_api;