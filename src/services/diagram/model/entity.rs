use uuid::Uuid;

#[allow(dead_code)]
pub struct Business {
    pub id: Uuid,
    pub code: String,
    pub description: Option<String>,
    pub is_active: bool,
    pub create_time: chrono::NaiveDateTime,
    pub create_by: Uuid,
    pub update_time: chrono::NaiveDateTime,
    pub update_by: Uuid,
}

#[allow(dead_code)]
pub struct Flow {
    pub id: Uuid,
    pub business: String,
    pub state: String,
    pub is_initial_state: bool,
    pub next_states: Option<Vec<String>>,
    pub create_time: chrono::NaiveDateTime,
    pub create_by: Uuid,
    pub update_time: chrono::NaiveDateTime,
    pub update_by: Uuid,
}
