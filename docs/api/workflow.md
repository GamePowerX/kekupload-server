# KekUpload API

<br>

<h2><a href="#">Workflow</a></h2>

Workflow of uploading files to KekUpload.

## Examples

### Upload a text file containing a string

1. Create a new stream using the <a href="routes/create">create</a> route. <br>
```sh
curl --request POST \
    --data ""
    --url ~/c/txt
```
which should give you the following response: <br>
```json
{
    "stream": "{stream}"
}
```
Now you need to extract the stream from the response. (Just copy the {stream})

<br>

1. Upload a chunk using the <a href="routes/upload">upload</a> route. <br>
`// Please change '{stream}' to the value you extracted earlier.` <br>
```sh
curl --request POST \
    --data "kekw"
    --url ~/u/{stream}/efb815e2393a127f19c8caf79f6a5f676aedb62a
```
which should give you the following response: <br>
```json
{
    "success": true
}
```

<br>

1. Finalize the stream using the <a href="routes/finish">finish</a> route. <br>
`// Please change '{stream}' to the value you extracted earlier.` <br>
```sh
curl --request POST \
    --data ""
    --url ~/f/{stream}/efb815e2393a127f19c8caf79f6a5f676aedb62a
```
which should give you the following response: <br>
```json
{
    "id": "{id}"
}
```

<br>

You did it :) Now you can download your file from `~/d/{id}`