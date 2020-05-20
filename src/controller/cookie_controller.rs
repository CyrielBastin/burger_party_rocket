use rocket::response::Redirect;

/*
 * Handle all requests that will setup Cookies
 */

//==================================================================================================
// All routes ares prefixed with /cookie
//==================================================================================================

pub fn set_cookie() -> Redirect
{
    Redirect::to(uri!(super::homepage_controller::index))
}
