#  legalzinho

Uma ferramenta **simples e minimalista** para monitoramento de recursos do sistema Linux — mostrando **uptime**, **uso da CPU** e **memória RAM**, em tempo real.

---

##  Visão Geral

O `topizinho` exibe de forma limpa a porcentagem de uso da **CPU** e da **RAM**, além do **tempo de atividade (uptime)** do sistema.

---

##  Instalação

Clone o repositório e compile o projeto:

```bash
git clone https://github.com/poledna0/topzinho.git
cd topzinho
make
```

### O comando make irá:

  #### Compilar o projeto em modo release (cargo build --release);

  #### Mover o executável para /usr/local/bin/tp;

  #### Permitir que você execute o programa apenas digitando tp no terminal.

---

Se preferir instalar apenas no seu usuário (sem sudo), edite o Makefile e troque:

```bash
BIN_DIR=/usr/local/bin
```

por:

```bash
BIN_DIR=$(HOME)/.local/bin
```
