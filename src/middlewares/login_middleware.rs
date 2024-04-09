use rocket::{
    fairing::{Fairing, Info, Kind},
    http::Cookie,
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
        let _auth_cookie = request.cookies().get("auth");
        println!("on_request: {:?}", request.headers());
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Cookie::new("auth", "test cookie"));
        println!("on_response: {:?}", response.headers());
    }
}
