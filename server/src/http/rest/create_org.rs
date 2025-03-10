use crate::database::{Database, Priority};
use aide::{axum::IntoApiResponse, transform::TransformOperation};
use axum::{Json, extract::State};
use chrono::Utc;
use proto::ping::Pong;

pub async fn serve_create_org(State(db): State<Database>) -> impl IntoApiResponse {
    db.create_organisation(Priority::High, svc, name)
}

pub fn transform_create_org(t: TransformOperation) -> TransformOperation {
    let example = CreateOrganisation {
        service: Service::new("service"),
        name: "name".to_string(),
    };

    t.description("Create a new organisation.")
        .security_requirement("create-organisation")
        .response_with::<200, Json<CreateOrganisation>, _>(|r| {
            r.description("A successful organisation creation.")
                .example(example)
        })
}
