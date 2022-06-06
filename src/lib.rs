// need to add macro_use macro to make schema.rs and models.rs work.
// it maked diesel avaialble avaialable so we need not to import everywhere

#[macro_use]
extern crate diesel;

pub mod schema;
pub mod models;
