pub async fn with_correct_env<F>(closure: F)
where
    F: std::future::Future<Output = ()> + std::future::IntoFuture,
{
    temp_env::async_with_vars(
        [
            ("POSTGRES_USER", Some("postgres")),
            ("POSTGRES_PASSWORD", Some("postgres")),
            ("POSTGRES_HOST", Some("localhost")),
            ("POSTGRES_DATABASE", Some("postgres")),
        ],
        closure,
    )
    .await;
}
