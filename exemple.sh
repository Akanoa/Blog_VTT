read -p "Connection sur le port 8080" -n1 -s
echo
curl -v 127.1:8080
read -p "Connection sur le port 8081" -n1 -s
echo
curl -v 127.1:8081
