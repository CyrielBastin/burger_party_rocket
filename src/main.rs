#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket_contrib::templates::Template;

mod data_access;
mod entity;
mod types;
mod controller;
mod validators;


fn main()
{
    rocket::ignite()
        .mount("/", routes![
            controller::homepage_controller::index,
            controller::homepage_controller::who_are_we])
        .mount("/public", routes![
            controller::public_resources_controller::get_css,
            controller::public_resources_controller::get_font,
            controller::public_resources_controller::get_image,
            controller::public_resources_controller::get_js,
            controller::public_resources_controller::get_burger,
            controller::public_resources_controller::get_drink])
        .mount("/informations-products", routes![
            controller::infos_prod_controller::infos_burgers,
            controller::infos_prod_controller::infos_drinks,
            controller::infos_prod_controller::details_burger,
            controller::infos_prod_controller::details_drink])
        .mount("/command", routes![
            controller::command_controller::command_new,
            controller::command_controller::command_reset,
            controller::command_controller::command_add_burger,
            controller::command_controller::command_add_drink,
            controller::command_controller::command_details,
            controller::command_controller::set_cmd_details,
            controller::command_controller::fetch_burgers,
            controller::command_controller::fetch_drinks])
        .attach(Template::fairing())
        .launch();
}
