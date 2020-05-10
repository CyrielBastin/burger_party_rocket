use rocket_contrib::templates::Template;
use rocket_contrib::templates::tera::Context;
use crate::data_access::{DAOFactory, DAO};

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

    Template::render("commande/commande_new", context)
}
