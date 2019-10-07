use diesel::{self, prelude::*};

mod schema {
    table! {
        tasks (id) {
            id -> Integer,
            description -> Text,
            completed -> Bool,
            due_date -> Nullable<Date>,
        }
    }
}

use self::schema::tasks;
use self::schema::tasks::dsl::{completed as task_completed, tasks as all_tasks};
use chrono::NaiveDate;
use crate::util::NaiveDate as MyNaiveDate;

#[derive(Identifiable, Queryable, AsChangeset, Serialize, Deserialize)]
pub struct Task {
    pub id: i32,
    pub description: String,
    pub completed: bool,
    pub due_date: Option<NaiveDate>,
}

#[derive(Insertable)]
#[table_name = "tasks"]
pub struct InsertableTask {
    pub description: String,
    pub completed: bool,
    pub due_date: Option<NaiveDate>,
}

#[derive(FromForm)]
pub struct TaskFormInput {
    pub description: String,
    pub due_date: Option<MyNaiveDate>,
}

impl Task {
    pub fn all(conn: &SqliteConnection) -> Vec<Task> {
        all_tasks
            .order(tasks::id.desc())
            .load::<Task>(conn)
            .unwrap_or_else(|err| {
                eprintln!("Error: {:?}", err);
                Vec::new()
            })
    }

    pub fn toggle_with_id(id: i32, conn: &SqliteConnection) -> bool {
        let task = all_tasks.find(id).get_result::<Task>(conn);
        if task.is_err() {
            return false;
        }

        let new_status = !task.unwrap().completed;
        let updated_task = diesel::update(all_tasks.find(id));
        updated_task
            .set(task_completed.eq(new_status))
            .execute(conn)
            .is_ok()
    }

    pub fn delete_with_id(id: i32, conn: &SqliteConnection) -> bool {
        diesel::delete(all_tasks.find(id)).execute(conn).is_ok()
    }

    #[cfg(test)]
    pub fn delete_all(conn: &SqliteConnection) -> bool {
        diesel::delete(all_tasks).execute(conn).is_ok()
    }
}

impl InsertableTask {
    pub fn insert(task_form_input: TaskFormInput, conn: &SqliteConnection) -> bool {
        let task = InsertableTask {
            description: task_form_input.description,
            completed: false,
            due_date: task_form_input.due_date.map(|d| d.0),
        };
        diesel::insert_into(tasks::table)
            .values(&task)
            .execute(conn)
            .is_ok()
    }
}
