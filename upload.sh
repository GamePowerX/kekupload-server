port="6942"
file="example.txt"
hash="kek"
hash=$(sha1sum example.txt | grep -Eo ^[a-z0-9]+ | head -n 1)

id=$(curl -s --request POST http://127.0.0.1:$port/c/png)
echo $id
result=$(curl -s --request POST http://127.0.0.1:$port/u/$id/$hash --data-raw "$(cat example.txt)")
echo $result

result=$(curl -s --request POST http://127.0.0.1:$port/f/$id/$hash)
echo $result