pub use gloo_console;

#[allow(unused_macros)]
#[macro_export]
macro_rules! file_line {
    ($macros:ident!($($expr:expr),*)) => {
        $crate::gloo_console::$macros!("%c%s:%i", "color: grey", file!(), line!(), $($expr),*)
    }
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! debug { ($($expr:expr),*) => { $crate::file_line!(debug!($($expr),*)) } }

#[allow(unused_macros)]
#[macro_export]
macro_rules! info { ($($expr:expr),*) => { $crate::file_line!(info!($($expr),*)) } }

#[allow(unused_macros)]
#[macro_export]
macro_rules! log { ($($expr:expr),*) => { $crate::file_line!(log!($($expr),*)) }; }

#[allow(unused_macros)]
#[macro_export]
macro_rules! warn { ($($expr:expr),*) => { $crate::file_line!(warn!($($expr),*)) }; }

#[allow(unused_macros)]
#[macro_export]
macro_rules! error { ($($expr:expr),*) => { $crate::file_line!(error!($($expr),*)) }; }
