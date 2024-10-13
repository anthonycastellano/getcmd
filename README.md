# getcmd
A simple CLI which utilizes OpenAI LLMs to recommend and run shell commands.

### Initial Configuration
Upon running `getcmd` for the first time, a prompt will appear asking for your OpenAI API key. The key will be persisted for future queries in your OS's default configuration folder, for example `~/.config/` on Linux.

### Usage
```
$ getcmd generate a FIPS-compliant RSA keypair for SSH and save it in the current directory
getcmd returned the following command:

ssh-keygen -t rsa -b 2048 -o -a 100 -f ./fips_rsa_key -N ''

run it? (y/n)
y
Generating public/private rsa key pair.
Your identification has been saved in ./fips_rsa_key
Your public key has been saved in ./fips_rsa_key.pub
The key fingerprint is:
SHA256:CLgKanabrQe4KFZR1zPkbdjzmOb0+c3wtqxsp5RX0e0 tony@mypc
The key's randomart image is:
+---[RSA 2048]----+
|        o.       |
|   . . ..++     o|
|  . o .  oo=   .o|
|   o . .  . =  ..|
|. o . . S  = .  E|
|oo o      + . o .|
|+oo..      . =.. |
|=o. +.      o.+=o|
|o  +o.      .++==|
+----[SHA256]-----+
```
- *Note: getcmd will not execute any commands without confirmation from the user*