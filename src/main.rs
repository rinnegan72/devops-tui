use devops_tui::tui::main_renderer;

#[tokio::main]
async fn main() {
    match main_renderer().await {
        Ok(result) => {
            println!("all good!");
        }
        Err(e) => {
            let error_message = format!("Error {e} occured unable to render TUI");
            println!("{error_message}");
        }
    }
}
