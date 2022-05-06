# KekUpload API

<br>

<h2><a href="#">Extension</a></h2>

An extension is like a file extension but with a few exceptions. See [workflow](../workflow) for more information.


## Notice

* `none`: <br>
  If an extension is named `none`, it won't be appended to the hash in the Content-Disposition header when downloading. 

* `images`: <br>
  If an extension is named like one of the following, special media tags for embedding images will be added to the embed response. <br>
  * `png`
  * `jpg`
  * `jpeg`
  * `ico`
  * `gif`
  * `bmp`
  * `svg`

<br>