use rocket_contrib::templates::Template;
use std::collections::HashMap;

#[get("/")]
pub fn index() -> Template
{
    let context: HashMap<u8, u8> = HashMap::new();

    Template::render("index", context)
}

#[get("/qui-sommes-nous")]
pub fn qui_sommes_nous() -> Template
{
    let context: HashMap<u8, u8> = HashMap::new();

    Template::render("qui_sommes_nous", context)
}
