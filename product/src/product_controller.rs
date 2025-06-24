use axum::{
    extract::State,
    http::StatusCode,
    Json
};

use serde::{
    Deserialize,
    Serialize
};

use serde_json::{
    json,
    Value
};

use sqlx::postgres::PgPool;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateProduct {
    name: String,
    price: i32
}

pub async fn create_product(
    State(pool): State<PgPool>,
    Json(product): Json<CreateProduct>)
    -> Result<Json<Value>, (StatusCode, String) > {
    let _result = sqlx::query(
        r#"
                insert into
                products (name, price)
                values ($1, $2);
            "#
        )
        .bind(&product.name)
        .bind(&product.price)
        .execute(&pool)
        .await
        .map_err(
            |err| (StatusCode::INTERNAL_SERVER_ERROR, format!("Error is: {}", err) )
        )?;
    Ok(Json(json!(product)))
}


#[derive(Debug, Serialize, Deserialize, sqlx::FromRow )]
pub struct Product {
    id: i32,
    name: String,
    price: i32
}

pub async fn get_products(
    State(pool): State<PgPool>)
    -> Result<Json<Vec<Product>>, (StatusCode, String)  >{
    let result = sqlx::query_as(
    r#"
            select
                *
            from
                products;
        "#
    )
    .fetch_all(&pool)
    .await
    .map_err(
        |err| (StatusCode::INTERNAL_SERVER_ERROR, format!("Error is: {}", err) )
    )?;

    Ok(Json(result))
}

