// NexCore -- Nexflow Parser: ApiDSL Builder
// Copyright (c) 2024-2026 Chandra Mohn. All Rights Reserved.

//! Builds a typed `ApiDefinition` AST from ApiDSL source text.
//!
//! Entry point: `parse_api(source) -> Result<ApiDefinition>`

use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::tree::ParseTree;
use antlr_rust::InputStream;

use crate::ast::api::*;
use crate::ast::common::*;
use crate::generated::apidsllexer::ApiDSLLexer;
use crate::generated::apidslparser::*;
use crate::parse::helpers::{api_text as terminal_text, unquote};
use super::ParseError;

/// Parse an `.api` source string into a typed `ApiDefinition`.
///
/// # Errors
///
/// Returns `ParseError::Grammar` if the ANTLR4 parser fails to produce a parse tree.
/// Returns `ParseError::Ast` if a required AST node is missing from the parse tree.
pub fn parse_api(source: &str) -> Result<ApiDefinition, ParseError> {
    let input = InputStream::new(source);
    let lexer = ApiDSLLexer::new(input);
    let token_stream = CommonTokenStream::new(lexer);
    let mut parser = ApiDSLParser::new(token_stream);

    let tree = parser
        .compilationUnit()
        .map_err(|e| ParseError::grammar("ApiDSL", format!("{e:?}")))?;

    build_compilation_unit(&*tree)
}

fn build_compilation_unit(ctx: &CompilationUnitContext<'_>) -> Result<ApiDefinition, ParseError> {
    let imports: Vec<ImportPath> = ctx
        .importStatement_all()
        .iter()
        .filter_map(|s| build_import(s).ok())
        .collect();

    let api_ctx = ctx
        .apiDefinition()
        .ok_or_else(|| ParseError::ast("ApiDSL", "Missing api definition"))?;

    let mut def = build_api_definition(&*api_ctx)?;
    def.imports = imports;
    Ok(def)
}

fn build_import(ctx: &ImportStatementContext<'_>) -> Result<ImportPath, ParseError> {
    let path_ctx = ctx
        .importPath()
        .ok_or_else(|| ParseError::ast("ApiDSL", "Missing import path"))?;
    Ok(ImportPath {
        raw: path_ctx.get_text(),
    })
}

fn build_api_definition(ctx: &ApiDefinitionContext<'_>) -> Result<ApiDefinition, ParseError> {
    let name = ctx
        .IDENTIFIER()
        .map(|n| terminal_text(&*n))
        .unwrap_or_default();

    let mut def = ApiDefinition {
        imports: Vec::new(),
        name,
        version: None,
        base_path: None,
        description: None,
        targets: Vec::new(),
        auth_default: None,
        content_type: None,
        rate_limit: None,
        pagination: None,
        config: Vec::new(),
        cors: None,
        endpoints: Vec::new(),
        events: Vec::new(),
        dependencies: Vec::new(),
        health_path: None,
        ready_path: None,
    };

    if let Some(body) = ctx.apiBody() {
        for elem in body.apiElement_all() {
            apply_api_element(&mut def, &*elem);
        }
    }

    Ok(def)
}

fn apply_api_element(def: &mut ApiDefinition, ctx: &ApiElementContext<'_>) {
    if let Some(v) = ctx.versionDecl() {
        def.version = v.STRING_LITERAL().map(|n| unquote(&terminal_text(&*n)));
    }
    if let Some(b) = ctx.baseDecl() {
        def.base_path = b.STRING_LITERAL().map(|n| unquote(&terminal_text(&*n)));
    }
    if let Some(d) = ctx.descriptionDecl() {
        def.description = d.STRING_LITERAL().map(|n| unquote(&terminal_text(&*n)));
    }
    if let Some(t) = ctx.targetsDecl() {
        def.targets = build_identifier_list(&*t);
    }
    if let Some(a) = ctx.authDefaultDecl() {
        if let Some(scheme) = a.authScheme() {
            def.auth_default = Some(build_auth_scheme(&*scheme));
        }
    }
    if let Some(ct) = ctx.contentTypeDecl() {
        def.content_type = ct.IDENTIFIER().map(|n| terminal_text(&*n));
    }
    if let Some(rl) = ctx.rateLimitDecl() {
        def.rate_limit = Some(build_rate_limit(&*rl));
    }
    if let Some(p) = ctx.paginationDecl() {
        def.pagination = Some(build_pagination(&*p));
    }
    if let Some(c) = ctx.configBlock() {
        def.config = build_config_block(&*c);
    }
    if let Some(cors) = ctx.corsBlock() {
        def.cors = Some(build_cors(&*cors));
    }
    if let Some(ep) = ctx.endpointDecl() {
        def.endpoints.push(build_endpoint(&*ep, false));
    }
    if let Some(dep) = ctx.deprecatedEndpointDecl() {
        def.endpoints.push(build_deprecated_endpoint(&*dep));
    }
    if let Some(ev) = ctx.eventDecl() {
        def.events.push(build_event(&*ev));
    }
    if let Some(deps) = ctx.dependenciesBlock() {
        def.dependencies = build_dependencies(&*deps);
    }
    if let Some(h) = ctx.healthDecl() {
        def.health_path = h.STRING_LITERAL().map(|n| unquote(&terminal_text(&*n)));
    }
    if let Some(r) = ctx.readyDecl() {
        def.ready_path = r.STRING_LITERAL().map(|n| unquote(&terminal_text(&*n)));
    }
}

fn build_identifier_list(ctx: &TargetsDeclContext<'_>) -> Vec<String> {
    if let Some(list) = ctx.identifierList() {
        list.IDENTIFIER_all()
            .iter()
            .map(|n| terminal_text(n))
            .filter(|s| s != ",")
            .collect()
    } else {
        Vec::new()
    }
}

fn build_auth_scheme(ctx: &AuthSchemeContext<'_>) -> AuthScheme {
    let scheme = ctx
        .IDENTIFIER()
        .map(|n| terminal_text(&*n))
        .unwrap_or_default();

    let mut scope = None;
    let mut header = None;

    for arg in ctx.authSchemeArg_all() {
        if let Some(s) = arg.STRING_LITERAL() {
            let text = unquote(&terminal_text(&*s));
            // Check if this is a scope or header arg by looking at tokens
            let full = arg.get_text();
            if full.starts_with("scope") {
                scope = Some(text);
            } else if full.starts_with("header") {
                header = Some(text);
            }
        }
    }

    AuthScheme {
        scheme,
        scope,
        header,
    }
}

fn build_rate_limit(ctx: &RateLimitDeclContext<'_>) -> RateLimit {
    let values: Vec<_> = ctx.valueOrCfg_all();
    let limit = values
        .first()
        .map(|v| build_value_or_cfg(v))
        .unwrap_or(ValueOrCfg::Integer(0));
    let burst = values.get(1).map(|v| build_value_or_cfg(v));

    let per = ctx
        .IDENTIFIER()
        .map(|n| terminal_text(&*n))
        .unwrap_or_else(|| "minute".to_string());

    RateLimit { limit, per, burst }
}

fn build_pagination(ctx: &PaginationDeclContext<'_>) -> Pagination {
    let style = ctx
        .IDENTIFIER()
        .map(|n| terminal_text(&*n))
        .unwrap_or_default();

    let default_size = ctx
        .INTEGER(0)
        .and_then(|n| terminal_text(&*n).parse().ok())
        .unwrap_or(25);
    let max_size = ctx
        .INTEGER(1)
        .and_then(|n| terminal_text(&*n).parse().ok())
        .unwrap_or(100);

    Pagination {
        style,
        default_size,
        max_size,
    }
}

fn build_config_block(ctx: &ConfigBlockContext<'_>) -> Vec<ConfigDirective> {
    ctx.configDirective_all()
        .iter()
        .map(|d| {
            let key = d
                .IDENTIFIER()
                .map(|n| terminal_text(&*n))
                .unwrap_or_default();
            let values = d
                .configValue_all()
                .iter()
                .map(|v| build_config_value(v))
                .collect();
            ConfigDirective { key, values }
        })
        .collect()
}

fn build_config_value(ctx: &ConfigValueContext<'_>) -> ConfigValue {
    if let Some(n) = ctx.INTEGER() {
        ConfigValue::Integer(terminal_text(&*n).parse().unwrap_or(0))
    } else if let Some(n) = ctx.DECIMAL_LITERAL() {
        ConfigValue::Decimal(terminal_text(&*n).parse().unwrap_or(0.0))
    } else if let Some(n) = ctx.STRING_LITERAL() {
        ConfigValue::String(unquote(&terminal_text(&*n)))
    } else if ctx.TRUE().is_some() {
        ConfigValue::Boolean(true)
    } else if ctx.FALSE().is_some() {
        ConfigValue::Boolean(false)
    } else if let Some(n) = ctx.IDENTIFIER() {
        ConfigValue::Identifier(terminal_text(&*n))
    } else {
        ConfigValue::String(ctx.get_text())
    }
}

fn build_cors(ctx: &CorsBlockContext<'_>) -> CorsConfig {
    let mut cors = CorsConfig {
        origins: Vec::new(),
        methods: Vec::new(),
        headers: Vec::new(),
        max_age: None,
    };

    for dir in ctx.corsDirective_all() {
        let text = dir.get_text();
        if text.starts_with("origins") {
            if let Some(list) = dir.valueOrCfgList() {
                cors.origins = list
                    .valueOrCfg_all()
                    .iter()
                    .map(|v| build_value_or_cfg(v))
                    .collect();
            }
        } else if text.starts_with("methods") {
            if let Some(list) = dir.httpMethodList() {
                cors.methods = list
                    .httpMethod_all()
                    .iter()
                    .map(|m| build_http_method(m))
                    .collect();
            }
        } else if text.starts_with("headers") {
            if let Some(list) = dir.stringList() {
                cors.headers = list
                    .STRING_LITERAL_all()
                    .iter()
                    .map(|n| unquote(&terminal_text(n)))
                    .collect();
            }
        } else if text.starts_with("max_age") {
            if let Some(v) = dir.valueOrCfg() {
                cors.max_age = Some(build_value_or_cfg(&*v));
            }
        }
    }

    cors
}

fn build_endpoint(ctx: &EndpointDeclContext<'_>, deprecated: bool) -> Endpoint {
    let name = ctx
        .IDENTIFIER()
        .map(|n| terminal_text(&*n))
        .unwrap_or_default();

    let mut ep = Endpoint {
        name,
        method: None,
        path: None,
        description: None,
        params: Vec::new(),
        query: Vec::new(),
        headers: Vec::new(),
        request: None,
        response: None,
        errors: Vec::new(),
        auth: None,
        validate: None,
        rate_limit: None,
        timeout: None,
        cache: None,
        idempotent_key: None,
        is_async: false,
        is_deprecated: deprecated,
        sunset: None,
        replacement: None,
    };

    if let Some(body) = ctx.endpointBody() {
        for clause in body.endpointClause_all() {
            apply_endpoint_clause(&mut ep, &*clause);
        }
    }

    ep
}

fn build_deprecated_endpoint(ctx: &DeprecatedEndpointDeclContext<'_>) -> Endpoint {
    let name = ctx
        .IDENTIFIER()
        .map(|n| terminal_text(&*n))
        .unwrap_or_default();

    let mut ep = Endpoint {
        name,
        method: None,
        path: None,
        description: None,
        params: Vec::new(),
        query: Vec::new(),
        headers: Vec::new(),
        request: None,
        response: None,
        errors: Vec::new(),
        auth: None,
        validate: None,
        rate_limit: None,
        timeout: None,
        cache: None,
        idempotent_key: None,
        is_async: false,
        is_deprecated: true,
        sunset: None,
        replacement: None,
    };

    if let Some(body) = ctx.endpointBody() {
        for clause in body.endpointClause_all() {
            apply_endpoint_clause(&mut ep, &*clause);
        }
    }

    if let Some(s) = ctx.sunsetDecl() {
        ep.sunset = s.STRING_LITERAL().map(|n| unquote(&terminal_text(&*n)));
    }
    if let Some(r) = ctx.replacementDecl() {
        ep.replacement = r.IDENTIFIER().map(|n| terminal_text(&*n));
    }

    ep
}

fn apply_endpoint_clause(ep: &mut Endpoint, ctx: &EndpointClauseContext<'_>) {
    if let Some(m) = ctx.methodDecl() {
        if let Some(hm) = m.httpMethod() {
            ep.method = Some(build_http_method(&*hm));
        }
    }
    if let Some(p) = ctx.pathDecl() {
        ep.path = p.STRING_LITERAL().map(|n| unquote(&terminal_text(&*n)));
    }
    if let Some(d) = ctx.descriptionDecl() {
        ep.description = d.STRING_LITERAL().map(|n| unquote(&terminal_text(&*n)));
    }
    if let Some(pb) = ctx.paramsBlock() {
        ep.params = pb
            .paramDecl_all()
            .iter()
            .map(|p| build_param(p))
            .collect();
    }
    if let Some(qb) = ctx.queryBlock() {
        ep.query = qb
            .paramDecl_all()
            .iter()
            .map(|p| build_param(p))
            .collect();
    }
    if let Some(hb) = ctx.headersBlock() {
        ep.headers = hb
            .paramDecl_all()
            .iter()
            .map(|p| build_param(p))
            .collect();
    }
    if let Some(r) = ctx.requestDecl() {
        if let Some(sr) = r.schemaRef() {
            ep.request = Some(build_schema_ref(&*sr));
        }
    }
    if let Some(r) = ctx.responseDecl() {
        ep.response = Some(build_response(&*r));
    }
    if let Some(eb) = ctx.errorsBlock() {
        ep.errors = eb
            .errorMapping_all()
            .iter()
            .map(|e| build_error_mapping(e))
            .collect();
    }
    if let Some(a) = ctx.authDecl() {
        if let Some(scheme) = a.authScheme() {
            ep.auth = Some(build_auth_scheme(&*scheme));
        }
    }
    if let Some(v) = ctx.validateDecl() {
        if let Some(qr) = v.qualifiedRef() {
            ep.validate = Some(build_qualified_ref(&*qr));
        }
    }
    if let Some(rl) = ctx.rateLimitDecl() {
        ep.rate_limit = Some(build_rate_limit(&*rl));
    }
    if let Some(t) = ctx.timeoutDecl() {
        ep.timeout = Some(build_timeout(&*t));
    }
    if let Some(c) = ctx.cacheDecl() {
        ep.cache = Some(build_cache(&*c));
    }
    if let Some(i) = ctx.idempotentDecl() {
        ep.idempotent_key = i.STRING_LITERAL().map(|n| unquote(&terminal_text(&*n)));
    }
    if ctx.asyncDecl().is_some() {
        ep.is_async = true;
    }
}

fn build_param(ctx: &ParamDeclContext<'_>) -> ParamDecl {
    let name = ctx
        .IDENTIFIER()
        .map(|n| terminal_text(&*n))
        .unwrap_or_default();

    let type_ref = ctx
        .typeRef()
        .map(|t| t.get_text())
        .unwrap_or_default();

    let mut required = false;
    let mut optional = false;
    let mut default = None;

    for m in ctx.paramModifier_all() {
        let text = m.get_text();
        if text.starts_with("required") {
            required = true;
        } else if text.starts_with("optional") {
            optional = true;
        } else if text.starts_with("default") {
            if let Some(lit) = m.literal() {
                default = Some(lit.get_text().replace('"', ""));
            }
        }
    }

    ParamDecl {
        name,
        type_ref,
        required,
        optional,
        default,
    }
}

fn build_response(ctx: &ResponseDeclContext<'_>) -> ResponseDecl {
    let modifier = ctx.responseModifier().map(|m| {
        let text = m.get_text();
        if text == "paginated" {
            ResponseModifier::Paginated
        } else {
            ResponseModifier::List
        }
    });

    let schema = ctx
        .schemaRef()
        .map(|s| build_schema_ref(&*s))
        .unwrap_or_else(|| SchemaRef {
            name: String::new(),
            qualifier: None,
        });

    ResponseDecl { modifier, schema }
}

fn build_error_mapping(ctx: &ErrorMappingContext<'_>) -> ErrorMapping {
    let status_code = ctx
        .INTEGER()
        .map(|n| terminal_text(&*n).parse().unwrap_or(0))
        .unwrap_or(0);

    let schema = ctx
        .schemaRef()
        .map(|s| build_schema_ref(&*s))
        .unwrap_or_else(|| SchemaRef {
            name: String::new(),
            qualifier: None,
        });

    ErrorMapping {
        status_code,
        schema,
    }
}

fn build_timeout(ctx: &TimeoutDeclContext<'_>) -> TimeoutDecl {
    let value = ctx
        .valueOrCfg()
        .map(|v| build_value_or_cfg(&*v))
        .unwrap_or(ValueOrCfg::Integer(0));

    let unit = ctx
        .IDENTIFIER()
        .map(|n| terminal_text(&*n))
        .unwrap_or_else(|| "seconds".to_string());

    TimeoutDecl { value, unit }
}

fn build_cache(ctx: &CacheDeclContext<'_>) -> CacheDecl {
    let duration = ctx
        .valueOrCfg()
        .map(|v| build_value_or_cfg(&*v))
        .unwrap_or(ValueOrCfg::Integer(0));

    let unit = ctx
        .IDENTIFIER()
        .map(|n| terminal_text(&*n))
        .unwrap_or_else(|| "seconds".to_string());

    CacheDecl { duration, unit }
}

fn build_event(ctx: &EventDeclContext<'_>) -> EventContract {
    let name = ctx
        .IDENTIFIER()
        .map(|n| terminal_text(&*n))
        .unwrap_or_default();

    let mut event = EventContract {
        name,
        topic: None,
        payload: None,
        partitioned_by: None,
    };

    if let Some(body) = ctx.eventBody() {
        for clause in body.eventClause_all() {
            if let Some(t) = clause.topicDecl() {
                event.topic = t.STRING_LITERAL().map(|n| unquote(&terminal_text(&*n)));
            }
            if let Some(p) = clause.payloadDecl() {
                if let Some(sr) = p.schemaRef() {
                    event.payload = Some(build_schema_ref(&*sr));
                }
            }
            if let Some(pb) = clause.partitionedByDecl() {
                event.partitioned_by = pb.IDENTIFIER().map(|n| terminal_text(&*n));
            }
        }
    }

    event
}

fn build_dependencies(ctx: &DependenciesBlockContext<'_>) -> Vec<Dependency> {
    ctx.dependencyDecl_all()
        .iter()
        .map(|d| {
            let api_name = d
                .IDENTIFIER()
                .map(|n| terminal_text(&*n))
                .unwrap_or_default();
            let version = d
                .STRING_LITERAL()
                .map(|n| unquote(&terminal_text(&*n)))
                .unwrap_or_default();
            Dependency { api_name, version }
        })
        .collect()
}

// --- Shared builders ------------------------------------------------

fn build_value_or_cfg(ctx: &ValueOrCfgContext<'_>) -> ValueOrCfg {
    if let Some(n) = ctx.INTEGER() {
        ValueOrCfg::Integer(terminal_text(&*n).parse().unwrap_or(0))
    } else if let Some(n) = ctx.DECIMAL_LITERAL() {
        ValueOrCfg::Decimal(terminal_text(&*n).parse().unwrap_or(0.0))
    } else if let Some(n) = ctx.STRING_LITERAL() {
        ValueOrCfg::String(unquote(&terminal_text(&*n)))
    } else if let Some(qa) = ctx.qualifiedAnnotation() {
        let ids: Vec<_> = qa.IDENTIFIER_all();
        let namespace = ids.first().map(|n| terminal_text(n)).unwrap_or_default();
        let key = ids.get(1).map(|n| terminal_text(n)).unwrap_or_default();
        ValueOrCfg::CfgRef { namespace, key }
    } else {
        ValueOrCfg::String(ctx.get_text())
    }
}

fn build_schema_ref(ctx: &SchemaRefContext<'_>) -> SchemaRef {
    let ids: Vec<_> = ctx.IDENTIFIER_all();
    let name = ids
        .last()
        .map(|n| terminal_text(n))
        .unwrap_or_default();
    let qualifier = if ids.len() > 1 {
        ids.first().map(|n| terminal_text(n))
    } else {
        None
    };
    SchemaRef { name, qualifier }
}

fn build_qualified_ref(ctx: &QualifiedRefContext<'_>) -> QualifiedRef {
    let ids: Vec<_> = ctx.IDENTIFIER_all();
    let qualifier = ids.first().map(|n| terminal_text(n)).unwrap_or_default();
    let name = ids.get(1).map(|n| terminal_text(n)).unwrap_or_default();
    QualifiedRef { qualifier, name }
}

fn build_http_method(ctx: &HttpMethodContext<'_>) -> HttpMethod {
    let text = ctx.get_text();
    match text.as_str() {
        "GET" => HttpMethod::Get,
        "POST" => HttpMethod::Post,
        "PUT" => HttpMethod::Put,
        "PATCH" => HttpMethod::Patch,
        "DELETE" => HttpMethod::Delete,
        _ => HttpMethod::Get,
    }
}
