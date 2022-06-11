use diesel::pg::PgConnection;
//need to import below crate for using load(), expect() etc.. from diesel
use diesel::prelude::*;

// import table object to use it in query
use server_test::schema::todos::dsl::*;
use server_test::schema::users::dsl::*;

use serde::{Serialize, Deserialize};

use server_test::models::*;

pub fn getUsersList(con: &PgConnection) {
    let usersList = users
        .limit(5)
        .load::<User>(con)
        .expect("Error loading users");

    println!("\nFetch 5 users from table ******************** ");
    for usr in usersList {
        println!("{} using pwd={} ", usr.username, usr.password);
    }
}

pub fn getTodosList(con: &PgConnection) {
    let todoList = todos.limit(5).load::<Todo>(con).expect("cant get todos");

    println!("\nFetch 5 todos from table ******************** ");
    for todo in todoList {
        let json: String = serde_json::to_string(&todo).unwrap();
        println!("TODO JSON :: {}", json);
    }
}

pub fn joinUserTodo(con: &PgConnection) {
    let join_data = users
        .inner_join(todos)
        // While joining tables we need to specify touple with Schemas
        .load::<(User, Todo)>(con)
        .expect("cant get join data");

    println!("\nJoin Users and Todos tables******************** ");
    for val in join_data {
        let rowJson = serde_json::to_string(&val).unwrap();
        println!("Join Data :: {}", rowJson);
    }
}

pub fn joinWithSelectedColumns(con: &PgConnection) {
    let selectCols: Vec<(String, String, Option<bool>)> = users
        .inner_join(todos)
        .select((username, task, is_completed))
        // We are selecting perticular columns with join so need to specify touple with types
        .load::<(String, String, Option<bool>)>(con)
        .expect("FAILED:: lib -> joinWithSelectedColumns() ");

    // Creating schema to store results returned by join.
    // with these after serialisation we will data in key-value pair
    #[derive(Serialize, Deserialize)]
    struct UserWithTask {
        name: String,
        task: String,
        // db has this column with allow null true, thats why using Option<dataType>
        completed: Option<bool>,
    }
    impl UserWithTask {
        // New function to assign data while creating User with data retrieved from DB
        fn new(name: String, taskName: String, completed: Option<bool>) -> UserWithTask {
            UserWithTask {
                name: name,
                task: taskName,
                completed: completed,
            }
        }
    }
    println!("\nSelect column from Users and Todos tables Join ******************** ");
    for item in selectCols {
        // accessing touple values returned by query using object notation followed by index
        let userData:UserWithTask = UserWithTask::new(item.0,item.1,item.2);
        // when we serialize schema it will convert Option value for completed correctly and return JSON
        let json = serde_json::to_string(&userData).unwrap();
        println!("Selected columns :: {}", json);
    }
}
