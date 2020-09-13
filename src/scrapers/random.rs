use crate::apis;
use rand::{thread_rng, Rng};

pub fn scrape() {
    let olive_api = apis::olive::OliveApi::new("Oklahoman");

    for year in 1901..2021 {
        let mut rng = thread_rng();

        let mut has_pdf_id = false;

        loop {
            let month = rng.gen_range(1, 13);
            let day   = rng.gen_range(1, 31);


            let date = apis::olive::NewspaperDate {
                year: year.to_string(),
                month: format!("{:02}", month),
                day: format!("{:02}", day),
                short: Some(String::from("dok"))
            };

           match olive_api.get_pdf_id_from_date(&date) {
                Ok(pdf_id) => {
                    match olive_api.download_pdf(
                        pdf_id,
                        format!(
                            "./newspapers/{}/{}/{}",
                            date.year,
                            date.month,
                            format!("DOK_{}_{:02}_{:02}.pdf", year, month, day)
                        )
                    ) {
                        Ok(_) => {
                            println!("Downloaded PDF for {}", date.to_string());
                            has_pdf_id = true;
                        },
                        Err(err) => {
                            println!("Error downloading the PDF for {}", date.to_string());
                            println!("\t{}", err);
                        }
                    };
                },
                Err(_) => {
                    println!("Error getting the PDF ID for {}", date.to_string());
                }
            };
                
            if has_pdf_id { break; }
        }
    }
}

