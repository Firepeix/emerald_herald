#!/bin/sh

create_base() {
    create_folder storage
    create_folder storage/log
}

create_folder() {
    if [ ! -d "$1" ]; then
        mkdir $1
    fi
}

echo "Iniciando Pastas"
create_base
echo "Iniciando Servidor"
exec emerald_herald > /storage/log/emerald_herald.log