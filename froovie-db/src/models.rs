#[derive(Queryable)]
pub struct User {
    pub id: i32, 
    pub nick: String,
}

impl ToString for User {
    fn to_string(&self) -> String {
        format!("id : {} , nick : {} ", self.id, self.nick) 
    }
}