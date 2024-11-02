use thirtyfour::{error::WebDriverError, WebDriver};

async fn scroll_page(driver: &WebDriver) -> Result<(), WebDriverError> {
    // Execute JavaScript to scroll to the bottom of the page
    driver
        .execute(r#"window.scrollTo(0, document.body.scrollHeight);"#, vec![])
        .await?;
    Ok(())
}
// impl From<CsvError> for WebDriverError {
//     fn from(error: CsvError) -> Self {
//         WebDriverError::UnknownError(format!("CSV error: {}", error))
//     }
// }