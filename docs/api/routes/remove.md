# KekUpload API

<br>

<h2><a href="#">POST ~/r/{stream}</a></h2>

Terminate an upload stream. See [workflow](../workflow) for more information.


## Params

<details>
<summary>stream</summary>

The stream which you get from the [create](create) route. See [workflow](../workflow) and [stream](../types/stream) for more information.

</details>

<br>


## Request

<details>
<summary>cURL</summary>

{% highlight sh %}
curl --request POST \
    --data ""
    --url ~/r/{stream}
{% endhighlight %}

```sh
curl --request POST \
    --data ""
    --url ~/r/{stream}
```

</details>

<br>


## Responses

<details>
<summary>200 - Ok</summary>

```json
{
    "success": true
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
    "generic": "FS_REMOVE",
    "field": "FILE",
    "error": "Error while removing file: {error}"
}
```

</details>