use rocket::{
    fairing::{Fairing, Info, Kind},
    Data, Request, Response,
};

pub struct LoginCheck {}

#[rocket::async_trait]
impl Fairing for LoginCheck {
    fn info(&self) -> Info {
        Info {
            name: "Login Check",
            kind: Kind::Request | Kind::Response,
        }
    }

    async fn on_request(&self, request: &mut Request<'_>, _: &mut Data<'_>) {
        println!("on_request: {:?}", request.headers())
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        println!("on_response: {:?}", response.headers())
    }
}
