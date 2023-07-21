# Envy
simple env tools

## install
### from cargo
```
cargo install env-y
```

### from dockerhub
```sh
# example
docker run --rm \
         -v $(PWD)/.env:/.env \
         leafchage/envy encrypt \
         -f /.env \
         -k 2sBel3LDvH0pM2BhTQiF2CS48e4UB3ylaHnt2u/ZPmE=
```


## Commands
### key
generate key to encrypt and decrypt

```bash
envy key
> 2sBel3LDvH0pM2BhTQiF2CS48e4UB3ylaHnt2u/ZPmE=
```

### encrypt | decrypt
```bash
> cat .env
KEY=VALUE
# this is comment
KEY2=VALUE
#%ENCRYPT
KEY3=VALUE

> envy encrypt -f .env -k 2sBel3LDvH0pM2BhTQiF2CS48e4UB3ylaHnt2u/ZPmE= | tee .env.enc
KEY=VALUE
# this is comment
KEY2=VALUE
#%ENCRYPTED(JrZJuruOyn2TVYm2)
KEY3=ZCnANf4kHhkKYSIAjmbycvOyn4Rf

> envy decrypt -f .env.env -k 2sBel3LDvH0pM2BhTQiF2CS48e4UB3ylaHnt2u/ZPmE=
KEY=VALUE
# this is comment
KEY2=VALUE
#%ENCRYPT
KEY3=VALUE
```

### load
```bash
> cat .env1
KEY=VALUE

> cat .env2
KEY2=VALUE2

> cat a.sh
echo $KEY $KEY2

> envy load -f .env1 .env2 --cmd sh a.sh
VALUE VALUE2
```

### json
```bash
> cat .env
KEY=VALUE
# this is comment
KEY2=VALUE
#%ENCRYPT
KEY3=VALUE

> envy json .env
{"KEY3":"VALUE","KEY":"VALUE","KEY2":"VALUE"}
```

