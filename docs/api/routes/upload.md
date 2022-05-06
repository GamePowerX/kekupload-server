# KekUpload API

<br>

<h2><a href="#">POST ~/u/{stream}/{hash}</a></h2>

Terminate an upload stream. See <a href="../workflow">workflow</a> for more information.


## Params

<details>
<summary>stream</summary>

The stream which you get from the <a href="create">create</a> route. See <a href="../workflow">workflow</a> and <a href="../types/stream">stream</a> for more information.

</details>

<br>

<details>
<summary>hash</summary>

The hash of the HTTP body. See <a href="../workflow">workflow</a> and <a href="../types/hash">hash</a> for more information.

</details>

<br>


## Request

<details>
<summary>cURL</summary>

{% highlight sh %}
curl --request POST \
    --data "kekw"
    --url ~/u/{stream}/efb815e2393a127f19c8caf79f6a5f676aedb62a
{% endhighlight %}

</details>

<br>


## Responses

<details>
<summary>200 - Ok</summary>

{% highlight json %}
{
    "success": true
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

{% highlight json %}
{
    "generic": "OVERFLOW",
    "field": "CHUNK",
    "error": "Chunk size exceeded"
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
    "generic": "FS_WRITE",
    "field": "FILE",
    "error": "Error while writing file: {error}"
}
{% endhighlight %}

</details>