use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct ActivityAddWorkTime {
    pub user: i64,
    pub task: i64,
    pub new_date_from: DateTime<Local>,
    pub new_date_to: DateTime<Local>,
    only_delete: bool,
    need_merged_data: bool,
}

impl ActivityAddWorkTime {
    pub fn for_adding(
        user_id: i64,
        task_id: i64,
        from: DateTime<Local>,
        to: DateTime<Local>,
    ) -> Self {
        Self {
            user: user_id,
            task: task_id,
            new_date_from: from,
            new_date_to: to,
            only_delete: false,
            // need_merged_data flag assumingly tells whether we want to get
            // resulting task data in response
            need_merged_data: true,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct ActivityTaskDataSource {
    id: i64,
    team: i64,
    name: String,
    slug: String,
    comment: String,
}

#[derive(Deserialize, Debug)]
pub struct ActivityTask {
    id: i64,
    name: String,
    slug: String,
    date_start: DateTime<Local>,
    date_end: DateTime<Local>,
    summary_time: f64,
    datasource: ActivityTaskDataSource,
}

#[derive(Deserialize, Debug)]
pub struct ActivityWorkTimeEntry {
    datetime_start: DateTime<Local>,
    datetime_end: DateTime<Local>,
    source: String,
    tasks: Vec<ActivityTask>,
}

pub type ActivityWorkTime = Vec<ActivityWorkTimeEntry>;
