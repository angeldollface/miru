use tokio;
use miru::App;
use miru::Request;
use miru::Response;
use serde::Serialize;
use serde::Deserialize;
use serde_json::from_str;

#[derive(Serialize, Deserialize)]
pub struct Echo {
    pub msg: String
}

pub async fn echo_payload(request: Request ) -> Result<Response<Echo>,MiruErr> {
    let msg: String = match request.body {
        Some(body) => body.body,
        None => {
            let e: String = "No body in response.".to_string();
            return Err::<(), MiruErr>(MiruErr::new(&e.to_string()))
        }
    };
    let echo: Echo = match from_str(&msg){
        Ok(echo) => echo,
        Err(e) => return Err::<Response<Echo>, MiruErr>(MiruErr::new(&e.to_string()))
    };
    Ok(Response::new(&echo).as_json())
}

#[tokio::main]
async fn main(){
    let mut my_app: App = App::new("127.0.0.1", "8080");
    my_app.add_service("/", echo_payload);
    my_app.run().await;
}