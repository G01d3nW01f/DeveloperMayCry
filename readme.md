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
