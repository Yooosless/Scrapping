use csv::Writer;
use thirtyfour::{error::WebDriverError, prelude::ElementWaitable, By, WebDriver, WebElement};

use std::{error::Error, fs::File, thread, time::Duration};

use crate::models::Airbnb;

pub async  fn search_with_location(driver: &WebDriver, place: &str)-> Result<(), WebDriverError>{
    click_search(driver).await?;
    enter_text(driver, place).await?;
    click_button(driver).await?;
    get_all_cards(driver).await.map_err(|e| {
        // Convert Box<dyn Error> to WebDriverError
        WebDriverError::CustomError(e.to_string())
    })?;
    Ok(())
}

async fn click_search(driver: &WebDriver)-> Result<(), WebDriverError>{
    driver.find(By::XPath(r#"//*[@placeholder][contains(@placeholder, 'Search')]"#)).await?.click().await?;
    Ok(())
}

async  fn enter_text(driver: &WebDriver, place: &str)-> Result<(), WebDriverError>{

    let input=     driver.find(By::XPath(r#"//*[@placeholder][contains(@placeholder, 'Search')]"#)).await?;
    input.wait_until().clickable().await?;

    input.send_keys(place).await?;
    Ok(())
}

async  fn click_button(driver: &WebDriver)-> Result<(),WebDriverError>{
    driver.find(By::XPath(r#"//*[contains(@data-testid, 'search-button')]"#)).await?.click().await?;

    Ok(())
}

async fn get_all_cards(driver: &WebDriver) -> Result<(), Box<dyn Error>> {
    // Initial scroll to load content
    thread::sleep(Duration::from_secs(4));
    driver.execute("window.scrollTo(0, document.body.scrollHeight);", vec![]).await?;
    thread::sleep(Duration::from_secs(2));

    let mut wtr = Writer::from_path("airbnb.csv")?;
    
    loop {
        // Try to find the "Next" button
        match driver.find(By::Css("#site-content > div > div.pbmlr01.atm_h3_t9kd1m.atm_gq_n9wab5.dir.dir-ltr > div > div > div > nav > div > a.l1ovpqvx.atm_1he2i46_1k8pnbi_10saat9.atm_yxpdqi_1pv6nv4_10saat9.atm_1a0hdzc_w1h1e8_10saat9.atm_2bu6ew_929bqk_10saat9.atm_12oyo1u_73u7pn_10saat9.atm_fiaz40_1etamxe_10saat9.c1ytbx3a.atm_mk_h2mmj6.atm_9s_1txwivl.atm_h_1h6ojuz.atm_fc_1h6ojuz.atm_bb_idpfg4.atm_26_1j28jx2.atm_3f_glywfm.atm_7l_hkljqm.atm_gi_idpfg4.atm_l8_idpfg4.atm_uc_10d7vwn.atm_kd_glywfm.atm_gz_8tjzot.atm_uc_glywfm__1rrf6b5.atm_26_zbnr2t_1rqz0hn_uv4tnr.atm_tr_kv3y6q_csw3t1.atm_26_zbnr2t_1ul2smo.atm_3f_glywfm_jo46a5.atm_l8_idpfg4_jo46a5.atm_gi_idpfg4_jo46a5.atm_3f_glywfm_1icshfk.atm_kd_glywfm_19774hq.atm_70_glywfm_1w3cfyq.atm_uc_aaiy6o_9xuho3.atm_70_18bflhl_9xuho3.atm_26_zbnr2t_9xuho3.atm_uc_glywfm_9xuho3_1rrf6b5.atm_70_glywfm_pfnrn2_1oszvuo.atm_uc_aaiy6o_1buez3b_1oszvuo.atm_70_18bflhl_1buez3b_1oszvuo.atm_26_zbnr2t_1buez3b_1oszvuo.atm_uc_glywfm_1buez3b_1o31aam.atm_7l_1wxwdr3_1o5j5ji.atm_9j_13gfvf7_1o5j5ji.atm_26_1j28jx2_154oz7f.atm_92_1yyfdc7_vmtskl.atm_9s_1ulexfb_vmtskl.atm_mk_stnw88_vmtskl.atm_tk_1ssbidh_vmtskl.atm_fq_1ssbidh_vmtskl.atm_tr_pryxvc_vmtskl.atm_vy_1vi7ecw_vmtskl.atm_e2_1vi7ecw_vmtskl.atm_5j_1ssbidh_vmtskl.atm_mk_h2mmj6_1ko0jae.dir.dir-ltr")).await {
            Ok(next_page_button) => {
                // Check if the next page button is clickable
                if next_page_button.is_clickable().await? {
                    thread::sleep(Duration::from_secs(2));

                    // Extract data from current page
                    extract_data(driver, &mut wtr).await?;

                    // Load the next page
                    load_next_page(next_page_button, driver).await?;
                } else {
                    // Break if the button is not clickable
                    break;
                }
            }
            Err(_) => {
                // If no next button is found, extract data from current page and exit
                extract_data(driver, &mut wtr).await?;
                break;
            }
        }
    }
    Ok(())
}

async fn extract_data(driver: &WebDriver, wtr: &mut Writer<File>) -> Result<(), Box<dyn Error>> {
    let cards = get_house_elements(driver).await?;

    for house_elem in cards {
        let airbnb = Airbnb::from(&house_elem).await?;
        wtr.serialize(airbnb)?;
    }
    Ok(())
}

async fn load_next_page(next_page_button: WebElement, driver: &WebDriver) -> Result<(), Box<dyn Error>> {
    next_page_button.click().await?;
    thread::sleep(Duration::from_secs(2));

    driver.execute("window.scrollTo(0, document.body.scrollHeight);", vec![]).await?;
    thread::sleep(Duration::from_secs(1));

    Ok(())
}


async fn get_house_elements(driver: &WebDriver) -> Result<Vec<WebElement>, WebDriverError> {
    driver.find_all(By::Css("#site-content > div > div:nth-child(2) > div > div > div > div > div")).await
}

