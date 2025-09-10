use lm_studio_api_extended::{Chat, Model, Context, Request};

#[tokio::main]
async fn main() {
    let mut chat = Chat::new(
        Model::Kimiko13b,   // Replace model if you are using a different one
        Context::new("You're Jarvis – my assistant.", 4090),
    );

    loop {
        eprint!("\n>> ");
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();

        let request = Request {
            messages: vec![buf.into()],
            context: true,
            stream: true,
            ..Default::default()
        };

        let _ = chat.send(request).await.unwrap();

        while let Some(result) = chat.next().await {
            match result {
                Ok(text) if !text.is_empty() => eprint!("{text}"),
                Err(e) => {
                    eprintln!("Error: {e}");
                    break;
                }
                _ => {}
            }
        }
    }
}
