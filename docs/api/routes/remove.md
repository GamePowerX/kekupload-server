# KekUpload API

<br>

<h2><a href="#">POST ~/r/{stream}</a></h2>

Terminate an upload stream. See <a href="../workflow">workflow</a> for more information.


## Params

<details>
<summary>stream</summary>

The stream which you get from the <a href="create">create</a> route. See <a href="../workflow">workflow</a> and <a href="../types/stream">stream</a> for more information.

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
    "generic": "FS_REMOVE",
    "field": "FILE",
    "error": "Error while removing file: {error}"
}
{% endhighlight %}

</details>