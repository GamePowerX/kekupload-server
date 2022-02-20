# Utility for configuring uploadserver
# Written by KekOnTheWorld

cyan="\033[38;2;100;255;200m"
yellow="\033[38;2;255;255;100m"
orange="\033[38;2;249;150;2m"
green="\033[38;2;100;255;100m"
reset="\033[0m"

dotenv=".env"

if [ -f $dotenv ]; then
    echo "Starting server..."
else
printf "" > $dotenv

printf $cyan"\nWelcome to the uploadserver configure script.\n\n"$reset

printf $yellow"Please enter the database host (localhost): "$cyan
read db_host

printf $yellow"Please enter the database port (5432): "$cyan
read db_port

printf $yellow"Please enter the database username (postgres): "$cyan
read db_username

printf $yellow"Please enter the database password (1234): "$cyan
read db_password

printf $yellow"Please enter the database name (upload): "$cyan
read db_database

db_url="postgres://"$db_username":"$db_password"@"$db_host":"$db_port"/"$db_database

printf $green"\nThe database url is set to: "$orange$db_url"\n\n"

echo "DATABASE_URL='$db_url'" >> $dotenv

printf $orange"Running diesel migrations...\n"$reset
diesel migration run || exit -1

printf $green"\nSuccess!\n\n"

# port
printf $yellow"Please enter the port (6942): "$cyan
read port

echo "port='$port'" >> $dotenv

# api_base
printf $yellow"Please enter the api base url (/api/): "$cyan
read api_base

echo "api_base='$api_base'" >> $dotenv

# chunk_size
printf $yellow"Please enter the max chunk size in KiB (2048): "$cyan
read chunk_size

echo "chunk_size='$chunk_size'" >> $dotenv

# upload_dir
printf $yellow"Please enter the upload directory location (upload/): "$cyan
read upload_dir
mkdir -p "$upload_dir"

echo "upload_dir='$upload_dir'" >> $dotenv

# tmp_dir
printf $yellow"Please enter the directory location for temporary files (tmp/): "$cyan
read tmp_dir
mkdir -p "$tmp_dir"

echo "tmp_dir='$tmp_dir'" >> $dotenv


printf $orange"\nFrontend\n\n"

printf $yellow"Please enter the git url from the frontent of your choice or none if you will host it anywhere else (https://github.com/KotwOSS/uploadclient): "$cyan
read frontend

if [ "$frontend" != "none" ]; then
    dir=$(git clone $frontend 2>&1 | grep "Cloning into" | awk '{print $3}' 2>&1 | grep -Po "[a-zA-Z0-9]+")

    cd $dir
    chmod +x configure.sh
    ./configure.sh
    cd ..
    rm -rf $dir

    # web_dir
    echo "web_dir='web/'" >> $dotenv
    # web_host
    echo "web_host='true'" >> $dotenv

    # web_base
    printf $yellow"Please enter the web base (/): "$cyan
    read web_base

    echo "web_base='$web_base'" >> $dotenv
fi


printf $orange"\nEmbeds\n\n"

# embed_description
printf $yellow"Please enter the embed description (File uploaded to UploadServer): "$cyan
read embed_description

echo "embed_description='$embed_description'" >> $dotenv

# embed_color
printf $yellow"Please enter the embed color (#ffffff): "$cyan
read embed_color

echo "embed_color='$embed_color'" >> $dotenv

# download_url
printf $yellow"Please enter the download url (http://localhost:6942/api/d/): "$cyan
read download_url

echo "download_url='$download_url'" >> $dotenv

# embed_route_base
printf $yellow"Please enter the embed route (/): "$cyan
read embed_route_base

echo "embed_route_base='$embed_route_base'" >> $dotenv

# download_route_base
printf $yellow"Please enter the download route (/): "$cyan
read download_route_base

echo "download_route_base='$download_route_base'" >> $dotenv
fi

./uploadserver