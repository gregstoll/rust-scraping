use reqwest::Error;
use scraper::{Html, Selector};
use lazy_static::lazy_static;

// Convenience function to avoid unwrap()ing all the time
fn make_selector(selector: &str) -> Selector {
    Selector::parse(selector).unwrap()
}

lazy_static! {
    static ref TABLE: Selector = make_selector("table");
    static ref TR: Selector = make_selector("tr");
    static ref TD: Selector = make_selector("td");
}

// Holds the column indices where the values for number of people
// still alive are held.
struct ColumnIndices {
    row_number: u32,
    male: u32,
    female: u32
}

fn parse_page(year: u32) -> Result<(), Error> {
    println!("Parsing year {}", year);
    let url = format!("https://www.ssa.gov/oact/NOTES/as120/LifeTables_Tbl_7_{}.html", year);
    let body = reqwest::blocking::get(url)?
        .text()?;

    let document = Html::parse_document(&body);
    // Find the table with the most rows
    let main_table = document.select(&TABLE).max_by_key(|table| {
        table.select(&TR).count()
    }).expect("No tables found in document?");

    // Find the columns we want
    let column_indices: Option<ColumnIndices> = None;
    for row in main_table.select(&TR) {
        let entries = row.select(&TD);
        //TODO
        /*if entries.clone().count() < 4 {
            // row is too short; skip it
            continue;
        }*/
        if column_indices.is_none() {
            let mut row_number_index: Option<u32> = None;
            let mut male_index: Option<u32> = None;
            let mut female_index: Option<u32> = None;
            // look for values of "0" (for the row number) and "100000"
            for (column_index, cell) in entries.into_iter().enumerate() {
                // The DOM allows multiple text nodes of an element, so join them all together.
                let text: String = cell.text().collect::<Vec<_>>().join("");
                let text: &str = text.trim();
                if text == "0" {
                    // Only want the first column that has a value of "0"
                    row_number_index = row_number_index.or(Some(column_index as u32));
                } else if text == "100000" {
                    // male columns are first
                    if male_index.is_none() {
                        male_index = Some(column_index as u32);
                    }
                    else if female_index.is_none() {
                        female_index = Some(column_index as u32);
                    }
                    else {
                        panic!("Found too many columns with text \"100000\"!");
                    }
                }
            }
        }
    }
    Ok(())
}

fn main() -> Result<(), Error> {
    //TODO
    //for year in (1900..=2100).step_by(10) {
    for year in (1900..=1901).step_by(10) {
        parse_page(year)?;
    }
    return Ok(());
}
