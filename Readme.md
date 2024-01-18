

# hyper-tls-server-cam

Just exploring how hyper-tls gets integrated with native-tls.

This might help see what could happen in hyper-tls as far as server types go.

This is also explained in main.rs, but here are some commands to know:

1. run the server `cargo run`

Then to hit server with curl:
```bash
curl https://localhost:12345 -k
# You should see "Hello, World!" on your console.
```

Hit server with wget: (if you don't have curl)
```bash
wget https://localhost:12345 --no-check-certificate -O - -q
# You should see "Hello, World!" on your console.
```

To re-create cert.pem and key.pem:
```bash
openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem -nodes -subj '/CN=localhost'
```