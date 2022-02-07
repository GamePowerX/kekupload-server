port="6942"
file="example.txt"
hash=$(md5sum example.txt | grep -Eo ^[a-z0-9]+ | head -n 1)

id=$(curl -s --request POST http://127.0.0.1:$port/c/png)
echo $id
result=$(curl -s --request POST http://127.0.0.1:$port/u/$id/$hash --data-raw '$(cat example.txt)')
echo $result