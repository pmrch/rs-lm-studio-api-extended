use lm_studio_api_extended::{Chat, Model, Context, Request};

#[tokio::main]
async fn main() {
    let mut chat = Chat::new(
        Model::Kimiko13b,
        Context::new(
            "You're Jarvis – my assistant.", 
            4090
        ),
    );

    let request = Request {
        messages: vec!["Hello, Jarvis.".into()],
        context: true,
        stream: false,
        ..Default::default()
    };

    match chat.send(request).await {
        Ok(Some(res)) => println!("{}", res.text()),
        Err(e) => eprintln!("Error: {e}"),
        _ => {}
    }
}
