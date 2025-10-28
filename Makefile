# Caminho do binário destino (pasta do sistema onde ficam executáveis locais)
BIN_DIR=/usr/local/bin
BIN_NAME=tp
BUILD_TARGET=target/release/topizinho

# Regra padrão
all: build install

# Compila o projeto em modo release
build:
	cargo build --release

# Move o binário compilado para /usr/local/bin/tp
install:
	sudo mv $(BUILD_TARGET) $(BIN_DIR)/$(BIN_NAME)
	sudo chmod +x $(BIN_DIR)/$(BIN_NAME)
	@echo "✅ Instalado como $(BIN_DIR)/$(BIN_NAME)"

# Executa o binário (após instalado)
run:
	$(BIN_DIR)/$(BIN_NAME)

# Remove o binário instalado
uninstall:
	sudo rm -f $(BIN_DIR)/$(BIN_NAME)
	@echo "❌ Removido $(BIN_DIR)/$(BIN_NAME)"
