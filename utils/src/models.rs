use serde::Serialize;
use thirtyfour::{error::WebDriverError, By, WebElement};

#[derive(Debug, Serialize)]
pub struct Airbnb{
    pub image: String,
    pub location: String,
    pub price: String,
}


impl Airbnb{
    pub async fn from(element: &WebElement)-> Result<Self, WebDriverError>{

        let image= Airbnb::get_image_link(&element).await?;
        let location= Airbnb::get_location(&element).await?;
        let price =Airbnb::get_price(&element).await?;
        Ok(Self{
            image,
            location,
            price
        })
    }
//#site-content > div > div:nth-child(2) > div > div > div > div > div > div:nth-child(1) > div > div.c1l1h97y.atm_d2_1kqhmmj.dir.dir-ltr > div > div > div > div > div > div.m1v28t5c.atm_gq_1gibeiw.atm_72g4pb_llf7ib.atm_1mz0ff6_1lk22r2.atm_1p5rjfj_h2mmj6.dir.dir-ltr > div > div > div.s1yvqyx7.atm_d2_1mxew8z.atm_gp_1rldl0o.atm_go_1kc7gxz.atm_gy_1gdueyf.atm_gx_1q7iut.atm_mh_k8b3f6.atm_l8_55ikqx.dir.dir-ltr > div > div > div > div > a:nth-child(1) > div > div > picture > img
    async fn get_image_link(house_elem: &WebElement) -> Result<String, WebDriverError> {
        let srcset = house_elem
        .find(By::Css("div:nth-child(1) > div > div.c1l1h97y.atm_d2_1kqhmmj.dir.dir-ltr > div > div > div > div > div > div.m1v28t5c.atm_gq_1gibeiw.atm_72g4pb_llf7ib.atm_1mz0ff6_1lk22r2.atm_1p5rjfj_h2mmj6.dir.dir-ltr > div > div > div.s1yvqyx7.atm_d2_1mxew8z.atm_gp_1rldl0o.atm_go_1kc7gxz.atm_gy_1gdueyf.atm_gx_1q7iut.atm_mh_k8b3f6.atm_l8_55ikqx.dir.dir-ltr > div > div > div > div > a:nth-child(1) > div > div > picture > source"))
        .await?
        .attr("srcset")
        .await?;

        Ok(srcset.unwrap_or_default())

    }
    
    async fn get_price(house_elem: &WebElement) -> Result<String, WebDriverError>{
        house_elem
            .find(By::Css("div:nth-child(3) > div > div.c1l1h97y.atm_d2_1kqhmmj.dir.dir-ltr > div > div > div > div > div > div.g1qv1ctd.atm_u80d3j_1li1fea.atm_c8_o7aogt.atm_g3_8jkm7i.c1v0rf5q.atm_9s_11p5wf0.atm_cx_4wguik.atm_dz_7esijk.atm_e0_1lo05zz.dir.dir-ltr > div.pquyp1l.atm_da_cbdd7d.pi11895.atm_h3_lh1qj6.dir.dir-ltr > div:nth-child(2) > div > div > span > div > span._11jcbg2"))
            .await?
            .text()
            .await
    }
    async fn get_location(house_elem: &WebElement) -> Result<String, WebDriverError> {
    
        let host = house_elem
        .find(By::Css("div:nth-child(3) > div > div.c1l1h97y.atm_d2_1kqhmmj.dir.dir-ltr > div > div > div > div > div > div.g1qv1ctd.atm_u80d3j_1li1fea.atm_c8_o7aogt.atm_g3_8jkm7i.c1v0rf5q.atm_9s_11p5wf0.atm_cx_4wguik.atm_dz_7esijk.atm_e0_1lo05zz.dir.dir-ltr > div:nth-child(2)"))
        .await;
    if let Ok(host) = host {
        host.text().await
    } else {
        house_elem
            .find(By::Css("div:nth-child(3) > span"))
            .await?
            .text()
            .await
    }
}
}