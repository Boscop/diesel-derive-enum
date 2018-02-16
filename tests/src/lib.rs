#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_derive_enum;

mod common;
mod simple;
mod rename;
mod nullable;
