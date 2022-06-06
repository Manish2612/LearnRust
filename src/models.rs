use std::time::SystemTime;

#[derive(Queryable)]
pub struct Users {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub created_on: SystemTime // Timestamp was not working so had to use other compatible type for date field
}
