use axum::{Extension,Json};
use serde_json::Value;
use sqlx::PgPool;
use std::collections::HashMap;

use crate::models;



pub async fn get_user_data_json_array(
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Value>, String> {
    let mut user_data: HashMap<String, serde_json::Value> = HashMap::new();

    let query = "SELECT c.id, c.name, c.phone, a.address, a.pincode
                FROM contact c
                LEFT JOIN address a ON c.id = a.contact_id".to_owned();

    let result = sqlx::query_as::<_, models::auth::get_json_data>(&query)
        .fetch_all(&pool)
        .await
        .map_err(|err| {
            dbg!(err);
            "error".to_string()
        })?;

    dbg!(&result);

    if !result.is_empty() {
        for row in result {
            let user_name = match row.name {
                Some(name) => name.clone(),
                None => {
                    // Handle the case where the user does not exist.
                    continue;
                },
            };

            let user_data_entry = user_data.entry(user_name.clone()).or_insert_with(|| {
                serde_json::json!({
                    "name": user_name.clone(),
                    "contact": row.phone,
                    "address": Vec::<serde_json::Value>::new(),
                })
            });

            let address_data = serde_json::json!({
                "address1": row.address,
                "pincode": row.pincode,
            });

            user_data_entry["address"].as_array_mut().unwrap().push(address_data);
        }
        println!("{:?} atre",user_data);
        let user_data_array: Vec<serde_json::Value> = user_data.values().cloned().collect();

        let response = Json(serde_json::Value::Array(user_data_array));
        Ok(response)
    } else {
        Err("No users found".to_string())
    }
}


pub async fn get_user(
    Json(credentials): Json<models::auth::User>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Value>, String> {
    // Extract the 'id' field from the JSON payload
    let id = &credentials.id;

    // Use a query with a WHERE clause to filter based on the 'id'
    let query = "SELECT c.id, c.name, c.phone, a.address, a.pincode
                 FROM contact c
                 LEFT JOIN address a ON c.id = a.contact_id
                 WHERE c.id = $1".to_owned();

    let result = sqlx::query_as::<_, models::auth::get_json_data>(&query)
        .bind(id) // Bind the 'id' value to the query parameter
        .fetch_all(&pool)
        .await
        .map_err(|err| {
            dbg!(err);
            "error".to_string()
        })?;

    dbg!(&result);

    if !result.is_empty() {
        // Create a JSON object to hold the response
        let mut user_data_object = serde_json::json!({
            "address": [],
            "contact": "",
            "name": "",
        });

        // Extract data from the result and format it into the response
        for row in result {
            // Extract the relevant data from the query result
            let address_data = serde_json::json!({
                "address1": row.address,
                "pincode": row.pincode,
            });

            // Push the address data into the "address" array in the response
            user_data_object["address"].as_array_mut().unwrap().push(address_data);

            // Set the "contact" and "name" fields in the response
            user_data_object["contact"] = serde_json::json!(row.phone);
            user_data_object["name"] = serde_json::json!(row.name);
        }

        let response = Json(user_data_object);
        Ok(response)
    } else {
        Err(format!("No user found for id: {}", id))
    }
}
