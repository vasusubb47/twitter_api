use rocket::{
    fairing::{Fairing, Info, Kind},
    Data, Request,
};

pub struct Logging {}

#[allow(dead_code)]
#[derive(Debug)]
struct RequestLog {
    path: String,
    http_method: String,
    ip_address: String,
}

impl RequestLog {
    fn new(req: &Request) -> Self {
        RequestLog {
            path: req.uri().path().to_string(),
            http_method: req.method().to_string(),
            ip_address: req.client_ip().unwrap().to_string(),
        }
    }
}

#[rocket::async_trait]
impl Fairing for Logging {
    fn info(&self) -> Info {
        Info {
            name: "Logging",
            kind: Kind::Request,
        }
    }

    async fn on_request(&self, request: &mut Request<'_>, _: &mut Data<'_>) {
        let req_log = RequestLog::new(request);
        println!("{:?}", req_log);
    }
}
