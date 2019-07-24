use diesel::{self, prelude::*};

mod schema {
    table! {
        plans {
            id -> Integer,
            description -> Nullable<Text>,
            completed -> Bool,
        }
    }
}

use self::schema::plans;
use self::schema::plans::dsl::plans as all_plans;

// #[belongs_to(TaskWithId)]
// #[table_name = "plans"]
#[derive(Serialize, Queryable, Debug, Clone)]
pub struct PlanWithId {
    pub id: i32,
    pub description: Option<String>,
    pub completed: bool,
}

#[derive(Serialize, Insertable, Debug, Clone)]
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
