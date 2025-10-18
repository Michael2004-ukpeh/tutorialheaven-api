use serde::Deserialize;

#[derive{Deserialze, Debug}]
pub struct SignupDto{
    pub username:String,
    pub full_name:Option<String>, 
    pub email:String, 
    pub password:String,

}