# Errors

An error object looks like this:
```json
{
    "generic": "NOT_FOUND",
    "field": "STREAM",
    "error": "Stream not found"
}
```

```// Generic errors``` <br>

**404 Not Found:** <br>
* `NOT_FOUND`: The requested resource was not found. <br>

**400 Bad Request:** <br>
* `PARAM_LENGTH`: Length of parameter out of bounds. <br>
* `MISSING`: Missing a required field. <br>
* `HASH_MATCH`: Hash doesn't match. <br>
* `OVERFLOW`: Request body too big. <br>
<br>

```// Internal errors``` <br>

**500 Internal Server Error:**
* `DB_CONNECT`: Database connection errors. <br>
* `DB_QUERY`: Database query errors. <br>
* `FS_CREATE`: Filesystem error while creating a file. <br>
* `FS_RENAME`: Filesystem error while renaming a file. <br>
* `FS_REMOVE`: Filesystem error while removing a file. <br>
* `FS_OPEN`: Filesystem error while opening a file. <br>
* `FS_WRITE`: Filesystem error while writing to a file. <br>
<br>