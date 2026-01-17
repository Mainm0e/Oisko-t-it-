use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct VisitResponse {
    pub is_first_visit: bool,
    pub is_first_of_day: bool,
    pub total_unique_visitors: i64,
    pub today_visitors: i64,
}
