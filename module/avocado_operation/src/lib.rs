use holium_rust_sdk::*;

#[holium_bindgen]
pub struct SalesDetails {
    plu_4046: u32,
    plu_4225: u32,
    plu_4770: u32,
    date: String,
    average_price: u32,
    total_volume: u32,
    total_bags: u32,
    small_bags: u32,
    large_bags: u32,
    xlarge_bags: u32,
    avocado_type: String,
    year: u32,
    geography: String,
}

#[holium_bindgen]
/// [`sort_by_sales`] will fetch puls sales based on a year.
pub fn get_sales_by_year(
    sales_details: Vec<SalesDetails>,
    year_to_explore: u32,
) -> Vec<(String, u32)> {
    let mut sales_results: Vec<(String, u32)> = Vec::new();

    let mut result_4046_sales = 0u32;
    let mut result_4225_sales = 0u32;
    let mut result_4770_sales = 0u32;

    for sale_details in sales_details.iter().filter(|sd| sd.year == year_to_explore) {
        result_4046_sales += sale_details.plu_4046;
        result_4225_sales += sale_details.plu_4225;
        result_4770_sales += sale_details.plu_4770;
    }

    sales_results.push((String::from("4046"), result_4046_sales));
    sales_results.push((String::from("4225"), result_4225_sales));
    sales_results.push((String::from("4770"), result_4770_sales));

    sales_results
}
