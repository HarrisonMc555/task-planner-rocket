use diesel::{self, prelude::*};

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
    pub description: Option<String>,
    pub completed: bool,
}

impl Plan {
    pub fn all(conn: &SqliteConnection) -> Vec<Plan> {
        all_plans
            .order(plans::id.desc())
            .load::<Plan>(conn)
            .unwrap()
    }
}
