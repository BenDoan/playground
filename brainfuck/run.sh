cat $1 | bfc > out &&
chmod +x out &&
./out &&
rm out
