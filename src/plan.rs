use diesel::{self, prelude::*};

mod schema {
    table! {
        plans {
            id -> Integer,
            description -> Text,
            completed -> Bool,
        }
    }
}

use self::schema::plans;
use self::schema::plans::dsl::{completed as plan_completed, plans as all_plans};

#[derive(Serialize, Queryable, Debug, Clone)]
#[belongs_to(TaskWithId)]
#[table_name = "plans"]
pub struct PlanWithId {
    pub id: i32,
    pub description: Nullable<String>,
    pub completed: bool,
}

#[derive(Serialize, Insertable, Debug, Clone)]
#[table_name = "plans"]
pub struct Plan {
    pub description: Nullable<String>,
    pub completed: bool,
}
