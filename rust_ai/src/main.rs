use ollama_rs::generation::completion::request::GenerationRequest;
use ollama_rs::Ollama;
use std::io;
use std::io::Write;

#[tokio::main]
async fn main() {
        println!("ATTENTION: CNTRL-C to exit => (5trni bo5li)");
    loop {
        
        print!("GIMME YOUR COMMAND: \n>>>");
        io::stdout().flush().unwrap();
        let mut buffer = String::new();
        let _ = io::stdin().read_line(&mut buffer);
        let ollama = Ollama::default();
        let model = "llama3.1".to_string();
        let prompt = buffer.to_string();

        let res = ollama.generate(GenerationRequest::new(model, prompt)).await;

        if let Ok(res) = res {
        println!("{}", res.response);
        }
    }
}
