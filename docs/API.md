# API Documentation

This explains how you can use this api.

<details>
<summary>&emsp;Routes</summary>  
 
[Create upload stream](https://kotw.dev/uploadserver/docs/API#route_create)<br>
[Upload chunk](https://kotw.dev/uploadserver/docs/API#route_upload)<br>
[Finalize file](https://kotw.dev/uploadserver/docs/API#route_finish)<br>
[Remove file](https://kotw.dev/uploadserver/docs/API#route_remove)<br>
[Download file](https://kotw.dev/uploadserver/docs/API#route_download)<br>
[Embed file](https://kotw.dev/uploadserver/docs/API#route_embed)<br>
</details>

<br>

<a name="routes">

## Routes
~ is the base url of the api!

<a name="route_create">

### POST ~/c/\<ext>
This route is used to create a new upload stream for uploading to.

<br>

**Parameters:**<br>

| Name         | Description     | Type |
| -------------- | ----------- | ------------ |
| \<ext> | The file extension      | string (0 < len < 7)        |

<br>

**Result:**<br>
If the request succeeded you will receive a 64 character long string which is the id needed to access your upload stream.

Example:

`POST http://localhost:6942/c/png` → `85bvoJGgos5pD5p4yH3hGV-0GnpxsM6Kr7fRJwf-GGcC-xgfww6XHqFNVUwYH_i1`

</a>

<br>

<hr>

<a name="route_upload">

### POST ~/u/\<id>/\<hash>
This route is used to upload a chunk of data to the upload stream.

<br>

**Parameters:**<br>
| Name         | Description     | Type |
|--------------|-----------|------------|
| \<id> | The upload stream id      | string (len == 64, result of [create](#route_create))        |
| \<hash> | The sha1 hash of the body      | hex digest (lowercase len == 40)        |

<br>

**Body:**<br>
The body size must be under 2mb!

<br>

**Result:**<br>
If the request succeeded you will receive an 'OK'.

Example:

`POST http://localhost:6942/u/85bvoJGgos5pD5p4yH3hGV-0GnpxsM6Kr7fRJwf-GGcC-xgfww6XHqFNVUwYH_i1/539a0aaa37ae283644be2c1b47b8a3e48d8525e0 'Test somes'` → `OK`

</a>

<br>

<hr>

<a name="route_remove">

### POST ~/r/\<id>/
This route is used to close an upload stream. This will delete the tmp file that was generated.

<br>

**Parameters:**<br>
| Name         | Description     | Type |
|--------------|-----------|------------|
| \<id> | The upload stream id      | string (len == 64, result of [create](#route_create))        |

<br>

**Result:**<br>
If the request succeeded you will receive an 'OK'.

Example:

`POST http://localhost:6942/r/85bvoJGgos5pD5p4yH3hGV-0GnpxsM6Kr7fRJwf-GGcC-xgfww6XHqFNVUwYH_i1` → `OK`

</a>

<br>

<hr>

<a name="route_finish">

### POST ~/f/\<id>/\<hash>
This route is used to finalize an upload stream.

<br>

**Parameters:**<br>
| Name         | Description     | Type |
|--------------|-----------|------------|
| \<id> | The upload stream id      | (len == 64, result of [create](#route_create))        |
| \<hash> | The sha1 hash of the entire file      | hex digest (lowercase len == 40)        |

<br>

**Result:**<br>
If the request succeeded you will receive a string with the length 6 which is the final id used to download the file. You can access the file using [download](#route_download)

Example:

`POST http://localhost:6942/f/85bvoJGgos5pD5p4yH3hGV-0GnpxsM6Kr7fRJwf-GGcC-xgfww6XHqFNVUwYH_i1/539a0aaa37ae283644be2c1b47b8a3e48d8525e0` → `hG5D_q`

</a>

<br>

<hr>

<a name="route_download">

### GET ~/d/\<id>/
This route is used to download an uploaded file. The id is the result of [finish](#route_finish)!

<br>

**Parameters:**<br>
| Name         | Description     | Type |
|--------------|-----------|------------|
| \<id> | The file id      | string (len == 6)        |

<br>

**Result:**<br>
If the request succeeds and the id is valid you will receive the uploaded file.

Example:

`GET http://localhost:6942/d/hG5D_q` → `Test somes`

</a>

<br>

<hr>

<a name="route_embed">

### GET ~/e/\<id>/
This route is used to embed an uploaded file on discord or other platforms that support embeding. The id is the result of [finish](#route_finish)!

<br>

**Parameters:**<br>
| Name         | Description     | Type |
|--------------|-----------|------------|
| \<id> | The file id      | string (len == 6)        |

<br>

**Result:**<br>
If the request succeeds and the id is valid you will receive HTML which contains the correct tags to embed the file.

</a>

</a>