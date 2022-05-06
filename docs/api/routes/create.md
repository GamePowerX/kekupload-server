# KekUpload API

<br>

<h2><a href="#">POST ~/c/{extension}</a></h2>

Create a new upload stream which you can upload to. See [workflow](../workflow) for more information.


## Params

<details>
<summary>extension</summary>

The extension which will later after uploading be used to be put after the hash in the Content-Disposition header. See [extension](../types/extension) for more information.

</details>

<br>


## Request

<details>
<summary>cURL</summary>

```sh
curl --request POST \
    --data ""
    --url ~/c/{extension}
```
</details>

<br>


## Responses

<details>
<summary>200 - Ok</summary>

```json
{
    "stream": "{stream}"
}
```
</details>

<br>

<details>
<summary>400 - Bad Request</summary>

```json
{
    "generic": "PARAM_LENGTH",
    "field": "EXTENSION",
    "error": "EXTENSION must be in bounds 0-{config::EXTENSION_MAX_LENGTH}"
}
```

</details>

<br>

<details>
<summary>500 - Internal Server Error</summary>

```json
{
    "generic": "FS_CREATE",
    "field": "FILE",
    "error": "Error while creating file: {error}"
}
```

</details>