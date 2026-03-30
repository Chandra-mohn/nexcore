// Nexflow DSL Toolchain
// Copyright (c) 2024-2026 Mphasis Corporation. All Rights Reserved.
// Author: Chandra Mohn
//
// PROPRIETARY AND CONFIDENTIAL
// Unauthorized use, reproduction, or distribution is prohibited.

# Nexflow DSL Suite: .api and .service Grammar Design Specification

**Status:** DESIGN (brainstorm complete, grammar drafting next)
**Date:** 2026-03-30
**Author:** Chandra Mohn + Claude (AI-assisted design)

---

## 1. Executive Summary

This document captures the design decisions for two new Nexflow grammars:
- **ApiDSL (.api)** -- Declarative service contract specification
- **ServiceDSL (.service)** -- Microservice request/response orchestration

These grammars complete the Nexflow DSL suite, enabling full-stack application
development from data pipelines to web services, for both COBOL modernization
and greenfield development.

---

## 2. The 3-Layer / 7-Grammar Model

### 2.1 Design Principle

The Nexflow DSL suite is organized into three layers based on the nature of
each grammar's concerns:

```
Layer 1 -- Building Blocks (define WHAT)
+----------+   +----------+   +----------+
|  .schema |   |  .rules  |   |  .xform  |
+----------+   +----------+   +----------+
      \              |              /
       \             |             /
        +------------+------------+
                     |
Layer 2 -- Contracts (define the INTERFACE)
         +-----------+-----------+
         |                       |
    +--------+             +-----------+
    |  .api  |             |  .screen  |
    +--------+             +-----------+
         |                       |
Layer 3 -- Orchestration (define HOW things wire together)
         |                       |
    +----------+           +----------+
    | .service |           |  .proc   |
    +----------+           +----------+
```

### 2.2 Grammar Inventory

| Grammar | Extension | Layer | Role | Status |
|---|---|---|---|---|
| SchemaDSL | .schema | Building Block | Data types, structures, constraints, relationships | HAVE |
| RulesDSL | .rules | Building Block | Business decisions, validation, decision tables | HAVE |
| TransformDSL | .xform | Building Block | Data transformations, field mappings, computations | HAVE |
| ApiDSL | .api | Contract | Service contracts: endpoints, events, auth, versioning | NEW |
| ScreenDSL | .screen | Contract | UI contracts: forms, fields, navigation, layout | FUTURE |
| ServiceDSL | .service | Orchestration | Microservice orchestration (request/response) | NEW |
| ProcDSL | .proc | Orchestration | Stream/batch pipeline orchestration (data flow) | HAVE |

### 2.3 Layer Descriptions

**Layer 1 -- Building Blocks:** Define reusable, composable units of data,
logic, and transformation. These are the atoms of the system. They define
WHAT exists, not HOW or WHEN it executes.

**Layer 2 -- Contracts:** Define external-facing interfaces. These are
declarative specifications that describe WHAT an interface looks like from
the outside, without prescribing implementation. Contracts are published,
versioned, and shared across teams.

**Layer 3 -- Orchestration:** Wire building blocks together into running
systems. These are the execution grammars that define HOW things work.
Each orchestration grammar targets a different execution paradigm.

---

## 3. The Paradigm Split: .proc vs .service

### 3.1 Why Two Orchestration Grammars?

.proc and .service represent fundamentally different execution paradigms
with almost no primitive overlap:

| Concern | .proc (Stream/Batch) | .service (Request/Response) |
|---|---|---|
| Entry point | `receive records from stream` | HTTP/gRPC request arrives |
| Mental model | Continuous flow of records | Single request -> response |
| Core primitives | window, join, aggregate, watermark | validate, authorize, persist, respond |
| State model | Streaming (counters, TTL maps) | Request-scoped (transaction, cache) |
| Output | `emit to sink` | HTTP response + status code |
| Time semantics | Event time, watermarks, late data | Request timeout, SLA |
| Error model | Backpressure, dead letter, replay | HTTP 4xx/5xx, circuit breaker |
| Execution | Always-on (stream) or scheduled (batch) | On-demand, per-request |
| Scale model | Parallelism, partitioning | Concurrent requests, thread pool |

Trying to force microservice semantics into .proc (or vice versa) would
create a Frankenstein grammar. The primitives are distinct enough that
each paradigm deserves its own grammar.

### 3.2 How .proc and .service Interact

They are peers, not parent-child:

- `.proc` calls services via `.api` references: `call AccountAPI.getBalance`
- `.service` triggers `.proc` pipelines: `publish event` that a `.proc` receives
- Both reference the same `.schema`, `.rules`, `.xform` building blocks
- A system typically has BOTH: `.service` for the web layer, `.proc` for background processing

### 3.3 Scale Observation

In modern enterprise systems, there are more web-based applications (microservices)
than core data processing pipelines. .proc is optimized for the batch/stream/ETL
world. Without .service, Nexflow leaves the largest category of applications
without a dedicated orchestration grammar.

---

## 4. ApiDSL (.api) -- Service Contract Specification

### 4.1 Purpose

ApiDSL defines the published, versioned, external-facing contract of a service.
It is the Nexflow equivalent of OpenAPI + AsyncAPI + gRPC proto, unified in a
single grammar.

### 4.2 Design Rules

1. **No inline schema definitions.** Always reference `.schema` types.
2. **Protocol-agnostic contract.** The same .api generates REST, GraphQL, and gRPC.
3. **Events are part of the public contract.** Published domain events belong in .api.
4. **Dependencies are explicit.** Services declare what other .api contracts they consume.
5. **Versioning and deprecation are first-class.** Lifecycle management built into grammar.

### 4.3 What .api Covers

| Concern | Description |
|---|---|
| Service identity | Name, version, base path, description |
| Endpoints | Method, path, path/query/header params |
| Request/response | References to .schema types |
| Error contracts | Error codes mapped to .schema error types |
| Authentication | Auth schemes (bearer, API key, OAuth, mTLS), scopes, roles |
| Rate limiting | SLA declarations (requests per time window) |
| Versioning | URL-based, header-based; deprecation with sunset dates |
| Protocol targets | REST, GraphQL, gRPC generation targets |
| Event contracts | Topics published, payload schemas, partitioning |
| Dependencies | Other .api contracts this service consumes |
| Pagination | Cursor, offset, keyset patterns |
| Caching | Cache directives for GET endpoints |
| CORS | Allowed origins, methods, headers |
| Health/readiness | Standard health endpoint declarations |

### 4.4 Example: Full API Contract

```
import ./schemas/account.schema
import ./schemas/errors.schema

api AccountAPI
    version "2.1"
    base "/api/v2/accounts"
    targets rest, graphql, grpc
    auth default bearer scope "accounts:read"
    content_type json
    rate_limit 1000 per minute burst 50
    pagination cursor default_size 50 max_size 200

    // --- Sync Endpoints ---

    endpoint listAccounts
        method GET
        path "/"
        query
            status AccountStatus.schema optional
            region string optional
            since datetime optional
        end
        response paginated AccountSummary.schema
        cache 60 seconds
    end

    endpoint getBalance
        method GET
        path "/{accountId}/balance"
        params
            accountId string required
        end
        response AccountBalance.schema
        errors
            404 using AccountNotFound.schema
            403 using InsufficientPermission.schema
        end
        auth bearer scope "accounts:balance"
        timeout 5 seconds
    end

    endpoint updateAddress
        method PUT
        path "/{accountId}/address"
        request AddressUpdate.schema
        response AccountDetail.schema
        validate using AddressValidation.rules
        auth bearer scope "accounts:write"
        idempotent key "X-Idempotency-Key"
    end

    endpoint transferFunds
        method POST
        path "/transfers"
        request TransferRequest.schema
        response TransferResult.schema
        auth bearer scope "accounts:transfer"
        rate_limit 100 per minute
        timeout 30 seconds
        async    // Returns 202 Accepted
    end

    // --- Async Event Contracts (published domain events) ---

    event AccountCreated
        topic "accounts.created"
        payload AccountCreatedEvent.schema
        partitioned_by account_id
    end

    event BalanceChanged
        topic "accounts.balance.changed"
        payload BalanceChangedEvent.schema
    end

    // --- Service Dependencies ---

    dependencies
        AuditAPI version "1.x"
        NotificationAPI version "2.x"
    end

    // --- Deprecation Lifecycle ---

    deprecated endpoint getAccountV1
        method GET
        path "/v1/{accountId}"
        sunset "2026-12-31"
        replacement getBalance
    end
end
```

### 4.5 Compilation Targets

From a single .api contract, the Nexflow compiler generates:

| Target | Output |
|---|---|
| REST | OpenAPI 3.x spec + Spring Boot / Quarkus controllers |
| GraphQL | SDL schema + resolver stubs |
| gRPC | .proto files + service stubs |
| AsyncAPI | Event contract specs + Kafka producer/consumer config |
| Client SDKs | TypeScript, Java, Python client libraries |
| Documentation | API reference docs (HTML/Markdown) |

This is a major differentiator: write once in .api, deploy to any protocol.

### 4.6 GraphQL Relationship

GraphQL is a compilation target for .api, not a parallel grammar:

| GraphQL Concept | Nexflow Equivalent |
|---|---|
| `type User { name: String }` | .schema |
| `input CreateUserInput { ... }` | .schema (request type) |
| `type Query { getUser(id: ID!): User }` | .api endpoint (method GET) |
| `type Mutation { createUser(...): User }` | .api endpoint (method POST/PUT) |
| `type Subscription { orderUpdated: Order }` | .api event |
| Resolver function | .service handler |
| Apollo Federation | .api dependencies + composition |

GraphQL combines schema + contract + protocol into one SDL. Nexflow separates
them (.schema + .api + protocol choice), enabling multi-protocol generation.

---

## 5. ServiceDSL (.service) -- Microservice Orchestration

### 5.1 Purpose

ServiceDSL defines the implementation of a microservice: how incoming requests
are handled, how building blocks (.schema, .rules, .xform) are wired together,
and how service-level concerns (middleware, caching, transactions) are managed.

### 5.2 The M:N Relationship with .api

The .api <-> .service relationship is M:N, expressed through two verbs:

- **`implements`** = "I provide these endpoints" (inbound contracts)
- **`consumes`** = "I depend on these services" (outbound dependencies)

One service can implement multiple APIs:
```
service AccountService
    implements AccountAPI, AccountAdminAPI
```

One API can be implemented by multiple services (gateway, BFF, version migration):
```
service MobileBffService
    implements MobileBffAPI
    consumes AccountAPI, PaymentAPI    // Aggregates backend services
```

### 5.3 What .service Covers

| Concern | Description |
|---|---|
| API binding | implements/consumes declarations |
| Request handlers | Per-endpoint implementation pipelines |
| Middleware chains | Logging, tracing, auth verification, rate limit enforcement |
| Transaction management | Begin/commit/rollback, saga orchestration |
| Cache strategies | Read-through, write-behind, invalidation rules |
| Service-to-service calls | Type-safe calls to consumed .api contracts |
| Event publishing | Domain events after operations |
| Error handling | Exception mapping to HTTP status codes |
| Observability | Metrics, traces, log correlation |
| Health/readiness | Implementation of health check logic |

### 5.4 Example: Service Implementation

```
import ./apis/account-api.api
import ./apis/audit-api.api
import ./rules/account-validation.rules
import ./transforms/account-transform.xform

service AccountService
    implements AccountAPI
    consumes AuditAPI

    // --- Middleware Chain ---

    middleware
        logging correlation_id
        tracing opentelemetry
        metrics prometheus
    end

    // --- Request Handlers ---

    handler getBalance
        authorize scope "accounts:balance"
        validate using AccountValidation.rules
        transform request using NormalizeAccount.xform into normalized
        lookup account from AccountDB
        transform account using FormatBalance.xform into response
        on_error
            404 when account is null
            500 fallback
        end
    end

    handler transferFunds
        authorize scope "accounts:transfer"
        transaction
            validate using TransferValidation.rules
            call DebitService.debit
            call CreditService.credit
            persist to TransferDB
            publish AccountDebited
        on_rollback
            call DebitService.reverse
        end
    end

    handler updateAddress
        authorize scope "accounts:write"
        validate using AddressValidation.rules
        transform request using AddressNormalize.xform
        persist to AccountDB
        call AuditAPI.logChange    // Type-safe: compiler verifies endpoint exists
        publish AddressChanged
        respond 200 AccountDetail.schema
    end

    // --- Caching Strategy ---

    cache
        getBalance ttl 30 seconds invalidate_on updateAddress
    end

    // --- Health Checks ---

    health "/health"
        check database AccountDB timeout 5 seconds
        check service AuditAPI timeout 3 seconds
    end

    ready "/ready"
end
```

### 5.5 Use Cases

**Use Case 1: COBOL Modernization (Account System)**
```
Original: 15 COBOL programs in account subsystem
Result:
  account-api.api           -- External REST/GraphQL contract
  account-admin-api.api     -- Internal admin contract
  account-service.service   -- Implements both APIs
  account-batch.proc        -- Batch reporting pipeline
```

**Use Case 2: Greenfield E-commerce**
```
catalog-api.api + catalog-service.service
order-api.api + order-service.service (consumes PaymentAPI, CatalogAPI)
payment-api.api + payment-service.service
notification-api.api + notification-service.service
order-fulfillment.proc     -- Stream: order events -> fulfillment
inventory-sync.proc        -- Batch: nightly reconciliation
```

**Use Case 3: API Gateway / BFF Pattern**
```
mobile-bff-api.api
mobile-bff-service.service
    implements MobileBffAPI
    consumes AccountAPI, PaymentAPI

    handler getDashboard
        parallel
            call AccountAPI.getBalance into balance
            call PaymentAPI.getRecent into payments
        end
        transform using DashboardAssembler.xform into response
    end
```

---

## 6. Cross-Grammar Relationships

### 6.1 Reference Graph

```
.schema  <-----+-----+-----+-----+-----+-----+
                |     |     |     |     |     |
.rules   <-----+-----|-----|-----|-----|     |
                |     |     |     |           |
.xform   <-----+-----|-----|     |           |
                |     |     |     |           |
.api     <---------- .service    |           |
                      |           |           |
.api     <---------- .proc      .screen --> .api
```

- .api references .schema (request/response types, error types)
- .api references .rules (validation rules for endpoints)
- .service references .api (implements/consumes)
- .service references .schema, .rules, .xform (building blocks in handlers)
- .proc references .api (service calls, event subscriptions)
- .proc references .schema, .rules, .xform (building blocks in pipelines)
- .screen references .api (calls service endpoints)
- .screen references .schema (displays data types)

### 6.2 Event Flow

Events bridge .api, .service, and .proc:

1. **Declaration:** Event contracts live in .api (public interface)
2. **Publishing:** .service handlers publish events after operations
3. **Subscription:** .proc pipelines subscribe to events for processing
4. **Compiler verification:** All event references are type-checked across grammars

```
[.api declares event]  -->  [.service publishes event]  -->  [.proc subscribes to event]
```

### 6.3 Type Safety Across Grammars

The Nexflow compiler enforces type safety at grammar boundaries:
- .service handler signatures must match .api endpoint contracts
- .service `call X.method` verifies the endpoint exists in the consumed .api
- .proc `receive from X.Event` verifies the event is declared in the referenced .api
- .api `response X.schema` verifies the schema type exists

---

## 7. COBOL-to-DSL Extraction Mapping (Updated)

| COBOL Source | DSL Target | Extraction Source |
|---|---|---|
| WORKING-STORAGE, copybooks, FDs | .schema | AST (have) |
| IF/EVALUATE decision trees | .rules | AST (have) |
| MOVE, COMPUTE, arithmetic | .xform | AST (have) |
| PERFORM chains, paragraph flow | .proc | AST (have) |
| JCL jobs/steps | .proc (batch phases) | JCL parser (planned) |
| EXEC SQL COMMIT/ROLLBACK | .proc (transaction blocks) | AST (detectable) |
| CICS queues, START | .proc (events) | CICS parser (planned) |
| CALL graph + LINKAGE SECTION | .api (service contracts) | Intel graph (have) |
| CICS COMMAREA layout | .api (endpoints) | CICS parser (planned) |
| CICS RECEIVE/SEND MAP | .api (endpoints) + .screen | CICS parser (planned) |
| BMS map definitions | .screen (forms) | BMS parser (new) |
| Cross-program CALL graph | .api (dependencies) | Intel graph (have) |

---

## 8. Nexflow Compiler Target Mapping (Updated)

DSLs -> Hexagonal architecture layers:

```
.schema + .rules + .xform       -> Domain layer (entities, value objects, services)
.api                             -> Port layer (inbound interfaces / contracts)
.service                         -> Application layer (use cases, request handlers)
.proc (stream/batch/event)       -> Application layer (background jobs, pipelines)
.proc (service calls via .api)   -> Port layer (outbound interfaces)
.screen                          -> Adapter layer (UI controllers, forms)
```

---

## 9. Design Decisions Log

| # | Decision | Rationale | Date |
|---|---|---|---|
| D1 | 7-grammar / 3-layer model | .proc and .service are fundamentally different paradigms; contracts deserve their own layer | 2026-03-30 |
| D2 | No inline schema in .api | Single source of truth for types; avoids duplication with .schema | 2026-03-30 |
| D3 | .api <-> .service is M:N | Real-world services implement multiple APIs and consume multiple APIs | 2026-03-30 |
| D4 | GraphQL is a compilation target | .api is protocol-agnostic; same contract generates REST, GraphQL, gRPC | 2026-03-30 |
| D5 | Events declared in .api | Published events are part of the service's public contract | 2026-03-30 |
| D6 | implements/consumes verbs | Clear inbound/outbound distinction for service dependencies | 2026-03-30 |
| D7 | .schema, .rules, .xform unchanged | Building block grammars are stable; no reorganization needed | 2026-03-30 |
| D8 | Design .api and .service together | Tightly coupled grammars; contract and implementation co-designed | 2026-03-30 |
| D9 | Nexflow for both modernization and greenfield | Grammar design must serve both use cases equally well | 2026-03-30 |

---

## 10. Implementation Home

These grammars and their Rust implementation live in the nexcore workspace.
See docs/nexcore_architecture_spec.md for the full workspace design.

```
~/workspace/nexcore/
    grammar/nexflow/ApiDSL.g4
    grammar/nexflow/ServiceDSL.g4
    crates/nexflow-parser/          # ANTLR4 parsers + typed ASTs
    crates/nexflow-compiler/        # Type checker, semantic analysis
    crates/nexflow-codegen/         # Code generation backends
    crates/nexflow-lsp/             # LSP server (tower-lsp)
```

---

## 11. Open Items

| # | Item | Status |
|---|---|---|
| O1 | Draft .api ANTLR4 grammar (ApiDSL.g4) | NEXT |
| O2 | Draft .service ANTLR4 grammar (ServiceDSL.g4) | NEXT |
| O3 | Protocol-specific annotations (REST vs GraphQL vs gRPC differences) | DESIGN NEEDED |
| O4 | .proc extensions for batch (transaction block, service call enhancement) | FUTURE |
| O5 | .screen grammar design | FUTURE |
| O6 | Event subscription semantics (who declares subscriptions?) | DESIGN NEEDED |
| O7 | Saga/choreography patterns in .service | DESIGN NEEDED |
| O8 | Webhook contracts in .api | DESIGN NEEDED |

---

## 12. Next Steps

1. Create nexcore workspace with Cargo.toml and grammar/ directory
2. Write 3-4 complete .api example files covering: REST, events, multi-version, deprecation
3. Write 3-4 complete .service example files covering: CRUD, saga, BFF, event-driven
4. Extract grammar rules from examples
5. Draft ApiDSL.g4 and ServiceDSL.g4 ANTLR4 grammars
6. Set up antlr-rust build pipeline in nexflow-parser crate
7. Build typed AST structs and parser integration tests
