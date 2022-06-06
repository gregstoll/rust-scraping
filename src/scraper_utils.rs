pub fn get_element_text(cell: &scraper::ElementRef) -> String {
    // The DOM allows multiple text nodes of an element, so join them all together.
    cell.text().collect::<Vec<_>>().join("").trim().to_string()
}