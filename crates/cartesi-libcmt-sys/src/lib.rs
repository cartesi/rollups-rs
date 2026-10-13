#![doc = include_str!("../README.md")]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
// The generated items carry libcmt's Doxygen comments (`@param [in]`, ...),
// which rustdoc would otherwise read as intra-doc links and HTML tags.
#![allow(rustdoc::broken_intra_doc_links)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::bare_urls)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
