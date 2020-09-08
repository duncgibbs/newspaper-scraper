use rand::{thread_rng, Rng};
mod apis;

fn sequential() {
    let olive_api = apis::olive::OliveApi::new("Oklahoman");

    for year in 1901..2020 {
        for month in 1..13 {
            for day in 1..32 {
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
            }
        }
    }
}

// TODO
fn random() {
    let olive_api = apis::olive::OliveApi::new("Oklahoman");

    for year in 1901..2020 {
        for month in 1..13 {
            for day in 1..32 {
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
            }
        }
    }
}

fn main() {
    sequential();
}
