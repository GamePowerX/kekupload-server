# KekUpload API

<br>

<h2><a href="#">POST ~/c/{extension}</a></h2>

Create a new upload stream which you can upload to. See <a href="../workflow">workflow</a> for more information.


## Params

<details>
<summary>extension</summary>

The extension which will later after uploading be used to be put after the hash in the Content-Disposition header. See <a href="../types/extension">extension</a> for more information.

</details>

<br>


## Request

<details>
<summary>cURL</summary>

{% highlight sh %}
curl --request POST \
    --data ""
    --url ~/c/{extension}
{% endhighlight %}

</details>

<br>


## Responses

<details>
<summary>200 - Ok</summary>

{% highlight json %}
{
    "stream": "{stream}"
}
{% endhighlight %}

</details>

<br>

<details>
<summary>400 - Bad Request</summary>

{% highlight json %}
{
    "generic": "PARAM_LENGTH",
    "field": "EXTENSION",
    "error": "EXTENSION must be in bounds 0-{config::EXTENSION_MAX_LENGTH}"
}
{% endhighlight %}

</details>

<br>

<details>
<summary>500 - Internal Server Error</summary>

{% highlight json %}
{
    "generic": "FS_CREATE",
    "field": "FILE",
    "error": "Error while creating file: {error}"
}
{% endhighlight %}

</details>