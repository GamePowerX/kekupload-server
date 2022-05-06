# KekUpload API

<h2><a href="#">Workflow</a></h2>

Workflow of uploading files to KekUpload.

<br>

## Examples

### Upload a text file containing a string

1. Create a new stream using the [create](routes/create) route. <br>
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

2. Upload a chunk using the [upload](routes/upload) route. <br>
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

3. Finalize the stream using the [finish](routes/finish) route. <br>
`// Please change '{stream}' to the value you extracted earlier.` <br>
```sh
curl --request POST \
    --data ""
    --url ~/f/{stream}/efb815e2393a127f19c8caf79f6a5f676aedb62a
```