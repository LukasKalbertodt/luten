
pub const WEBSITE_TITLE: &str = "Testatverwaltung";

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


pub const INITIAL_REQ_COOKIE_NAME: &str = "initial_request_path";
