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

[requests.headers]
Content-Type="application/json"

body = """
{
  "username":"admin",
  "password":"password"
}
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
