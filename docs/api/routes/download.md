# KekUpload API

<h2><a href="#">GET ~/d/{id}</a></h2>

Download an uploaded file. See [workflow](../workflow) for more information.

<br>


## Params

<details>
<summary>id</summary>

The id which you get from the [finish](finish) route. See [workflow](../workflow) for more information.

</details>

<br>


## Request

<details>
<summary>cURL</summary>

```sh
curl --request POST \
    --data ""
    --url ~/d/{id}
```
</details>

<br>


## Responses

<details>
<summary>200 - Ok</summary>

Returns the actual file content.

</details>

<br>

<details>
<summary>404 - Not Found</summary>

```json
{
    "generic": "NOT_FOUND",
    "field": "ID",
    "error": "File with id not found"
}
```

</details>

<br>

<details>
<summary>500 - Internal Server Error</summary>

```json
{
    "generic": "FS_OPEN",
    "field": "FILE",
    "error": "Error while opening file: {error}"
}
```

```json
{
    "generic": "DB_QUERY",
    "field": "QUERY",
    "error": "Error while selecting files: {error}"
}
```

</details>