# DeveloperMayCry

## Overview

DeveloperMayCry (DMC) is a TOML-driven HTTP client designed for Web Application Security Testing and API testing.

It allows you to describe one or more HTTP requests in a single `payload.toml` file and execute them sequentially while preserving sessions, cookies and validation results.

## Features

- HTTP GET / POST / PUT / PATCH / DELETE
- Query parameters
- Custom headers
- Cookies
- JSON or arbitrary request bodies
- Multipart/form-data (fields, multiple files, custom filename, custom content type)
- GraphQL (Query, Mutation, Variables)
- Variable expansion (`{{variable}}`)
- Cookie Jar / Session persistence
- Response validation (status, contains, not_contains)
- Payload validation
- OpenAPI import (servers, paths, path/query/header/cookie parameters)

## Build

```bash
cargo build --release
```

## Usage

```bash
dmc payload.toml
dmc import openapi.json
```

## Payload Format

```toml
[[requests]]
name="Example"
method="GET"
url="http://localhost:8000/"
```

### Variables

```toml
[variables]
host="localhost"
port="8000"

[[requests]]
url="http://{{host}}:{{port}}/"
```

### Query

```toml
[requests.query]
page="1"
search="admin"
```

### Headers

```toml
[requests.headers]
Authorization="Bearer {{token}}"
```

### Cookies

```toml
[requests.cookies]
SESSIONID="{{session}}"
```

### JSON Body

````toml
[[requests]]
method="POST"
url="http://localhost:8000/api"

body = """
{
  "username":"admin",
  "password":"password"
}

[requests.headers]
Content-Type="application/json"


"""
````

### Multipart

```toml
[[requests]]
method="POST"
url="http://localhost:8000/upload"

[multipart.fields]
username="admin"

[multipart.files.payload]
path="shell.php"
filename="image.png"
content_type="image/png"
```

### GraphQL

````toml
[[requests]]
method="POST"
url="http://localhost:8000/graphql"

[requests.graphql]
query = """
query {
    hello
}
"""
````

### GraphQL Variables

````toml
[requests.graphql]
query = """
mutation Login($user: LoginInput!) {
    login(user:$user)
}
"""

[requests.graphql.variables.user]
name="admin"
password="{{password}}"
enabled=true
roles=["admin","dev"]

[requests.graphql.variables]
retry=3
````

### Validation

```toml
[requests.expect]
status=200
contains="DeveloperMayCry"
not_contains="Exception"
```

## OpenAPI Import

```bash
dmc import petstore.json
```

Example output:

```toml
[[requests]]
name="Get User"
method="GET"
url="http://localhost:8000/users/{{id}}"

[requests.query]
page="{{page}}"

[requests.headers]
Authorization="{{Authorization}}"

[requests.cookies]
SESSIONID="{{SESSIONID}}"
```

## Typical Use Cases

- REST API testing
- Authentication testing
- Session testing
- Cookie manipulation
- Header testing
- GraphQL testing
- File upload testing
- OpenAPI-based payload generation

# DeveloperMayCry

DeveloperMayCry (DMC) is a lightweight, TOML-driven HTTP client designed for Web Application Security Testing, API testing, and Proof-of-Concept (PoC) reproduction.

Unlike traditional API clients, DeveloperMayCry focuses on describing HTTP requests as reusable payload files. A single TOML file can contain multiple requests that are executed sequentially while automatically maintaining session state through an internal cookie jar.

The project aims to provide a simple, scriptable, and human-readable way to reproduce vulnerabilities, validate fixes, and automate repetitive web security testing tasks.

DeveloperMayCry is written entirely in Rust, providing a fast, portable, and dependency-light command-line tool suitable for penetration testers, security researchers, developers, and bug bounty hunters.

## Why DeveloperMayCry?

Most API clients are designed for API development.

DeveloperMayCry is designed specifically for security testing.

Instead of manually recreating requests every time, you describe them once in a TOML payload file and execute them whenever needed.

Typical use cases include:

- Reproducing reported vulnerabilities
- Verifying security fixes
- Testing authentication flows
- Uploading files with crafted metadata
- Executing GraphQL queries and mutations
- Testing REST APIs
- Regression testing after code changes
- Importing OpenAPI specifications to generate testing templates

## Philosophy

DeveloperMayCry follows several core principles.

### Human-readable payloads

Payload files should be easy to read, edit, and review without requiring a graphical interface.

### Reproducibility

A vulnerability should be reproducible using a single payload file whenever possible.

### Simplicity

Common testing scenarios should require minimal configuration while still allowing advanced customization.

### Automation Friendly

Payload files are intended to work well with shell scripts, CI/CD pipelines, and automated regression testing.

### Security Research First

The primary goal is to assist security professionals in reproducing and validating web application vulnerabilities rather than replacing full-featured API development tools.

## Current Capabilities

DeveloperMayCry currently supports:

- HTTP GET, POST, PUT, PATCH and DELETE
- Query parameters
- Custom request headers
- Cookie management
- Session persistence
- Variable expansion
- Raw request bodies
- Multipart/form-data
- Multiple file uploads
- Custom filenames
- Custom MIME types
- GraphQL queries
- GraphQL mutations
- Nested GraphQL variables
- Response validation
- Payload validation
- OpenAPI import
- Automatic payload generation

## Features

DeveloperMayCry provides a lightweight yet powerful environment for reproducing and validating web application vulnerabilities.

### HTTP Methods

Supports the most commonly used HTTP methods.

- GET
- POST
- PUT
- PATCH
- DELETE

Each request can be defined independently within a single payload file.

---

### Multiple Requests

A single payload file may contain multiple requests.

Requests are executed sequentially, allowing complex workflows such as:

- Login → Access Profile
- Authenticate → Upload File
- Create Resource → Update Resource → Delete Resource

---

### Query Parameters

Query parameters can be defined separately from the URL.

Example:

```toml
[requests.query]
page = "1"
search = "admin"
sort = "name"
```

DeveloperMayCry automatically generates the final request URL.

---

### Custom HTTP Headers

Any HTTP header can be added.

Example:

```toml
[requests.headers]
Authorization = "Bearer {{token}}"
User-Agent = "DeveloperMayCry"
X-Forwarded-For = "127.0.0.1"
```

This is useful for testing:

- Authentication
- Reverse proxies
- WAF bypasses
- Custom APIs

---

### Cookie Management

Cookies may be specified manually or managed automatically.

Manual cookies:

```toml
[requests.cookies]
SESSIONID = "{{session}}"
```

Automatic cookie handling is performed using an internal cookie jar.

Cookies returned by one request are automatically sent with subsequent requests.

---

### Variable Expansion

Variables can be reused throughout the payload.

Example:

```toml
[variables]
host = "localhost"
port = "8000"

[[requests]]
url = "http://{{host}}:{{port}}/"
```

This makes payloads portable and easy to maintain.

---

### Raw Request Bodies

DeveloperMayCry supports arbitrary request bodies.

Example:

```toml
body = """
{
    "username":"admin",
    "password":"password"
}
"""
```

This can be used for:

- JSON
- XML
- SOAP
- Plain text
- Custom formats

---

### Multipart/Form-Data

Supports multipart requests with both text fields and file uploads.

Features include:

- Multiple files
- Multiple form fields
- Custom filenames
- Custom MIME types

Example:

```toml
[multipart.fields]
username = "admin"

[multipart.files.payload]
path = "shell.php"
filename = "image.png"
content_type = "image/png"
```

This is useful for file upload testing and validation.

---

### GraphQL Support

Native GraphQL support is built in.

Supported features:

- Queries
- Mutations
- Nested variables
- Arrays
- Objects
- Automatic JSON serialization

Example:

```toml
[requests.graphql]

query = """
query {
    hello
}
"""
```

Nested variables are converted into the appropriate JSON structure automatically.

---

### Response Validation

Responses can be validated automatically.

Supported checks:

- HTTP status code
- Response contains text
- Response does not contain text

Example:

```toml
[requests.expect]
status = 200
contains = "Welcome"
not_contains = "Exception"
```

This makes payloads suitable for regression testing.

---

### Payload Validation

Before executing requests, DeveloperMayCry validates the payload file.

Examples of detected issues include:

- Missing URL
- Invalid HTTP version
- Missing request body
- Missing Content-Type
- Invalid configuration

Validation helps detect mistakes before sending requests.

---

### Session Persistence

DeveloperMayCry maintains session state automatically.

Examples:

- Login
- Receive cookies
- Access authenticated endpoints

No manual cookie extraction is required.

---

### OpenAPI Import

DeveloperMayCry can import OpenAPI 3.x specifications.

Current capabilities include:

- Server URLs
- Paths
- HTTP methods
- Path parameters
- Query parameters
- Header parameters
- Cookie parameters

The importer automatically generates a ready-to-use `payload.toml`.

Example:

```bash
dmc import petstore.json
```

---

### Automatic Payload Generation

Imported OpenAPI specifications are converted into executable payload templates.

Example output:

```toml
[[requests]]
name = "Get User"
method = "GET"
url = "http://localhost:8000/users/{{id}}"

[requests.query]
page = "{{page}}"

[requests.headers]
Authorization = "{{Authorization}}"

[requests.cookies]
SESSIONID = "{{SESSIONID}}"
```

This significantly reduces the time required to begin testing an API.

---

### Lightweight CLI

DeveloperMayCry is a standalone command-line application written in Rust.

Benefits include:

- Fast startup
- Low memory usage
- Cross-platform portability
- No graphical interface required
- Easy integration with shell scripts and CI/CD pipelines
