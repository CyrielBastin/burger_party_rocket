use rocket_contrib::templates::Template;
use rocket_contrib::templates::tera::Context;
use rocket::request::Form;
use rocket::response::Redirect;
use crate::data_access::{DAOFactory, DAO};
use crate::validators::command_validator::are_datas_valid;
use crate::data_access::command_details_handler::{write_cmd_details, empty_command_details_content, fetch_cmd_burgers, fetch_cmd_drinks, fetch_cmd_by_datetime};
use crate::entity::Command;
use crate::entity::command::get_local_to_string;

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

#[get("/details/overview")]
pub fn command_details() -> Template
{
    let context = Context::new();

    Template::render("command/command_details", context)
}

#[get("/payed-and-accepted")]
pub fn command_payed_and_accepted() -> Template
{
    let mut dao_command = DAOFactory::create_dao_command();
    let mut command = Command::new();
    let date_time = get_local_to_string();
    command.set_date_time(&date_time);
    command.append_burgers_and_drinks();

    dao_command.create(command);

    let _ = empty_command_details_content();

    let mut context = Context::new();
    context.insert("datetime", &date_time);

    Template::render("command/command_payed_and_accepted", context)
}

//==================================================================================================
// Structure to receive the kind (burger or drink), its `id` and `quantity` to add to the command
//==================================================================================================
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

//==================================================================================================
// Section to fetch burgers and drinks from command
//==================================================================================================

#[get("/fetch/burgers")]
pub fn fetch_burgers() -> Option<String>
{
    fetch_cmd_burgers()
}

#[get("/fetch/drinks")]
pub fn fetch_drinks() -> Option<String>
{
    fetch_cmd_drinks()
}

#[get("/fetch/command/<datetime>")]
pub fn fetch_command(datetime: String) -> Option<String>
{
    fetch_cmd_by_datetime(&datetime)
}
