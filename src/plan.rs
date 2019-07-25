use diesel;
use diesel::prelude::*;

mod schema {
    table! {
        plans {
            id -> Integer,
            task_id -> Integer,
            description -> Nullable<Text>,
            completed -> Bool,
        }
    }
}

use self::schema::plans;
use self::schema::plans::dsl::plans as all_plans;
use crate::task::TaskWithId;

#[derive(Serialize, Identifiable, Queryable, Associations, Debug, Clone)]
#[belongs_to(TaskWithId, foreign_key = "task_id")]
#[table_name = "plans"]
pub struct PlanWithId {
    pub id: i32,
    pub task_id: i32,
    pub description: Option<String>,
    pub completed: bool,
}

#[derive(Serialize, Insertable, Associations, Debug, Clone)]
#[table_name = "plans"]
pub struct Plan {
    pub description: Option<String>,
    pub completed: bool,
}

impl PlanWithId {
    pub fn all(conn: &SqliteConnection) -> Vec<PlanWithId> {
        all_plans
            .order(plans::id.desc())
            .load::<PlanWithId>(conn)
            .unwrap()
    }
}
