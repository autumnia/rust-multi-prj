use axum::{
    extract::{State, Path},
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

pub async fn get_products(State(pool): State<PgPool>)
    -> Result<Json<Vec<Product>>, (StatusCode, String)> {
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

pub async fn get_one_product(State(pool): State<PgPool>, Path(id): Path<i32>)
    -> Result<Json<Product>, (StatusCode, String)> {
    let result = sqlx::query_as(
        r#"
            select
                id, name, price
            from
                products
            where
                id = $1;
        "#
    )
    .bind(id)
    .fetch_one(&pool)
    .await
    .map_err( | err| match err {
           sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, format!("Error is {}", err)),
            _                       => (StatusCode::INTERNAL_SERVER_ERROR, format!("Error is {}", err))

    })?;

    Ok(Json(result))
}

pub async fn delete_one_product(State(pool): State<PgPool>, Path(id): Path<i32>)
    -> Result<Json<Value>, (StatusCode, String)> {
    let _result = sqlx::query(
        r#"
            delete from
                products
            where
                id = $1;
        "#
    )
    .bind(id)
    .execute(&pool)
    .await
    .map_err( | err| match err {
        sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, format!("Error is {}", err)),
        _                       => (StatusCode::INTERNAL_SERVER_ERROR, format!("Error is {}", err))

    })?;

    Ok( Json( json!({"msg": "product deleted successfully"}) ) )
}

pub async fn update_one_product(State(pool): State<PgPool>, Path(id): Path<i32>, Json(product): Json<Product>)
    -> Result<Json<Value>, (StatusCode, String)> {
    let _result = sqlx::query(
        r#"
            update products
            set name=$1, price=$2
            where
                id = $3;
        "#
    )
    .bind( &product.name )
    .bind( &product.price )
    .bind(id)
    .execute(&pool)
    .await
    .map_err( |err| match err {
        sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, format!("Error is {}", err)),
        _                       => (StatusCode::INTERNAL_SERVER_ERROR, format!("Error is {}", err))

    })?;

    Ok( Json( json!({"msg": "product updated successfully"}) ) )
}