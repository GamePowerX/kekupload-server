# KekUpload API

<br>

<h2><a href="#">POST ~/f/{stream}/{hash}</a></h2>

Finalize an upload stream. See [workflow](../workflow) for more information.


## Params

<details>
<summary>stream</summary>

The stream which you get from the [create](create) route. See [workflow](../workflow) and [stream](../types/stream) for more information.

</details>

<br>

<details>
<summary>hash</summary>

The hash of all the chunks that have been uploaded using the [upload](upload) route. See [workflow](../workflow) and [hash](../types/hash) for more information.

</details>

<br>


## Request

<details>
<summary>cURL</summary>

```sh
curl --request POST \
    --data ""
    --url ~/f/{stream}/{hash}
```
</details>

<br>


## Responses

<details>
<summary>200 - Ok</summary>

```json
{
    "id": "{id}"
}
```

</details>

<br>

<details>
<summary>400 - Bad Request</summary>

```json
{
    "generic": "HASH_MATCH",
    "field": "HASH",
    "error": "Hash doesn't match"
}
```

</details>

<br>

<details>
<summary>404 - Not Found</summary>

```json
{
    "generic": "NOT_FOUND",
    "field": "STREAM",
    "error": "Stream not found"
}
```

</details>

<br>

<details>
<summary>500 - Internal Server Error</summary>

```json
{
    "generic": "FS_RENAME",
    "field": "FILE",
    "error": "Error while renaming file: {error}"
}
```

```json
{
    "generic": "DB_QUERY",
    "field": "QUERY",
    "error": "Error while inserting file: {error}"
}
```

</details>