use diesel::{self, prelude::*};

mod schema {
    table! {
        tasks {
            id -> Integer,
            description -> Text,
            completed -> Bool,
        }
    }
}

use self::schema::tasks;
use self::schema::tasks::dsl::{completed as task_completed, tasks as all_tasks};

// This gives me the error "cannot find attribute macro `table_name` in this
// scope". I believe this is a compiler error (or at least an inaccurate
// compiler error message). However, it compiles fine without it, which actually
// confuses me even more.
// #[table_name = "tasks"]
#[derive(Serialize, Identifiable, Queryable, Debug, Clone)]
#[table_name = "tasks"]
pub struct TaskWithId {
    pub id: i32,
    pub description: String,
    pub completed: bool,
    // pub task: Task,
}

#[derive(Serialize, Insertable, Debug, Clone)]
#[table_name = "tasks"]
pub struct Task {
    pub description: String,
    pub completed: bool,
}

#[derive(FromForm)]
pub struct TaskFormInput {
    pub description: String,
}

impl Task {
    pub fn insert(task_form_input: TaskFormInput, conn: &SqliteConnection) -> bool {
        let t = Task {
            description: task_form_input.description,
            completed: false,
        };
        diesel::insert_into(tasks::table)
            .values(&t)
            .execute(conn)
            .is_ok()
    }
}

impl TaskWithId {
    pub fn all(conn: &SqliteConnection) -> Vec<TaskWithId> {
        all_tasks
            .order(tasks::id.desc())
            .load::<TaskWithId>(conn)
            .unwrap()
    }

    pub fn toggle_with_id(id: i32, conn: &SqliteConnection) -> bool {
        let task = all_tasks.find(id).get_result::<TaskWithId>(conn);
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
