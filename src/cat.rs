use async_graphql::{FieldResult, SimpleObject};

#[derive(serde::Deserialize, SimpleObject)]
pub(crate) struct CatFactResponse {
    fact: String,
}

pub(crate) async fn get_cat_fact() -> FieldResult<CatFactResponse> {
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
        })
}
