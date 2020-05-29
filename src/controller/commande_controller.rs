use rocket_contrib::templates::Template;
use rocket_contrib::templates::tera::Context;
use rocket::request::Form;
use rocket::response::Redirect;
use crate::data_access::{DAOFactory, DAO};
use crate::validators::commande_validator::are_datas_valid;
use crate::data_access::command_details_handler::{write_cmd_details, empty_command_details_content};

//==================================================================================================
// All routes ares prefixed with /commande
//==================================================================================================

#[get("/new")]
pub fn commande_new() -> Template
{
    let mut burger_repo = DAOFactory::create_dao_burger();
    let list_burgers = burger_repo.find_all();
    let mut boisson_repo = DAOFactory::create_dao_boisson();
    let list_boissons = boisson_repo.find_all();

    let mut context = Context::new();
    context.insert("burgers", &list_burgers);
    context.insert("boissons", &list_boissons);

    Template::render("commande/_commande_new", context)
}

#[get("/reset")]
pub fn commande_reset() -> Redirect
{
    empty_command_details_content().unwrap();

    Redirect::to(format!("/commande{}", uri!(commande_new)))
}

#[get("/product/add/burger/<id>")]
pub fn commande_add_burger(id: u32) -> Template
{
    let mut burger_repo = DAOFactory::create_dao_burger();
    let burger = burger_repo.find_by_id(id);

    let mut context = Context::new();
    context.insert("burger", &burger);

    Template::render("commande/add_product", context)
}

#[get("/product/add/boisson/<id>")]
pub fn commande_add_boisson(id: u32) -> Template
{
    let mut boisson_repo = DAOFactory::create_dao_boisson();
    let boisson = boisson_repo.find_by_id(id);

    let mut context = Context::new();
    context.insert("boisson", &boisson);

    Template::render("commande/add_product", context)
}


/*
 * Structure to receive the kind (burger or drink), its `id` and `quantity` to add to the command
 */
#[derive(FromForm, Debug)]
pub struct CmdQte
{
    pub kind: String,
    pub id: u32,
    pub quantite: u8
}

#[post("/details/set", data = "<cmd_det>")]
pub fn set_cmd_details(cmd_det: Form<CmdQte>) -> Redirect
{
    if are_datas_valid(&cmd_det)
    {
        write_cmd_details(&cmd_det);
    }

    Redirect::to(format!("/commande{}", uri!(commande_new)))
}
