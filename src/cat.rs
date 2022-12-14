use async_graphql::FieldResult;

#[derive(serde::Deserialize)]
struct CatFactResponse {
    fact: String,
}

pub(crate) async fn get_cat_fact() -> FieldResult<String> {
    reqwest::get("https://catfact.ninja/fact")
        .await
        .map_err(|err| {
            tracing::error!(?err, "Error getting cat fact");
            async_graphql::FieldError::new("Error getting cat fact")
        })?
        .json::<CatFactResponse>()
        .await
        .map_err(|err| {
            tracing::error!(?err, "Error parsing cat fact");
            async_graphql::FieldError::new("Error parsing cat fact")
        }).map(|response| response.fact)
}
