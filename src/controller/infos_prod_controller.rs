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

#[get("/burgers/burger/<id>")]
pub fn details_burger(id: u32) -> Template
{
    let mut burger_repo = DAOFactory::create_dao_burger();
    let burger = burger_repo.find_by_id(id);
    let mut context = Context::new();

    let mut calories: u16 = 0;
    for ing in burger.get_ingredients()
    {
        calories += (*ing).get_calories() * (*ing).get_quantite() as u16;
    }

    context.insert("burger", &burger);
    context.insert("calories", &calories);

    Template::render("infos_produits/details_burgers", context)
}

#[get("/boissons/boisson/<id>")]
pub fn details_boisson(id: u32) -> Template
{
    let mut boisson_repo = DAOFactory::create_dao_boisson();
    let boisson = boisson_repo.find_by_id(id);
    let mut context = Context::new();
    context.insert("boisson", &boisson);

    Template::render("infos_produits/details_boissons", context)
}
