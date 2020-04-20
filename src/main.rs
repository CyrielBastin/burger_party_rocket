#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod data_access;
mod entity;
mod types;
mod controller;


fn main()
{
    rocket::ignite().mount("/", routes![controller::homepage_controller::index]).launch();
}
