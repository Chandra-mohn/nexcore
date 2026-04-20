use reqwest::Client;

use crate::error::RegistryError;
use crate::models::{
    CompatibilityResult, RegistryConfig, SchemaVersionInfo, SubjectInfo,
    WireCompatibility, WireError, WireRegisterId, WireSchema,
};

/// Async client for the Confluent Schema Registry REST API.
#[derive(Debug)]
pub struct RegistryClient {
    client: Client,
    base_url: String,
}

impl RegistryClient {
    /// Create a new client from connection config.
    pub fn new(config: &RegistryConfig) -> Result<Self, RegistryError> {
        let url = config.url.trim_end_matches('/').to_string();
        if url.is_empty() {
            return Err(RegistryError::Config("URL must not be empty".into()));
        }

        let builder = Client::builder();
        let client = builder.build().map_err(RegistryError::Http)?;
        Ok(Self {
            client,
            base_url: url,
        })
    }

    /// Build a request with optional basic auth.
    fn get(&self, path: &str) -> reqwest::RequestBuilder {
        let url = format!("{}{path}", self.base_url);
        self.client.get(&url)
    }

    fn post(&self, path: &str) -> reqwest::RequestBuilder {
        let url = format!("{}{path}", self.base_url);
        self.client
            .post(&url)
            .header("Content-Type", "application/vnd.schemaregistry.v1+json")
    }

    /// Apply basic auth to a request if credentials are configured.
    /// We don't store creds on the struct directly -- caller passes config.
    fn with_auth(
        &self,
        req: reqwest::RequestBuilder,
        config: &RegistryConfig,
    ) -> reqwest::RequestBuilder {
        if let (Some(user), Some(pass)) = (&config.username, &config.password) {
            req.basic_auth(user, Some(pass))
        } else {
            req
        }
    }

    /// Check connectivity by listing subjects.
    pub async fn test_connection(
        &self,
        config: &RegistryConfig,
    ) -> Result<String, RegistryError> {
        let resp = self
            .with_auth(self.get("/subjects"), config)
            .send()
            .await?;

        if resp.status().is_success() {
            let subjects: Vec<String> = resp.json().await?;
            Ok(format!("Connected -- {} subjects", subjects.len()))
        } else {
            let err = parse_error(resp).await;
            Err(err)
        }
    }

    /// List all subject names.
    pub async fn list_subjects(
        &self,
        config: &RegistryConfig,
    ) -> Result<Vec<String>, RegistryError> {
        let resp = self
            .with_auth(self.get("/subjects"), config)
            .send()
            .await?;

        if resp.status().is_success() {
            Ok(resp.json().await?)
        } else {
            Err(parse_error(resp).await)
        }
    }

    /// Get summary info for a subject (versions list + latest schema type).
    pub async fn get_subject_info(
        &self,
        config: &RegistryConfig,
        subject: &str,
    ) -> Result<SubjectInfo, RegistryError> {
        // Fetch version list
        let versions_resp = self
            .with_auth(
                self.get(&format!("/subjects/{subject}/versions")),
                config,
            )
            .send()
            .await?;

        if !versions_resp.status().is_success() {
            return Err(parse_error(versions_resp).await);
        }

        let versions: Vec<i32> = versions_resp.json().await?;
        let latest_version = versions.last().copied().unwrap_or(0);

        // Fetch latest version to get schema type
        let latest = self
            .get_schema(config, subject, latest_version)
            .await?;

        Ok(SubjectInfo {
            name: subject.to_string(),
            versions,
            latest_version,
            schema_type: latest.schema_type,
        })
    }

    /// List subjects with full info (versions + schema type per subject).
    pub async fn list_subjects_with_info(
        &self,
        config: &RegistryConfig,
    ) -> Result<Vec<SubjectInfo>, RegistryError> {
        let subjects = self.list_subjects(config).await?;
        let mut infos = Vec::with_capacity(subjects.len());

        for subject in &subjects {
            match self.get_subject_info(config, subject).await {
                Ok(info) => infos.push(info),
                Err(_) => {
                    // If we can't fetch info for a subject, include it with minimal data
                    infos.push(SubjectInfo {
                        name: subject.clone(),
                        versions: vec![],
                        latest_version: 0,
                        schema_type: "UNKNOWN".into(),
                    });
                }
            }
        }

        Ok(infos)
    }

    /// Get a specific schema version.
    pub async fn get_schema(
        &self,
        config: &RegistryConfig,
        subject: &str,
        version: i32,
    ) -> Result<SchemaVersionInfo, RegistryError> {
        let resp = self
            .with_auth(
                self.get(&format!("/subjects/{subject}/versions/{version}")),
                config,
            )
            .send()
            .await?;

        if !resp.status().is_success() {
            return Err(parse_error(resp).await);
        }

        let wire: WireSchema = resp.json().await?;
        Ok(SchemaVersionInfo {
            subject: wire.subject.unwrap_or_else(|| subject.to_string()),
            version: wire.version.unwrap_or(version),
            id: wire.id.unwrap_or(0),
            schema_type: wire.schema_type,
            schema: wire.schema.unwrap_or_default(),
            references: wire.references,
        })
    }

    /// Get a schema by its global ID.
    pub async fn get_schema_by_id(
        &self,
        config: &RegistryConfig,
        id: i32,
    ) -> Result<String, RegistryError> {
        let resp = self
            .with_auth(self.get(&format!("/schemas/ids/{id}")), config)
            .send()
            .await?;

        if !resp.status().is_success() {
            return Err(parse_error(resp).await);
        }

        let wire: WireSchema = resp.json().await?;
        Ok(wire.schema.unwrap_or_default())
    }

    /// Register a new schema under a subject. Returns the global schema ID.
    pub async fn register_schema(
        &self,
        config: &RegistryConfig,
        subject: &str,
        schema: &str,
        schema_type: &str,
    ) -> Result<i32, RegistryError> {
        let body = serde_json::json!({
            "schema": schema,
            "schemaType": schema_type,
        });

        let resp = self
            .with_auth(
                self.post(&format!("/subjects/{subject}/versions")),
                config,
            )
            .json(&body)
            .send()
            .await?;

        if !resp.status().is_success() {
            return Err(parse_error(resp).await);
        }

        let wire: WireRegisterId = resp.json().await?;
        Ok(wire.id)
    }

    /// Test schema compatibility against a specific version (or "latest").
    pub async fn check_compatibility(
        &self,
        config: &RegistryConfig,
        subject: &str,
        version: &str,
        schema: &str,
        schema_type: &str,
    ) -> Result<CompatibilityResult, RegistryError> {
        let body = serde_json::json!({
            "schema": schema,
            "schemaType": schema_type,
        });

        let url = format!("/compatibility/subjects/{subject}/versions/{version}");
        let resp = self
            .with_auth(self.post(&url), config)
            .json(&body)
            .send()
            .await?;

        if !resp.status().is_success() {
            return Err(parse_error(resp).await);
        }

        let wire: WireCompatibility = resp.json().await?;
        Ok(CompatibilityResult {
            is_compatible: wire.is_compatible,
            messages: wire.messages,
        })
    }
}

async fn parse_error(resp: reqwest::Response) -> RegistryError {
    match resp.json::<WireError>().await {
        Ok(wire) => RegistryError::Api {
            code: wire.error_code,
            message: wire.message,
        },
        Err(e) => RegistryError::Http(e),
    }
}
