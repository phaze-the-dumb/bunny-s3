# BunnyS3

Want to use BunnyCDN's storage with S3? Here's a really janky, duct-tape fix.

## What is this?

This is a local S3 server that converts and proxies all commands to bunny.

## How do I use it?

1. Make a folder for it

```bash
mkdir bunny-s3
cd bunny-s3
```

2. Download bunny-s3 binary

```bash
wget https://github.com/phaze-the-dumb/bunny-s3/releases/download/0.1.0/bunny-s3
```

3. Create your `.env` file

```bash
nano .env
```

Fill in this ENV template with the correct information.

You should securely randomly generate the `S3_CLIENT_KEY_ID` and `S3_CLIENT_SECRET` values and then note them down, you will have to use these to authenticate using your S3 client.

```
ENDPOINT=<STORAGE ZONE REGION>.storage.bunnycdn.com
BUCKET_NAME=<STORAGE ZONE NAME>
API_TOKEN=<READ-WRITE BUNNY TOKEN HERE>

S3_CLIENT_KEY_ID=<RANDOM S3 CLIENT KEY ID>
S3_CLIENT_SECRET=<RANDOM S3 CLIENT SECRET>

PORT=8080
```

For example if I had a storage zone in Germany called `my-very-secret-storage-zone`


```
ENDPOINT=de.storage.bunnycdn.com
BUCKET_NAME=my-very-secret-storage-zone
API_TOKEN=abcdefgh-this-isnt-arealapitoken-ijkl-mnop

S3_CLIENT_KEY_ID=my-secret-key-id
S3_CLIENT_SECRET=my-secret-secret-shhh-dont-tell-anyone

PORT=8080
```

4. Run it

```bash
chmod +x bunny-s3
./bunny-s3
```

You could also set it up as a systemd service:

`sudo nano /etc/systemd/system/bunny-s3.service`

```
[Unit]
Description=Bunny to S3 connector

[Service]
Type=simple
WorkingDirectory=<PATH TO CURRENT DIR>
ExecStart=<PATH TO CURRENT DIR>/bunny-s3
Restart=always

[Install]
WantedBy=multi-user.target
```

`sudo systemctl enable --now bunny-s3`

5. Connect to it using your S3 Client

Point your S3 client to `localhost:8080` (or whatever port you set it to) and set the "key id" and "secret" fields to the values you specified in your ENV files.

**You MUST set your url style to PATH. virtual-host style currently is not implemented**

6. Got any issues?

Feel free to open an issue here, or contact me privately on discord (`_phaz`) or bluesky (`@phaz.uk`)

## Why?

Simple answer: I got bored

Slightly less simple answer: I've seen a couple people asking for S3 support in the bunny discord server, and after a while I realised that I still have some stuff using cloudflare's R2 over S3 and wanted to move it to bunny.