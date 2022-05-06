# Setup Database

## Creating

1. Create database:
```sql
CREATE DATABASE upload;
```
2. Create user:
```sql
CREATE USER upload 
WITH PASSWORD '<password>';
```
3. Grant privileges:
```sql
GRANT ALL PRIVILEGES ON DATABASE upload TO upload;
```

<br>

## Removing

1. Drop user:
```sql
DROP USER upload;
```

2. Drop database:
```sql
DROP DATABASE upload;
```