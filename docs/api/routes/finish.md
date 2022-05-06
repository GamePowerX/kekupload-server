# KekUpload API

<br>

<h2><a href="#">POST ~/f/{stream}/{hash}</a></h2>

Finalize an upload stream. See <a href="../workflow">workflow</a> for more information.


## Params

<details>
<summary>stream</summary>

The stream which you get from the <a href="create">create</a> route. See <a href="../workflow">workflow</a> and <a href="../types/stream">stream</a> for more information.

</details>

<br>

<details>
<summary>hash</summary>

The hash of all the chunks that have been uploaded using the <a href="upload">upload</a> route. See <a href="../workflow">workflow</a> and <a href="../types/hash">hash</a> for more information.

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