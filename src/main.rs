use token_trackoooooooor::run;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    run().await;
}
