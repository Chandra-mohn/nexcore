// NexCore -- Nexflow Codegen: Integration Test
// Copyright (c) 2024-2026 Chandra Mohn. All Rights Reserved.

//! End-to-end test: parse real .api + .schema files, generate project.

use nexflow_parser::{parse_api, parse_schema};

#[test]
fn generate_from_account_api() {
    // Parse API
    let api_source = include_str!("../../../examples/nexflow/api/account-api.api");
    let api = parse_api(api_source).expect("Failed to parse account-api.api");

    // Parse all schema files
    let schema_sources = [
        include_str!("../../../examples/nexflow/schema/account.schema"),
        include_str!("../../../examples/nexflow/schema/address.schema"),
        include_str!("../../../examples/nexflow/schema/transfer.schema"),
        include_str!("../../../examples/nexflow/schema/errors.schema"),
    ];

    let mut all_schemas = Vec::new();
    for source in &schema_sources {
        let program = parse_schema(source).expect("Failed to parse schema");
        all_schemas.extend(program.schemas);
    }

    // Generate
    let project =
        nexflow_codegen::generate(&api, &all_schemas).expect("Code generation failed");

    // Verify all expected files are present
    assert!(project.files.contains_key("src/models.rs"));
    assert!(project.files.contains_key("src/errors.rs"));
    assert!(project.files.contains_key("src/handlers.rs"));
    assert!(project.files.contains_key("src/router.rs"));
    assert!(project.files.contains_key("src/middleware.rs"));
    assert!(project.files.contains_key("src/lib.rs"));
    assert!(project.files.contains_key("Cargo.toml"));

    let models = &project.files["src/models.rs"];
    let errors = &project.files["src/errors.rs"];
    let handlers = &project.files["src/handlers.rs"];
    let router = &project.files["src/router.rs"];
    let middleware = &project.files["src/middleware.rs"];

    // -- Models assertions --

    // Schema structs for request/response/error types referenced by the API
    assert!(models.contains("pub struct AccountDetail {"), "missing AccountDetail struct");
    assert!(models.contains("pub struct AccountSummary {"), "missing AccountSummary struct");
    assert!(models.contains("pub struct AccountBalance {"), "missing AccountBalance struct");
    assert!(models.contains("pub struct CreateAccountRequest {"), "missing CreateAccountRequest struct");
    assert!(models.contains("pub struct TransferRequest {"), "missing TransferRequest struct");
    assert!(models.contains("pub struct TransferResult {"), "missing TransferResult struct");
    assert!(models.contains("pub struct AddressUpdate {"), "missing AddressUpdate struct");
    assert!(models.contains("pub struct AccountClosureReceipt {"), "missing AccountClosureReceipt struct");

    // Error schema structs
    assert!(models.contains("pub struct ValidationError {"), "missing ValidationError struct");
    assert!(models.contains("pub struct AccountNotFound {"), "missing AccountNotFound struct");
    assert!(models.contains("pub struct InsufficientFunds {"), "missing InsufficientFunds struct");
    assert!(models.contains("pub struct AccountAlreadyExists {"), "missing AccountAlreadyExists struct");
    assert!(models.contains("pub struct AccountHasBalance {"), "missing AccountHasBalance struct");

    // Event payload schemas
    assert!(models.contains("pub struct AccountCreatedEvent {"), "missing AccountCreatedEvent struct");
    assert!(models.contains("pub struct AccountClosedEvent {"), "missing AccountClosedEvent struct");
    assert!(models.contains("pub struct BalanceChangedEvent {"), "missing BalanceChangedEvent struct");
    assert!(models.contains("pub struct AddressChangedEvent {"), "missing AddressChangedEvent struct");

    // Enum constraints should be generated
    assert!(models.contains("pub enum AccountDetailStatus {"), "missing status enum");

    // Field types
    assert!(models.contains("uuid::Uuid"), "missing uuid type");
    assert!(models.contains("rust_decimal::Decimal"), "missing decimal type");
    assert!(models.contains("chrono::NaiveDate"), "missing date type");

    // -- Errors assertions --
    assert!(errors.contains("pub enum ApiError {"), "missing ApiError enum");
    assert!(errors.contains("impl IntoResponse for ApiError"), "missing IntoResponse impl");
    assert!(errors.contains("StatusCode::NOT_FOUND"), "missing 404 status");
    assert!(errors.contains("StatusCode::FORBIDDEN"), "missing 403 status");
    assert!(errors.contains("StatusCode::BAD_REQUEST"), "missing 400 status");
    assert!(errors.contains("StatusCode::CONFLICT"), "missing 409 status");
    assert!(errors.contains("StatusCode::UNPROCESSABLE_ENTITY"), "missing 422 status");

    // -- Handlers assertions --
    assert!(handlers.contains("pub async fn list_accounts"), "missing list_accounts handler");
    assert!(handlers.contains("pub async fn get_account"), "missing get_account handler");
    assert!(handlers.contains("pub async fn get_balance"), "missing get_balance handler");
    assert!(handlers.contains("pub async fn create_account"), "missing create_account handler");
    assert!(handlers.contains("pub async fn update_address"), "missing update_address handler");
    assert!(handlers.contains("pub async fn close_account"), "missing close_account handler");
    assert!(handlers.contains("pub async fn transfer_funds"), "missing transfer_funds handler");

    // Extractors
    assert!(handlers.contains("Path(account_id): Path<String>"), "missing path extractor");
    assert!(handlers.contains("Json(body): Json<models::CreateAccountRequest>"), "missing JSON body extractor");
    assert!(handlers.contains("Json(body): Json<models::TransferRequest>"), "missing transfer body extractor");

    // Query params for listAccounts
    assert!(handlers.contains("ListAccountsQuery"), "missing query struct");

    // -- Router assertions --
    assert!(router.contains("pub fn create_router() -> Router"), "missing create_router fn");
    assert!(router.contains("get(handlers::list_accounts)"), "missing list_accounts route");
    assert!(router.contains("post(handlers::create_account)"), "missing create_account route");
    assert!(router.contains("delete(handlers::close_account)"), "missing close_account route");

    // Deprecated endpoint should NOT appear in router
    assert!(!router.contains("get_account_v1"), "deprecated endpoint should be excluded");

    // -- Middleware assertions --
    assert!(middleware.contains("pub fn cors_layer() -> CorsLayer"), "missing cors_layer fn");
    assert!(middleware.contains("pub struct AuthClaims"), "missing AuthClaims struct");
    assert!(middleware.contains("Method::GET"), "missing GET method in CORS");

    // Print file sizes for diagnostics
    for (path, content) in &project.files {
        println!("{path}: {} bytes, {} lines", content.len(), content.lines().count());
    }
}
