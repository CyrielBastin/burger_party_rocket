use rocket_contrib::templates::Template;
use rocket_contrib::templates::tera::Context;
use crate::data_access::{DAOFactory, DAO};

//==================================================================================================
// All routes ares prefixed with /informations-produits
//==================================================================================================

#[get("/burger")]
pub fn infos_produits() -> Template
{
    let mut burger_repo = DAOFactory::create_dao_burger();
    let list_burgers = burger_repo.find_all();
    let mut context = Context::new();
    context.insert("burgers", &list_burgers);

    Template::render("infos_produits/infos_produits", context)
}
