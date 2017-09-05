//! Handlers for special routes and error catchers. **Has routes.**
//!
//! Handles the following special routes:
//!
//! - GET `/static/...`: static files
//! - GET `/`: the index page
//!
//! It also defines catchers for certain errors (like 403 and 404).

pub mod catchers;
pub mod routes;
