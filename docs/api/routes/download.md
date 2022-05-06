# KekUpload API

<br>

<h2><a href="#">GET ~/d/{id}</a></h2>

Download an uploaded file. See <a href="../workflow">workflow</a> for more information.


## Params

<details>
<summary>id</summary>

The id which you get from the <a href="finish">finish</a> route. See <a href="../workflow">workflow</a> for more information.

</details>

<br>


## Request

<details>
<summary>cURL</summary>

{% highlight sh %}
curl --request POST \
    --data ""
    --url ~/d/{id}
{% endhighlight %}

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

{% highlight json %}
{
    "generic": "NOT_FOUND",
    "field": "ID",
    "error": "File with id not found"
}
{% endhighlight %}

</details>

<br>

<details>
<summary>500 - Internal Server Error</summary>

{% highlight json %}
{
    "generic": "FS_OPEN",
    "field": "FILE",
    "error": "Error while opening file: {error}"
}
{% endhighlight %}

{% highlight json %}
{
    "generic": "DB_QUERY",
    "field": "QUERY",
    "error": "Error while selecting files: {error}"
}
{% endhighlight %}

</details>