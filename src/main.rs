use std::error::Error;
use utils::driver::scrape;



#[tokio::main]
async fn main()-> Result<(), Box<dyn Error>> {

    // let client = Client::new();
    // // let body= reqwest::get("https://z-library.do/").await;
    // // print!("{:?}",body);



    scrape("bangalore").await?;

    Ok(())
}
