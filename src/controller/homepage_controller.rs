use rocket_contrib::templates::Template;
use rocket_contrib::templates::tera::Context;

#[get("/")]
pub fn index() -> Template
{
    let context = Context::new();

    Template::render("index", context)
}

#[get("/qui-sommes-nous")]
pub fn qui_sommes_nous() -> Template
{
    let context = Context::new();

    Template::render("qui_sommes_nous", context)
}
