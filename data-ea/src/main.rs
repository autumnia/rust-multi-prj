#![warn(unused_imports)]

use elusion::prelude::*;

// async fn template() -> Result<(), ElusionError> {
//     let path_data = "data/sales_order_report2.csv";
//     let df = CustomDataFrame::new(path_data, "sales_data").await?;
//
//     let result = df;
//
//     result.display().await?;
//     Ok(())
// }


#[tokio::main]
async fn main() -> ElusionResult<()> {
    println!("Hello Elusion!");

    agg_data().await?;
    // filter_data().await?;
    // select_data().await?;

    Ok(())
}

async fn agg_data() -> Result<(), ElusionError> {
    let path_data = "data-ea/data/sales_order_report2.csv";
    let df = CustomDataFrame::new(path_data, "sales_data").await?;

    const FILTER_CUSTOMER: &str = "customer_name == 'Customer IRRVL'";

    let result = df
        .select([
            "customer_name",
            "order_date",
            "ABS(billable_value) AS abs_billable_value",
            "ROUND(SQRT(billable_value), 2) AS SQRT_billable_value",
            // "billable_value * 2 AS double_billable_value" ,
            // "billable_value / 100 AS percentage_billable_value",
        ])
        .agg([
            "ROUND(AVG(ABS(billable_value)), 2) AS avg_abs_billable_value",
            "SUM(billable_value) AS total_billable_value",
            // "MAX(ABS(billable_value)) AS max_abs_billable_value",
            // "SUM(billable_value) * 2 AS duble_total_billable_value",
            // "SUM(billable_value) / 100 AS percentage_total_billable_value"
        ])
        .filter(FILTER_CUSTOMER)
        .group_by_all()
        .limit(20)
        .elusion("res_data").await?;

    result.display().await?;
    Ok(())
}

async fn filter_data() -> Result<(), ElusionError> {
    let path_data = "data-ea/data/sales_order_report2.csv";
    let df = CustomDataFrame::new(path_data, "sales_data").await?;
    let result = df
        .select([
            "customer_name",
            "order_date",
            "billable_value"
        ])
        .filter_many([("order_date > '2021-07-04'"), ("billable_value > 500.0")])
        // .filter("billable_value > 500.0")
        // .filter("order_date > '2021-07-04'")
        .order_by(["order_date"], [true])
        .limit(10)
        .elusion("df_alias").await?;

    result.display().await?;
    Ok(())
}

async fn select_data() -> Result<(), ElusionError> {
    let path_data = "data-ea/data/Customers.csv";
    let df = CustomDataFrame::new(path_data, "sales_data").await?;
    let result = df
        .select([
            "CustomerKey as customer_key",
            "FirstName as first_name",
            "LastName",
            "EmailAddress"
        ])
        .limit(10)
        .elusion("df_alias").await?;

    result.display().await?;
    Ok(())
}