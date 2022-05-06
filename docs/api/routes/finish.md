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

{% highlight sh %}
curl --request POST \
    --data ""
    --url ~/f/{stream}/{hash}
{% endhighlight %}

</details>

<br>


## Responses

<details>
<summary>200 - Ok</summary>

{% highlight json %}
{
    "id": "{id}"
}
{% endhighlight %}

</details>

<br>

<details>
<summary>400 - Bad Request</summary>

{% highlight json %}
{
    "generic": "HASH_MATCH",
    "field": "HASH",
    "error": "Hash doesn't match"
}
{% endhighlight %}

</details>

<br>

<details>
<summary>404 - Not Found</summary>

{% highlight json %}
{
    "generic": "NOT_FOUND",
    "field": "STREAM",
    "error": "Stream not found"
}
{% endhighlight %}

</details>

<br>

<details>
<summary>500 - Internal Server Error</summary>

{% highlight json %}
{
    "generic": "FS_RENAME",
    "field": "FILE",
    "error": "Error while renaming file: {error}"
}
{% endhighlight %}

{% highlight json %}
{
    "generic": "DB_QUERY",
    "field": "QUERY",
    "error": "Error while inserting file: {error}"
}
{% endhighlight %}

</details>