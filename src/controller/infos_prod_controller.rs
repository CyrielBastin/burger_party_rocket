use rocket_contrib::templates::Template;
use rocket_contrib::templates::tera::Context;
use crate::data_access::{DAOFactory, DAO};

//==================================================================================================
// All routes ares prefixed with /informations-products
//==================================================================================================

#[get("/burgers")]
pub fn infos_burgers() -> Template
{
    let mut burger_repo = DAOFactory::create_dao_burger();
    let list_burgers = burger_repo.find_all();
    let mut context = Context::new();
    context.insert("burgers", &list_burgers);

    Template::render("infos_products/infos_burgers", context)
}

#[get("/drinks")]
pub fn infos_drinks() -> Template
{
    let mut drink_repo = DAOFactory::create_dao_drink();
    let list_drinks = drink_repo.find_all();
    let mut context = Context::new();
    context.insert("drinks", &list_drinks);

    Template::render("infos_products/infos_drinks", context)
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
        calories += (*ing).get_calories() * (*ing).get_quantity() as u16;
    }

    context.insert("burger", &burger);
    context.insert("calories", &calories);

    Template::render("infos_products/details_burgers", context)
}

#[get("/drinks/drink/<id>")]
pub fn details_drink(id: u32) -> Template
{
    let mut drink_repo = DAOFactory::create_dao_drink();
    let drink = drink_repo.find_by_id(id);
    let mut context = Context::new();
    context.insert("drink", &drink);

    Template::render("infos_products/details_drinks", context)
}
