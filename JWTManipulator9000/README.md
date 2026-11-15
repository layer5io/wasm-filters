# JWTManipulator9000

A Proxy-WASM HTTP filter written in Rust to manipulate JSON Web Token (JWT) headers and payloads directly within Envoy and Istio service meshes. JWTManipulator9000 is part of the [Layer5 wasm-filters](https://github.com/layer5io/wasm-filters) collection, designed for experimentation, debugging, and claim mutation in cloud native environments orchestrated by [Meshery](https://meshery.io).

---

## 1. Overview

JWTManipulator9000 intercepts HTTP requests, extracts a JWT passed via the `Authorization` header, decodes its header and payload segments, performs configured claim transformations, and returns the modified token in the `jwt_test` response header.

### Supported Data Types
The current implementation deserializes both JWT headers and payloads into Rust string-to-string hash maps:
```rust
struct Jwt {
    headers: HashMap<String, String>,
    payload: HashMap<String, String>,
}
```
> [!IMPORTANT]
> **String-Only Claim Values:** All claims in both the JWT header and payload must be JSON strings (e.g., `"sub": "user123"`). Non-string claim values—such as integer timestamps (`"exp": 1700000000`), booleans (`"admin": true`), arrays, or nested objects—will cause deserialization to fail and trigger an Envoy `503 Service Unavailable` response.

---

## 2. Disclaimer and Security Warning

> [!WARNING]
> **Cryptographic Signature Invalidation & Security Notice**
>
> 1. **No Signature Regeneration:** This filter modifies the header and payload segments of a JWT, but does **not** regenerate, recompute, or re-sign the cryptographic signature segment (HMAC, RSA, ECDSA). The original signature segment is preserved and appended unchanged to the modified token.
> 2. **Broken Cryptographic Integrity:** Standard JWT validation (RFC 7519 / RFC 7515) requires verifying the signature over `base64(header).base64(payload)`. Modifying any claim without regenerating the signature mathematically invalidates the token. Any downstream service or gateway performing standard JWT verification will reject the modified token.
> 3. **No Authentication or Authorization:** This filter provides **no** cryptographic token validation, key verification, expiration checking, or access control protections.
> 4. **Controlled Environments Only:** JWTManipulator9000 is intended strictly for controlled development, testing, and mock scenarios. **Never use this filter in production environments as an authentication or security boundary.**

---

## 3. Features

The filter supports six claim modification operations configured via JSON:

| Operation | JSON Type | Description |
| :--- | :--- | :--- |
| `add_header` | Array of `[string, string]` pairs | Adds a new claim or overwrites an existing claim in the JWT header. |
| `del_header` | Array of strings (keys) | Removes specified claims from the JWT header. Safe if key does not exist. |
| `add_payload` | Array of `[string, string]` pairs | Adds a new claim or overwrites an existing claim in the JWT payload. |
| `del_payload` | Array of strings (keys) | Removes specified claims from the JWT payload. Safe if key does not exist. |
| `payload_to_header` | Array of strings (keys) | Moves a claim from the payload to the header (deletes from payload, inserts into header). **Key must exist in payload.** |
| `header_to_payload` | Array of strings (keys) | Moves a claim from the header to the payload (deletes from header, inserts into payload). **Key must exist in header.** |

### Operation Execution Order
Operations inside `modify_jwt()` execute in a **fixed sequence**, regardless of the key order in your configuration:
1. `add_header`
2. `del_header`
3. `add_payload`
4. `del_payload`
5. `payload_to_header`
6. `header_to_payload`

---

## 4. Architecture and Request Flow

The diagram below illustrates how Envoy initializes the WASM filter, loads configuration, processes downstream HTTP requests, and returns responses.

```mermaid
flowchart TD
    subgraph VM_Init ["1. WASM VM Initialization"]
        A["_start()"] --> B["proxy_wasm::set_root_context"]
        B --> C["UpstreamCallRoot::on_vm_start()"]
        C --> D["Fetch Plugin Configuration"]
        D --> E["Base64 Decode String"]
        E --> F["JSON Deserialize into ConfigJwt"]
    end

    subgraph Request_Processing ["2. HTTP Request Processing"]
        G["Incoming HTTP Request"] --> H{"Method == OPTIONS?"}
        H -- "Yes" --> I["send_http_response(204)\nAction::Pause"]
        H -- "No" --> J{"Authorization\nHeader Present?"}
        J -- "No" --> K["send_http_response(401)\nAction::Continue"]
        J -- "Yes" --> L["Extract JWT: splitn(3, '.')"]
        L --> M["Base64 Decode Header & Payload"]
        M --> N["Deserialize into HashMap<String, String>"]
        N --> O["Execute modify_jwt(config)"]
        O --> P["Serialize JSON & Base64 Encode"]
        P --> Q["Trim 2 chars (pop; pop)"]
        Q --> R["Reassemble: header.payload.signature"]
        R --> S["set_http_request_header(Authorization)"]
        S --> T["Append jwt_test to Response Headers"]
        T --> U["send_http_response(200, 'OK\n')\nAction::Pause"]
    end

    subgraph Upstream_Status ["3. Upstream Routing"]
        V["Upstream Service"]
        U -. "Not Forwarded\n(Halted by Action::Pause)" .-> V
    end
```

### Architectural Notes
- **Local Response Generation:** The filter terminates request processing inside `on_http_request_headers` by calling `self.send_http_response(200, headers, Some(b"OK\n"))` and returning `Action::Pause`.
- **Upstream Forwarding Blocked:** Although the filter mutates the downstream request header via `set_http_request_header("Authorization", ...)`, Envoy does not forward the request to upstream clusters. The request is satisfied immediately with an HTTP 200 local reply containing the modified token in the `jwt_test` response header.

---

## 5. Prerequisites

Before building and deploying the filter, ensure the following tools are installed:

- **Rust:** Nightly toolchain (tested with `nightly-x86_64-pc-windows-gnu` on Windows 11; standard `nightly` on Linux/macOS).
- **WASM Target:** `wasm32-unknown-unknown`.
- **Cargo:** Standard package manager bundled with Rust.
- **Docker:** Docker Desktop or Docker Engine with Linux containers enabled.
- **Envoy Runtime:** Envoy proxy with Proxy-WASM v0.1 support (tested using `istio/proxyv2:1.13.8` / Envoy 1.22).
- **curl:** Command-line tool for HTTP testing.

---

## 6. Installation and Build

### Step 1: Install Rust Nightly Toolchain and WASM Target
```powershell
# Windows (PowerShell)
rustup toolchain install nightly-x86_64-pc-windows-gnu
rustup target add wasm32-unknown-unknown --toolchain nightly-x86_64-pc-windows-gnu
```

```bash
# Linux / macOS (Bash)
rustup toolchain install nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
```

### Step 2: Compile the WASM Filter
The filter relies on Proxy-WASM hostcalls provided dynamically by the Envoy host at runtime. The linker flag `-C link-arg=--allow-undefined` is required to prevent linker errors during compilation.

Navigate to the `JWTManipulator9000` directory before compiling:

#### Windows (PowerShell - Verified)
```powershell
cd JWTManipulator9000
$env:RUSTFLAGS = "-C link-arg=--allow-undefined"
cargo +nightly-x86_64-pc-windows-gnu build --release --target wasm32-unknown-unknown
```

#### Linux / macOS (Standard)
```bash
cd JWTManipulator9000
RUSTFLAGS="-C link-arg=--allow-undefined" cargo +nightly build --release --target wasm32-unknown-unknown
```

### Step 3: Verify Compiled Artifact
The compiled binary will be placed at:
```text
target/wasm32-unknown-unknown/release/jwt_filter.wasm
```
- **Observed File Size:** ~302,574 bytes (measured with `rustc 1.102.0-nightly` on `nightly-x86_64-pc-windows-gnu`; exact size will drift with compiler releases and dependency changes).

### Expected Compiler Warnings
During build, Cargo will emit three harmless warnings:
1. `warning: field final_jwt is never read` in `src/lib.rs:109:5` (struct field retained for state derivation).
2. `warning: unused dependency bincode` in `Cargo.toml:13:1`.
3. `warning: unused dependency wasm-bindgen` in `Cargo.toml:17:1`.

---

## 7. Configuration Reference

The filter configuration is defined as a JSON object containing six top-level array fields. All fields are required in the JSON schema:

```json
{
  "add_header": [],
  "del_header": [],
  "add_payload": [],
  "del_payload": [],
  "payload_to_header": [],
  "header_to_payload": []
}
```

### 1. `add_header`
- **Purpose:** Adds or overwrites claims in the JWT header.
- **JSON Type:** Array of 2-element string arrays: `[["key", "value"], ...]`.
- **Example:** `[["kid", "key-2026"], ["env", "dev"]]`
- **Behavior:** Inserts keys into the header map. If a key already exists, its value is updated.

### 2. `del_header`
- **Purpose:** Removes claims from the JWT header.
- **JSON Type:** Array of strings: `["key1", "key2", ...]`.
- **Example:** `["typ"]`
- **Behavior:** Removes keys from the header map. If a specified key does not exist, it is safely ignored without error.

### 3. `add_payload`
- **Purpose:** Adds or overwrites claims in the JWT payload.
- **JSON Type:** Array of 2-element string arrays: `[["key", "value"], ...]`.
- **Example:** `[["tier", "premium"], ["location", "remote"]]`
- **Behavior:** Inserts keys into the payload map. Existing unrelated payload claims are preserved.

### 4. `del_payload`
- **Purpose:** Removes claims from the JWT payload.
- **JSON Type:** Array of strings: `["key1", "key2", ...]`.
- **Example:** `["department"]`
- **Behavior:** Removes keys from the payload map. If a specified key does not exist, it is safely ignored without error.

### 5. `payload_to_header`
- **Purpose:** Moves a claim from the JWT payload into the JWT header.
- **JSON Type:** Array of strings: `["key1", ...]`.
- **Example:** `["environment"]`
- **Behavior:** Copies the claim's value to the header and deletes it from the payload.
- **Edge Case / Failure Mode:** Calls `self.payload.get(i).unwrap()`. If the key does **not** exist in the payload, the filter **panics**, causing Envoy to terminate the request with `503 Service Unavailable`.

### 6. `header_to_payload`
- **Purpose:** Moves a claim from the JWT header into the JWT payload.
- **JSON Type:** Array of strings: `["key1", ...]`.
- **Example:** `["kid"]`
- **Behavior:** Copies the claim's value to the payload and deletes it from the header.
- **Edge Case / Failure Mode:** Calls `self.headers.get(i).unwrap()`. If the key does **not** exist in the header, the filter **panics**, causing Envoy to terminate the request with `503 Service Unavailable`.

---

## 8. Complete Configuration Example

Here is a verified JSON configuration exercising all six operations without trailing commas or syntax errors:

```json
{
  "add_header": [
    ["kid", "key-2026"]
  ],
  "del_header": [
    "typ"
  ],
  "add_payload": [
    ["location", "remote"]
  ],
  "del_payload": [
    "department"
  ],
  "payload_to_header": [
    "environment"
  ],
  "header_to_payload": [
    "kid"
  ]
}
```

### Base64 Encoding the Configuration
Envoy requires WASM configuration strings to be passed as Base64-encoded strings:

#### Windows PowerShell
```powershell
$json = '{"add_header":[["kid","key-2026"]],"del_header":["typ"],"add_payload":[["location","remote"]],"del_payload":["department"],"payload_to_header":["environment"],"header_to_payload":["kid"]}'
[Convert]::ToBase64String([System.Text.Encoding]::UTF8.GetBytes($json))
```

#### Linux / macOS
```bash
# Linux (GNU base64 disables line wrapping with -w 0; on macOS use: base64 | tr -d '\n')
echo -n '{"add_header":[["kid","key-2026"]],"del_header":["typ"],"add_payload":[["location","remote"]],"del_payload":["department"],"payload_to_header":["environment"],"header_to_payload":["kid"]}' | base64 -w 0
```

**Encoded Value:**
```text
eyJhZGRfaGVhZGVyIjpbWyJraWQiLCJrZXktMjAyNiJdXSwiZGVsX2hlYWRlciI6WyJ0eXAiXSwiYWRkX3BheWxvYWQiOltbImxvY2F0aW9uIiwicmVtb3RlIl1dLCJkZWxfcGF5bG9hZCI6WyJkZXBhcnRtZW50Il0sInBheWxvYWRfdG9faGVhZGVyIjpbImVudmlyb25tZW50Il0sImhlYWRlcl90b19wYXlsb2FkIjpbImtpZCJdfQ==
```

---

## 9. Practical Example Use Case

### Scenario: Service Mesh Role and Claim Transformation
During local development and integration testing, microservices often require specific header or payload claims (e.g., routing headers, elevated roles, or tenant tags) without having to spin up a full OpenID Connect (OIDC) identity provider.

### Initial Input JWT
- **Original Decoded Header:**
  ```json
  {
    "alg": "HS256",
    "typ": "JWT"
  }
  ```
- **Original Decoded Payload:**
  ```json
  {
    "sub": "user123",
    "role": "user",
    "department": "engineering",
    "environment": "test"
  }
  ```
- **Dummy Signature:** `test_signature`

### Configured Actions
1. `add_header`: Add `"kid": "key-2026"` to header.
2. `del_header`: Remove `"typ"` from header.
3. `add_payload`: Add `"location": "remote"` to payload.
4. `del_payload`: Remove `"department"` from payload.
5. `payload_to_header`: Move `"environment"` from payload to header.
6. `header_to_payload`: Move `"kid"` from header to payload.

### HTTP Request
```http
GET / HTTP/1.1
Host: localhost:18000
Authorization: eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyMTIzIiwicm9sZSI6InVzZXIiLCJkZXBhcnRtZW50IjoiZW5naW5lZXJpbmciLCJlbnZpcm9ubWVudCI6InRlc3QifQ==.test_signature
```

### HTTP Response
```http
HTTP/1.1 200 OK
powered-by: proxy-wasm
access-control-allow-origin: *
access-control-allow-methods: *
access-control-allow-headers: *
access-control-max-age: 3600
jwt_test: eyJhbGciOiJIUzI1NiIsImVudmlyb25tZW50IjoidGVzdC.eyJsb2NhdGlvbiI6InJlbW90ZSIsImtpZCI6ImtleS0yMDI2Iiwic3ViIjoidXNlcjEyMyIsInJvbGUiOiJ1c2VyIn.test_signature
content-length: 3
content-type: text/plain
x-app-serving: rate-limit-filter
date: Thu, 12 Nov 2026 18:44:38 GMT
server: envoy

OK
```

### Decoded Output Comparison
| Component | Expected Result | Actual Observed Result | Notes |
| :--- | :--- | :--- | :--- |
| **JWT Header** | `{"alg":"HS256","environment":"test"}` | `{"alg":"HS256","environment":"test` (truncated, see note) | `kid` added then moved to payload; `typ` removed; `environment` moved from payload. Trailing `"}` truncated (JSON length 36 is divisible by 3; removing two Base64 characters from unpadded Base64 loses two decoded bytes, stripping `"}`). |
| **JWT Payload** | `{"sub":"user123","role":"user","location":"remote","kid":"key-2026"}` | `{"location":"remote","kid":"key-2026","sub":"user123","role":"user"` (truncated, see note) | `department` removed; `environment` moved to header; `kid` moved from header; `location` added. Trailing `}` truncated (JSON length 68 has `len % 3 == 2`; removing two Base64 characters from single-padded Base64 removes `=` and loses one decoded byte, stripping `}`). |
| **Signature** | `test_signature` | `test_signature` | Original signature preserved unchanged. |

---

## 10. Envoy Integration

To run the filter inside Envoy, configure the HTTP connection manager using Envoy's v3 bootstrap API.

### Envoy Configuration (`envoy.yaml`)
```yaml
static_resources:
  listeners:
  - name: main
    address:
      socket_address:
        address: 0.0.0.0
        port_value: 18000
    filter_chains:
    - filters:
      - name: envoy.filters.network.http_connection_manager
        typed_config:
          "@type": type.googleapis.com/envoy.extensions.filters.network.http_connection_manager.v3.HttpConnectionManager
          stat_prefix: ingress_http
          codec_type: AUTO
          route_config:
            name: local_route
            virtual_hosts:
            - name: local_service
              domains: ["*"]
              routes:
              - match:
                  prefix: "/"
                route:
                  cluster: web_service
          http_filters:
          - name: envoy.filters.http.wasm
            typed_config:
              "@type": type.googleapis.com/envoy.extensions.filters.http.wasm.v3.Wasm
              config:
                name: "jwt_filter"
                root_id: "jwt_filter"
                configuration:
                  "@type": type.googleapis.com/google.protobuf.StringValue
                  value: "eyJhZGRfaGVhZGVyIjpbWyJraWQiLCJrZXktMjAyNiJdXSwiZGVsX2hlYWRlciI6WyJ0eXAiXSwiYWRkX3BheWxvYWQiOltbImxvY2F0aW9uIiwicmVtb3RlIl1dLCJkZWxfcGF5bG9hZCI6WyJkZXBhcnRtZW50Il0sInBheWxvYWRfdG9faGVhZGVyIjpbImVudmlyb25tZW50Il0sImhlYWRlcl90b19wYXlsb2FkIjpbImtpZCJdfQ=="
                vm_config:
                  vm_id: "jwt_filter"
                  runtime: "envoy.wasm.runtime.v8"
                  code:
                    local:
                      filename: "/etc/jwt_filter.wasm"
                  configuration:
                    "@type": type.googleapis.com/google.protobuf.StringValue
                    value: "eyJhZGRfaGVhZGVyIjpbWyJraWQiLCJrZXktMjAyNiJdXSwiZGVsX2hlYWRlciI6WyJ0eXAiXSwiYWRkX3BheWxvYWQiOltbImxvY2F0aW9uIiwicmVtb3RlIl1dLCJkZWxfcGF5bG9hZCI6WyJkZXBhcnRtZW50Il0sInBheWxvYWRfdG9faGVhZGVyIjpbImVudmlyb25tZW50Il0sImhlYWRlcl90b19wYXlsb2FkIjpbImtpZCJdfQ=="
          - name: envoy.filters.http.router
            typed_config:
              "@type": type.googleapis.com/envoy.extensions.filters.http.router.v3.Router
  clusters:
  - name: web_service
    connect_timeout: 0.25s
    type: STATIC
    lb_policy: ROUND_ROBIN
    load_assignment:
      cluster_name: web_service
      endpoints:
      - lb_endpoints:
        - endpoint:
            address:
              socket_address:
                address: 127.0.0.1
                port_value: 8099
admin:
  access_log_path: "/dev/null"
  address:
    socket_address:
      address: 127.0.0.1
      port_value: 18001
```

### Key Configuration Elements
- **WASM Filter Registration:** Registered under `http_filters` using `envoy.filters.http.wasm`.
- **Runtime Engine:** Uses `envoy.wasm.runtime.v8`.
- **WASM Binary:** Mounted into the container at `/etc/jwt_filter.wasm`.
- **Configuration Passing:** Supplied as a `google.protobuf.StringValue` containing the Base64-encoded JSON configuration string.
- **Root Context Initialization:** Passed to both `vm_config.configuration` and `config.configuration` to ensure `on_vm_start` receives the configuration bytes.

---

## 11. Running Locally

You can test the compiled WASM filter locally using Envoy and Docker.

### Step 1: Build the Envoy Container Image
From the repository root (`wasm-filters`):
```bash
docker build -t wasm-filters-envoy:1.13.8 -f envoy.Dockerfile .
```

### Step 2: Save Envoy Configuration in JWTManipulator9000 Directory
Change directory to `JWTManipulator9000/` and save the verified configuration from [Section 10](#10-envoy-integration) as `envoy.yaml`:
```bash
cd JWTManipulator9000
```

### Step 3: Run Envoy with Docker
Run the container from the `JWTManipulator9000/` directory where `envoy.yaml` and `target/wasm32-unknown-unknown/release/jwt_filter.wasm` are located.

> [!NOTE]
> For security, the administrative interface (port `18001`) is bound strictly to `127.0.0.1` so it is not exposed to other hosts on your local network.

#### Windows (PowerShell)
```powershell
docker run --rm -d --name test_jwt_envoy `
  -p 18000:18000 -p 127.0.0.1:18001:18001 `
  -v "${PWD}/envoy.yaml:/etc/envoy.yaml" `
  -v "${PWD}/target/wasm32-unknown-unknown/release/jwt_filter.wasm:/etc/jwt_filter.wasm" `
  wasm-filters-envoy:1.13.8
```

#### Linux / macOS
```bash
docker run --rm -d --name test_jwt_envoy \
  -p 18000:18000 -p 127.0.0.1:18001:18001 \
  -v "$(pwd)/envoy.yaml:/etc/envoy.yaml" \
  -v "$(pwd)/target/wasm32-unknown-unknown/release/jwt_filter.wasm:/etc/jwt_filter.wasm" \
  wasm-filters-envoy:1.13.8
```

### Step 4: View Runtime Logs
```bash
docker logs -f test_jwt_envoy
```

Proceed to [Section 12: Testing](#12-testing) to test requests against the running filter before stopping the container.

---

## 12. Testing

### Verified Test Results

| Test Case | Operation / Scenario | Tested | HTTP Status | Result / Behavior |
| :--- | :--- | :---: | :---: | :--- |
| **A** | `add_header` | Yes | `200 OK` | Key-value pair added to header. Unrelated claims preserved. |
| **B1** | `del_header` (existing) | Yes | `200 OK` | Target claim (`typ`) removed from header. |
| **B2** | `del_header` (nonexistent) | Yes | `200 OK` | Nonexistent claim ignored safely without error. |
| **C** | `add_payload` | Yes | `200 OK` | Multiple claims added to payload. Unrelated claims preserved. |
| **D1** | `del_payload` (existing) | Yes | `200 OK` | Target claim (`department`) removed from payload. |
| **D2** | `del_payload` (nonexistent) | Yes | `200 OK` | Nonexistent claim ignored safely without error. |
| **E1** | `payload_to_header` (existing) | Yes | `200 OK` | Claim moved from payload to header with value intact. |
| **E2** | `payload_to_header` (nonexistent) | Yes | `503 Unavailable` | `.unwrap()` panics on `None`; Envoy returns 503. |
| **F1** | `header_to_payload` (existing) | Yes | `200 OK` | Claim moved from header to payload with value intact. |
| **F2** | `header_to_payload` (nonexistent) | Yes | `503 Unavailable` | `.unwrap()` panics on `None`; Envoy returns 503. |
| **G** | Combined Operations | Yes | `200 OK` | All 6 operations executed in order; signature preserved. |
| **H1** | OPTIONS Request | Yes | `204 No Content` | CORS preflight intercepted directly with CORS headers. |
| **H2** | Missing Authorization | Yes | `401 Unauthorized` | Intercepted with `Unauthorized\n` body. |
| **H3** | Bearer Token Prefix | Yes | `503 Unavailable` | `base64::decode` fails on space; `.unwrap()` panics. |
| **H4** | Non-String JSON Claim | Yes | `503 Unavailable` | Serde fails on integer/bool; `.unwrap()` panics. |
| **H5** | Base64URL Dash/Underscore | Yes | `503 Unavailable` | Standard Base64 fails on `-` or `_`; `.unwrap()` panics. |

### Sending an HTTP Request with curl
```bash
# 1. Prepare raw JWT (Header.Payload.Signature)
JWT="eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyMTIzIiwicm9sZSI6InVzZXIiLCJkZXBhcnRtZW50IjoiZW5naW5lZXJpbmciLCJlbnZpcm9ubWVudCI6InRlc3QifQ==.test_signature"

# 2. Send request to Envoy listener
curl -i -H "Authorization: $JWT" http://localhost:18000/
```

### Inspecting and Decoding the Modified Token
Extract the token from the `jwt_test` header and decode each segment:

```bash
# Extract the payload segment (second segment) and decode:
echo "eyJsb2NhdGlvbiI6InJlbW90ZSIsImtpZCI6ImtleS0yMDI2Iiwic3ViIjoidXNlcjEyMyIsInJvbGUiOiJ1c2VyIn" | base64 -d
```

> [!NOTE]
> Because of the filter's `pop(); pop();` trimming, the number of lost decoded bytes depends on the original JSON byte length:
> - **`len % 3 == 0` (length divisible by 3):** Standard Base64 produces no padding (`0` `=` characters). Removing two Base64 characters strips 12 bits of data, losing **two decoded bytes** (both the closing quote and closing brace: `"`). To repair the JSON, append `"}`.
> - **`len % 3 == 2` (remainder 2):** Standard Base64 produces single `=` padding. Removing two Base64 characters strips the `=` and 6 bits of data, losing **one decoded byte** (the closing brace: `}`). To repair the JSON, append `}`.
> - **`len % 3 == 1` (remainder 1):** Standard Base64 produces double `==` padding. Removing two Base64 characters removes only the two padding characters, losing **zero decoded bytes** and leaving the JSON fully intact.

### Stopping the Container
Once testing is complete, stop the running Envoy container:
```bash
docker stop test_jwt_envoy
```

---

## 13. Known Limitations and Implementation Defects

Testing and source code analysis revealed several architectural limitations and bugs:

1. **Strict String-to-String Mapping:** Claims are typed as `HashMap<String, String>`. Common JWT claims such as numeric timestamps (`exp`, `nbf`, `iat`) or boolean flags will cause serde deserialization to fail and trigger an unwrap panic.
2. **Missing Field Panics on Key Move Operations:** In `payload_to_header` and `header_to_payload`, missing fields are fetched using `.unwrap()`:
   ```rust
   let (key, value) = (i.clone(), self.payload.get(i).unwrap().clone());
   ```
   If a configured field does not exist in the source map, the filter crashes the stream and Envoy returns `503 Service Unavailable`.
3. **Incompatibility with `Bearer ` Prefix:** The filter performs `jwt.splitn(3, ".")` directly on the header value. Passing `Authorization: Bearer <token>` causes `split_jwt[0]` to contain `"Bearer ey..."`, failing Base64 decoding and crashing with a 503 error.
4. **Base64 Dialect Mismatch:** The implementation uses standard Base64 (`base64::decode`) instead of URL-safe Base64 (`URL_SAFE_NO_PAD`). Standard JWT tokens containing `-` or `_` characters will fail decoding.
5. **Base64 Output Truncation Quirk:** The filter executes:
   ```rust
   b64_header.pop();
   b64_header.pop();
   b64_payload.pop();
   b64_payload.pop();
   ```
   This assumes standard Base64 always ends in two padding characters (`==`). When the JSON byte length is divisible by 3 (`len % 3 == 0`), Base64 produces no padding, and popping two characters loses two decoded bytes, stripping the closing quote and closing brace (`"`). When `len % 3 == 2`, Base64 produces one padding character (`=`), and popping two characters removes the `=` and loses one decoded byte, stripping the closing brace (`}`). Only when `len % 3 == 1` does popping two characters strip solely the `==` padding without data loss.
6. **No Upstream Forwarding:** The request handler invokes `self.send_http_response(200, ...)` and returns `Action::Pause`. The modified request is never transmitted to upstream backend services.
7. **No Cryptographic Re-signing:** The original signature segment is appended unmodified to the altered header and payload, producing a cryptographically invalid token.

---

## 14. Troubleshooting

| Issue / Symptom | Root Cause | Solution |
| :--- | :--- | :--- |
| `rust-lld: error: undefined symbol: proxy_...` | Proxy-WASM host calls are external symbols resolved by Envoy at runtime. | Set `$env:RUSTFLAGS = "-C link-arg=--allow-undefined"` (PowerShell) or `export RUSTFLAGS="-C link-arg=--allow-undefined"` (Bash) before compiling. |
| `cargo test` fails with linker error (`collect2: ld returned 1`) | Cargo test attempts to build host native binaries, but `proxy-wasm` hostcall symbols are missing on the host. | WASM filters must be tested inside a Proxy-WASM runtime (such as Envoy) rather than native host `cargo test`. |
| Envoy aborts on startup with `Unable to create Wasm HTTP filter jwt_filter` | The filter configuration contains invalid JSON (e.g. trailing comma) or invalid Base64. | Validate JSON syntax strictly. Do not use trailing commas. Ensure Base64 string is correctly encoded. |
| Envoy returns `503 Service Unavailable` (`wasm_fail_stream`) | The JWT contains a `Bearer ` prefix, non-string claims (integers, booleans), Base64URL characters (`-`, `_`), or a missing key in a move operation. | Pass raw JWT without `Bearer `; ensure all claim values are strings; ensure keys specified in `payload_to_header` and `header_to_payload` exist in the token. |
| Envoy returns `401 Unauthorized` | The HTTP request is missing the `Authorization` header. | Add the `Authorization: <token>` header to the request. |
| Decoded modified JWT is missing closing `"}` or `}` | The filter's `pop(); pop();` truncated data characters because JSON byte length was not `3n + 1`. | Recognized implementation quirk. Depending on segment byte length modulo 3, append `"}` (when `len % 3 == 0`, losing 2 bytes) or `}` (when `len % 3 == 2`, losing 1 byte) when inspecting modified claims. |

---

## 15. Dependencies

Key dependencies configured in [Cargo.toml](Cargo.toml):

- **[proxy-wasm](https://crates.io/crates/proxy-wasm) (`^0.1`):** WebAssembly SDK for Proxy-WASM, providing context traits and hostcall bindings for Envoy.
- **[serde](https://crates.io/crates/serde) (`1.0`) & [serde_json](https://crates.io/crates/serde_json) (`1.0`):** Serialization and deserialization framework for JSON configuration and JWT structures.
- **[base64](https://crates.io/crates/base64) (`0.13.0`):** Base64 encoding and decoding for configuration and token segments.
- **`bincode` & `wasm-bindgen`:** Declared in `Cargo.toml` but unused by the filter implementation.

---

## 16. Contributing and References

- **Layer5 WASM Filters Repository:** [layer5io/wasm-filters](https://github.com/layer5io/wasm-filters)
- **Target Issue:** [#36: [Docs] Improve the docs / Readme on the JWT Manipulator](https://github.com/layer5io/wasm-filters/issues/36)
- **Meshery Project:** [meshery.io](https://meshery.io)
- **Proxy-WASM Rust SDK:** [github.com/proxy-wasm/proxy-wasm-rust-sdk](https://github.com/proxy-wasm/proxy-wasm-rust-sdk)
- **Envoy WASM Filter Documentation:** [envoyproxy.io/docs/envoy/latest/configuration/http/http_filters/wasm_filter](https://www.envoyproxy.io/docs/envoy/latest/configuration/http/http_filters/wasm_filter)
- **Rust Installation Guide:** [rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
