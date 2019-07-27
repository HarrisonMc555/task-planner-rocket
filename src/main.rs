#![feature(proc_macro_hygiene)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate rocket_contrib;

mod plan;
mod task;
#[cfg(test)]
mod tests;

use diesel::{prelude::*, SqliteConnection};
use rocket::fairing::AdHoc;
use rocket::request::{FlashMessage, Form};
use rocket::response::{Flash, Redirect};
use rocket::Rocket;
use rocket_contrib::{serve::StaticFiles, templates::Template};

use crate::plan::Plan;
use crate::task::{InsertableTask, Task, TaskFormInput};

// This macro from `diesel_migrations` defines an `embedded_migrations` module
// containing a function named `run`. This allows the example to be run and
// tested without any outside setup of the database.
embed_migrations!();

#[database("sqlite_database")]
pub struct DbConn(SqliteConnection);

#[derive(Serialize)]
struct Context<'a, 'b> {
    msg: Option<(&'a str, &'b str)>,
    tasks_with_plans: Vec<(Task, Vec<Plan>)>,
}

fn get_tasks_with_plans(conn: &DbConn) -> Vec<(Task, Vec<Plan>)> {
    let tasks = Task::all(conn);
    let plans = Plan::belonging_to(&tasks)
        .load(&conn.0)
        .expect("Error loading tasks")
        .grouped_by(&tasks);
    tasks.into_iter().zip(plans).collect::<Vec<_>>()
}

impl<'a, 'b> Context<'a, 'b> {
    pub fn err(conn: &DbConn, msg: &'a str) -> Context<'static, 'a> {
        Context {
            msg: Some(("error", msg)),
            tasks_with_plans: get_tasks_with_plans(conn),
        }
    }

    pub fn raw(conn: &DbConn, msg: Option<(&'a str, &'b str)>) -> Context<'a, 'b> {
        Context {
            msg: msg,
            tasks_with_plans: get_tasks_with_plans(conn),
        }
    }
}

#[post("/", data = "<task_form>")]
fn new(task_form: Form<TaskFormInput>, conn: DbConn) -> Flash<Redirect> {
    let task_form_input = task_form.into_inner();
    if task_form_input.description.is_empty() {
        Flash::error(Redirect::to("/"), "Description cannot be empty.")
    } else if InsertableTask::insert(task_form_input, &conn) {
        Flash::success(Redirect::to("/"), "Task successfully added.")
    } else {
        Flash::error(Redirect::to("/"), "Whoops! The server failed.")
    }
}

#[put("/<id>")]
fn toggle(id: i32, conn: DbConn) -> Result<Redirect, Template> {
    if Task::toggle_with_id(id, &conn) {
        Ok(Redirect::to("/"))
    } else {
        Err(Template::render(
            "index",
            &Context::err(&conn, "Couldn't toggle task."),
        ))
    }
}

#[delete("/<id>")]
fn delete(id: i32, conn: DbConn) -> Result<Flash<Redirect>, Template> {
    if Task::delete_with_id(id, &conn) {
        Ok(Flash::success(Redirect::to("/"), "Task was deleted."))
    } else {
        Err(Template::render(
            "index",
            &Context::err(&conn, "Couldn't delete task."),
        ))
    }
}

#[get("/")]
fn index(msg: Option<FlashMessage<'_, '_>>, conn: DbConn) -> Template {
    Template::render(
        "index",
        &match msg {
            Some(ref msg) => Context::raw(&conn, Some((msg.name(), msg.msg()))),
            None => Context::raw(&conn, None),
        },
    )
}

fn run_db_migrations(rocket: Rocket) -> Result<Rocket, Rocket> {
    let conn = DbConn::get_one(&rocket).expect("database connection");
    match embedded_migrations::run(&*conn) {
        Ok(()) => Ok(rocket),
        Err(e) => {
            error!("Failed to run database migrations: {:?}", e);
            Err(rocket)
        }
    }
}

fn rocket() -> Rocket {
    rocket::ignite()
        .attach(DbConn::fairing())
        .attach(AdHoc::on_attach("Database Migrations", run_db_migrations))
        .mount("/", StaticFiles::from("static/"))
        .mount("/", routes![index])
        .mount("/task", routes![new, toggle, delete])
        .attach(Template::fairing())
}

fn main() {
    rocket().launch();
}
