#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket_contrib::templates::Template;

mod data_access;
mod entity;
mod types;
mod controller;


fn main()
{
    rocket::ignite()
        .mount("/", routes![
            controller::homepage_controller::index,
            controller::homepage_controller::qui_sommes_nous])
        .mount("/public", routes![
            controller::public_resources_controller::get_css,
            controller::public_resources_controller::get_font,
            controller::public_resources_controller::get_image,
            controller::public_resources_controller::get_js])
        .mount("/informations-produits", routes![
            controller::infos_prod_controller::infos_burgers,
            controller::infos_prod_controller::infos_boissons,
            controller::infos_prod_controller::details_burger,
            controller::infos_prod_controller::details_boisson])
        .mount("/commande", routes![
            controller::commande_controller::commande_new,
            controller::commande_controller::commande_add_burger,
            controller::commande_controller::commande_add_boisson])
        .attach(Template::fairing())
        .launch();
}
