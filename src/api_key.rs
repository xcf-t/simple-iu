use rocket::request::{FromRequest, Outcome};
use rocket::Request;
use rocket::http::Status;

#[derive(Debug, Clone)]
pub struct User {
    pub name: String,
    pub descriptor: String,
    pub upload_limit: u64,
}

#[rocket::async_trait]
impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = ();

    async fn from_request(req: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let keys: Vec<_> = req.headers().get("Authorization").collect();

        if keys.len() != 1 {
            return Outcome::Failure((Status::Forbidden, ()));
        }

        let key = keys.get(0).unwrap().to_string();
        let users = crate::USERS.read().await;

        if !users.contains_key(&key) {
            return Outcome::Failure((Status::Forbidden, ()));
        }

        return if let Some(u) = users.get(&key) {
            Outcome::Success(u.clone())
        } else {
            Outcome::Failure((Status::Forbidden, ()))
        }
    }
}