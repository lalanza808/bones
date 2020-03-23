# bones

Simple IRC bot written in Rust as a learning exercise. Uses the [irc](https://docs.rs/irc) crate.


## Notes

```
docker build -t torsocks torsocks
docker run --name torsocks --rm -d -p 9050:9050 torsocks
mkdir -p certs
openssl req -nodes -newkey rsa:2048 -keyout certs/lza_rustbot.pem -x509 -days 3650 -out certs/lza_rustbot.crt -subj "/CN=lza_rustbot"
openssl x509 -sha1 -noout -fingerprint -in certs/lza_rustbot.crt | sed -e 's/^.*=//;s/://g'
docker-compose up -d 
```
