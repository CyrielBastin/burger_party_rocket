use rocket_contrib::templates::Template;
use rocket_contrib::templates::tera::Context;

#[get("/")]
pub fn index() -> Template
{
    let context = Context::new();

    Template::render("index", context)
}

#[get("/who-are-we")]
pub fn who_are_we() -> Template
{
    let context = Context::new();

    Template::render("who_are_we", context)
}
