use miru::App;
use miru::Request;
use miru::Response;

pub struct Echo {
    pub msg: String
}

impl Echo {
    pub fn new(msg: &String) -> Echo {
        Echo { msg: msg.to_owned() }
    }
}

pub async fn echo_payload(request: midrar::Request ) -> Response<Echo> {
    
}

async fn main(){
    let mut my_app: App = App::new("127.0.0.1", "8080");
    my_app.add_service("/", echo_payload);
    my_app.run().await;
}