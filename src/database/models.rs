use chrono::naive::NaiveDateTime;


#[derive(Serialize, Deserialize, Queryable)]
pub struct Person {
    pub id: u64,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
