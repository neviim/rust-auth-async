# Escrevendo uma API basica em Rust

## Objetivo desta api
```doc
GET /users — retorna todos os usuarios
GET /users/{id} — retorna o usuário com um determinado id
POST /users — recebe uma JSON e cria um novo usuário com base nela
DELETE /users/{id} — exclui o usuário com um determinado id
```

## Comando de execução
```zsh
cargo run
```

## Acesso a API
```zsh
curl 127.0.0.1:8080/users
curl -X POST 127.0.0.1:8080/users
```