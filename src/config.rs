/// The main title of the website.
///
/// This title is used in the HTML `<title>` tag, in the nav bar and
/// potentially in a few other places.
pub const WEBSITE_TITLE: &str = "Info-A Testate";

/// The name for the cookie containing the session id.
pub const SESSION_COOKIE_NAME: &str = "session";

/// Length of the session id in bytes. 128 bit seems to be enough entropy
/// according to those sources:
///
/// - https://security.stackexchange.com/a/24852/147555
/// - https://security.stackexchange.com/a/138396/147555
///
/// If you change this value, you also have to change the database scheme,
/// since the length is checked there, too.
pub const SESSION_ID_LEN: usize = 16;

/// When a non-logged-in user requests a route that cannot be accessed when not
/// logged in, they are redirected to `/login`. After a successful login, the
/// user will be redirected to the route they initially requested. To store
/// this route, we use a cookie. The name of that cookie is defined here.
pub const INITIAL_REQ_COOKIE_NAME: &str = "initial_request_path";
