extern crate lm_studio_api_extended;  use lm_studio_api_extended::{ Model, Context, Chat, Request };

#[tokio::main]
async fn main() {
    // init chat:
    let mut chat = Chat::new(
        Model::Gemma3_4b,   // select AI model
        Context::new("You're Jarvis - my personal assistant. Call me master", 4090),  // write system prompt + set max size of messages context
    );
    

    // NO STREAM TEST:
    
    // init request:
    let request = Request {
        messages: vec!["Hi, what's your name?".into()],
        context: true,
        stream: false,
        ..Request::default()
    };

    // sending request:
    let result = chat.send(request).await;

    match result {
        // print results part:
        Ok(Some(response)) =>  println!("{}", response.text()),

        // print error:
        Err(e) => eprintln!("Error: {e}"),

        _ => {}
    }


    // STREAM TEST:

    loop {
        // reading user input:
        eprint!("\n>> ");
        
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();

        // generating answer:
        eprint!("<< ");
        
        // init request:
        let request = Request {
            messages: vec![buf.into()],
            context: true,
            stream: true,
            ..Request::default()
        };
        
        // sending request:
        let _ = chat.send(request).await.unwrap();

        // reading AI results:
        while let Some(result) = chat.next().await {
            match result {
                // print results part:
                Ok(text) if !text.is_empty() => {
                    eprint!("{text}");
                },

                // print error:
                Err(e) => {
                    eprintln!("Error: {e}");
                    break;
                },

                _ => {}
            }
        }
    }
}
