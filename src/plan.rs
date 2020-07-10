use chrono::NaiveDate;
use diesel::{self, prelude::*};

mod schema {
    table! {
        plans (id) {
            id -> Integer,
            task_id -> Integer,
            description -> Nullable<Text>,
            completed -> Bool,
        }
    }
}

use self::schema::plans;
use self::schema::plans::dsl::{completed as plan_completed, plans as all_plans};
use crate::task::Task;

#[derive(Identifiable, Queryable, AsChangeset, Associations, Serialize, Deserialize)]
#[belongs_to(Task, foreign_key = "task_id")]
pub struct Plan {
    pub id: i32,
    pub task_id: i32,
    pub description: Option<String>,
    pub completed: bool,
}

#[derive(Serialize, Insertable, Debug, Clone)]
#[table_name = "plans"]
pub struct InsertablePlan {
    pub task_id: i32,
    pub description: Option<String>,
    pub completed: bool,
}

#[derive(FromForm)]
pub struct PlanFormInput {
    pub task_id: i32,
    pub description: Option<String>,
}

impl Plan {
    pub fn all(conn: &SqliteConnection) -> Vec<Plan> {
        all_plans
            .order(plans::id.desc())
            .load::<Plan>(conn)
            .unwrap()
    }

    pub fn all_with_tasks(conn: &SqliteConnection) -> Vec<(Task, Vec<Plan>)> {
        let tasks = Task::all(conn);
        let plans = Plan::belonging_to(&tasks)
            .load(conn)
            .expect("Error loading tasks")
            .grouped_by(&tasks);
        tasks.into_iter().zip(plans).collect::<Vec<_>>()
    }

    pub fn of_task(conn: &SqliteConnection, task: &Task) -> Vec<Plan> {
        Plan::belonging_to(task)
            .load(conn)
            .expect("Error loading tasks")
    }

    pub fn toggle_with_id(id: i32, conn: &SqliteConnection) -> bool {
        let plan = all_plans.find(id).get_result::<Plan>(conn);
        if plan.is_err() {
            return false;
        }

        let new_status = !plan.unwrap().completed;
        let updated_plan = diesel::update(all_plans.find(id));
        updated_plan
            .set(plan_completed.eq(new_status))
            .execute(conn)
            .is_ok()
    }

    pub fn delete_with_id(id: i32, conn: &SqliteConnection) -> bool {
        diesel::delete(all_plans.find(id)).execute(conn).is_ok()
    }

    pub fn all_on_date(conn: &SqliteConnection, date: NaiveDate) -> Vec<Plan> {
        all_plans
            .filter(self::schema::plans::dsl::date.eq(date))
            .load::<Plan>(&conn)
            .expect("Error loading plans")
    }
}

impl InsertablePlan {
    pub fn insert(plan_form_input: PlanFormInput, conn: &SqliteConnection) -> bool {
        let description = plan_form_input.description.filter(|s| !s.is_empty());
        let plan = InsertablePlan {
            task_id: plan_form_input.task_id,
            description,
            completed: false,
        };
        diesel::insert_into(plans::table)
            .values(&plan)
            .execute(conn)
            .is_ok()
    }
}
