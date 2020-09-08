use actix_web::Responder;

pub async fn get_users() -> impl Responder {
    format!("Form para get users")
}

pub async fn get_user_by_id() -> impl Responder {
    format!("Form para get users by id")
}

pub async fn add_user() -> impl Responder {
    format!("Form para add user")
}

pub async fn delete_user() -> impl Responder {
    format!("Form para delete user")
}