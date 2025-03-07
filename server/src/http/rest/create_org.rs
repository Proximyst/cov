pub async fn serve_create_org() -> impl IntoApiResponse {
    Json(CreateOrganisation {
        service: Service::new("service"),
        name: "name".to_string(),
    })
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
