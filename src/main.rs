use std::collections::HashMap;

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

#[derive(Clone, Debug)]
struct SurvivorsAtAgeTable {
    pub male: Vec<f32>,
    pub female: Vec<f32>
}

// Holds the column indices where the values for number of people
// still alive are held.
#[derive(Clone, Copy, Debug)]
struct ColumnIndices {
    pub row_number: usize,
    pub male: usize,
    pub female: usize
}

impl ColumnIndices {
    pub fn max_index(&self) -> usize {
        *([self.row_number, self.male, self.female].iter().max().unwrap())
    }
}

fn get_text(cell: &scraper::ElementRef) -> String {
    // The DOM allows multiple text nodes of an element, so join them all together.
    cell.text().collect::<Vec<_>>().join("").trim().replace(",", "")
}

fn parse_page(year: u32) -> Result<SurvivorsAtAgeTable, Error> {
    println!("Parsing year {}", year);
    let url = format!("https://www.ssa.gov/oact/NOTES/as120/LifeTables_Tbl_7_{}.html", year);
    let body = reqwest::blocking::get(url)?
        .text()?;

    let document = Html::parse_document(&body);
    // Find the table with the most rows
    let main_table = document.select(&TABLE).max_by_key(|table| {
        table.select(&TR).count()
    }).expect("No tables found in document?");

    // TODO - better name
    let mut male_values = Vec::<f32>::new();
    let mut female_values = Vec::<f32>::new();
    // Find the columns we want
    let mut column_indices: Option<ColumnIndices> = None;
    let mut next_row_number: u32 = 0;
    for row in main_table.select(&TR) {
        // Need to collect this into a Vec<> because we're going to be iterating over it
        // multiple times.
        let entries = row.select(&TD).collect::<Vec<_>>();
        if column_indices.is_none() {
            let mut row_number_index: Option<usize> = None;
            let mut male_index: Option<usize> = None;
            let mut female_index: Option<usize> = None;
            // look for values of "0" (for the row number) and "100000"
            for (column_index, cell) in entries.iter().enumerate() {
                // The DOM allows multiple text nodes of an element, so join them all together.
                let text: String = get_text(cell);
                if text == "0" {
                    // Only want the first column that has a value of "0"
                    row_number_index = row_number_index.or(Some(column_index));
                } else if text == "100000" {
                    // male columns are first
                    if male_index.is_none() {
                        male_index = Some(column_index);
                    }
                    else if female_index.is_none() {
                        female_index = Some(column_index);
                    }
                    else {
                        panic!("Found too many columns with text \"100000\"!");
                    }
                }
            }
            assert_eq!(male_index.is_some(), female_index.is_some(), "Found male column but not female?");
            if male_index.is_some() {
                assert!(row_number_index.is_some(), "Found male column but not row number?");
                column_indices = Some(ColumnIndices {
                    row_number: row_number_index.unwrap(),
                    male: male_index.unwrap(),
                    female: female_index.unwrap()
                });
            }
        }
        if let Some(column_indices) = column_indices {
            if entries.len() < column_indices.max_index() {
                // Too few columns, this isn't a real row
                continue
            }
            let row_number_text = get_text(&entries[column_indices.row_number]);
            if row_number_text.parse::<u32>().and_then(|x| Ok(x == next_row_number)) == Ok(true) {
                next_row_number += 1;
                let male_value = get_text(&entries[column_indices.male]).parse::<u32>();
                let male_value = male_value.expect("Couldn't parse value in male cell");
                // TODO document this
                let male_value = male_value as f32 / 100000_f32;
                assert!(male_value <= 1.0, "male value is out of range");
                if male_values.len() > 0 {
                    assert!(*male_values.last().unwrap() >= male_value, "male values are not decreasing");
                }
                male_values.push(male_value);

                let female_value = get_text(&entries[column_indices.female]).parse::<u32>();
                let female_value = female_value.expect("Couldn't parse value in female cell");
                let female_value = female_value as f32 / 100000_f32;
                assert!(female_value <= 1.0, "female value is out of range");
                if female_values.len() > 0 {
                    assert!(*female_values.last().unwrap() >= female_value, "female values are not decreasing");
                }
                female_values.push(female_value);
            }
        }
    }
    assert_eq!(male_values.len(), female_values.len());
    assert!(male_values.len() > 50);

    Ok(SurvivorsAtAgeTable {
        male: male_values,
        female: female_values
    })
}

fn main() -> Result<(), Error> {
    let mut all_data: HashMap<u32, SurvivorsAtAgeTable> = HashMap::new();
    for year in (1900..=2100).step_by(10) {
        all_data.insert(year, parse_page(year)?);
    }
    return Ok(());
}
