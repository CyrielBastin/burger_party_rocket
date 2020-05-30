use rocket_contrib::templates::Template;
use rocket_contrib::templates::tera::Context;
use rocket::request::Form;
use rocket::response::Redirect;
use crate::data_access::{DAOFactory, DAO};
use crate::validators::command_validator::are_datas_valid;
use crate::data_access::command_details_handler::{write_cmd_details, empty_command_details_content};

//==================================================================================================
// All routes ares prefixed with /command
//==================================================================================================

#[get("/new")]
pub fn command_new() -> Template
{
    let mut burger_repo = DAOFactory::create_dao_burger();
    let list_burgers = burger_repo.find_all();
    let mut drink_repo = DAOFactory::create_dao_drink();
    let list_drinks = drink_repo.find_all();

    let mut context = Context::new();
    context.insert("burgers", &list_burgers);
    context.insert("drinks", &list_drinks);

    Template::render("command/_command_new", context)
}

#[get("/reset")]
pub fn command_reset() -> Redirect
{
    empty_command_details_content().unwrap();

    Redirect::to(format!("/command{}", uri!(command_new)))
}

#[get("/product/add/burger/<id>")]
pub fn command_add_burger(id: u32) -> Template
{
    let mut burger_repo = DAOFactory::create_dao_burger();
    let burger = burger_repo.find_by_id(id);

    let mut context = Context::new();
    context.insert("burger", &burger);

    Template::render("command/add_product", context)
}

#[get("/product/add/drink/<id>")]
pub fn command_add_drink(id: u32) -> Template
{
    let mut drink_repo = DAOFactory::create_dao_drink();
    let drink = drink_repo.find_by_id(id);

    let mut context = Context::new();
    context.insert("drink", &drink);

    Template::render("command/add_product", context)
}


/*
 * Structure to receive the kind (burger or drink), its `id` and `quantity` to add to the command
 */
#[derive(FromForm, Debug)]
pub struct CmdQty
{
    pub kind: String,
    pub id: u32,
    pub quantity: u8
}

#[post("/details/set", data = "<cmd_det>")]
pub fn set_cmd_details(cmd_det: Form<CmdQty>) -> Redirect
{
    if are_datas_valid(&cmd_det)
    {
        write_cmd_details(&cmd_det);
    }

    Redirect::to(format!("/command{}", uri!(command_new)))
}