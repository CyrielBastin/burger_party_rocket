use rocket_contrib::templates::Template;
use rocket_contrib::templates::tera::Context;
use crate::data_access::{DAOFactory, DAO};

//==================================================================================================
// All routes ares prefixed with /informations-produits
//==================================================================================================

#[get("/burgers")]
pub fn infos_burgers() -> Template
{
    let mut burger_repo = DAOFactory::create_dao_burger();
    let list_burgers = burger_repo.find_all();
    let mut context = Context::new();
    context.insert("burgers", &list_burgers);

    Template::render("infos_produits/infos_burgers", context)
}

#[get("/boissons")]
pub fn infos_boissons() -> Template
{
    let mut boisson_repo = DAOFactory::create_dao_boisson();
    let list_boissons = boisson_repo.find_all();
    let mut context = Context::new();
    context.insert("boissons", &list_boissons);

    Template::render("infos_produits/infos_boissons", context)
}
