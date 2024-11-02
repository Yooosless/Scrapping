use std::{error::Error, thread, time::Duration};

use thirtyfour::{error::WebDriverError, DesiredCapabilities, WebDriver};
use url::Url;

use crate::airbnb::search_with_location;

pub async fn scrape(place: &str)-> Result<(), Box<dyn Error>>{

    let driver= initialize_web_driver().await.unwrap();
    thread::sleep(Duration::from_secs(2));
    let url = Url::parse("https://www.airbnb.co.in").unwrap();

    driver.goto(url).await.unwrap();
    thread::sleep(Duration::from_secs(2));
    search_with_location(&driver,place).await?;

    Ok(())
}

//set the path to chromedriver
//export PATH=$PATH:/home/afridi/scrapping
use std::process::{Child, Command};
use std::net::TcpListener;
use std::thread::sleep;

fn find_free_port() -> u16 {
    TcpListener::bind("127.0.0.1:0").unwrap().local_addr().unwrap().port()
}

fn start_chromedriver(port: u16) -> Child {
    Command::new("chromedriver")
        .arg(format!("--port={}", port))
        .spawn()
        .expect("Failed to start chromedriver")
}

async fn initialize_web_driver() -> Result<WebDriver, WebDriverError> {
    
    let port = find_free_port();
    let _chromedriver = start_chromedriver(port);

   
    sleep(Duration::from_secs(2));

    let url = format!("http://localhost:{}", port);
    let  options = DesiredCapabilities::chrome();


    let driver = WebDriver::new(&url, options).await?;

    driver.maximize_window().await?;
    Ok(driver)
}


