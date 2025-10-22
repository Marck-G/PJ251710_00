# ====================================================
# 🌐 Sistema de Gestión Documental (SGD)
# Makefile universal (Linux / Windows / WSL)
# ====================================================

# Variables principales
PYTHON_DIR := python
RUST_DIR := rust
INFRA_DIR := infra

SHELL := /bin/bash
OS := $(shell uname 2>/dev/null || echo Windows)

# =========================
# 🧩 Ayuda
# =========================
.PHONY: help
help:
	@echo ""
	@echo "Comandos principales:"
	@echo "  make setup           → Inicializa todo el entorno (Poetry, Rust)"
	@echo "  make dev             → Compila y levanta el entorno de desarrollo"
	@echo "  make build           → Compila Rust + instala dependencias Python"
	@echo "  make test            → Ejecuta tests en ambos entornos"
	@echo "  make fmt             → Formatea código"
	@echo "  make lint            → Ejecuta linters"
	@echo "  make export-schemas  → Exporta modelos Rust a JSON Schema"
	@echo "  make infra-up        → Levanta base de datos, MQ, etc."
	@echo "  make clean           → Limpieza total"
	@echo ""

# =========================
# ⚙️ Setup multiplataforma
# =========================
.PHONY: setup check-tools rust-check python-install infra-up

setup: check-tools rust-check python-install
	@echo ""
	@echo "✅ Entorno de desarrollo inicializado correctamente."
	@echo "➡️  Ejecuta: make dev"

check-tools:
	@echo "🔍 Comprobando herramientas base..."
ifeq ($(OS),Windows)
	@where cargo >nul 2>nul || (echo "❌ Rust no está instalado. Instálalo desde https://rustup.rs/" && exit 1)
	@where python >nul 2>nul || (echo "❌ Python no está en PATH." && exit 1)
	@where docker >nul 2>nul || (echo "❌ Docker Desktop no está instalado o corriendo." && exit 1)
	@where poetry >nul 2>nul || (echo "🐍 Instalando Poetry..." && pip install poetry)
else
	@if ! command -v cargo >/dev/null; then \
		echo "❌ Rust no está instalado. Instálalo con: curl https://sh.rustup.rs -sSf | sh -s -- -y"; exit 1; \
	fi
	@if ! command -v poetry >/dev/null; then \
		echo "🐍 Instalando Poetry..."; \
		curl -sSL https://install.python-poetry.org | python3 -; \
	else \
		echo "✅ Poetry ya está instalado."; \
	fi
	@if ! command -v docker >/dev/null; then \
		echo "❌ Docker no está instalado o en PATH."; exit 1; \
	fi
endif
	@echo "✅ Herramientas verificadas correctamente."

# =========================
# 🦀 Rust
# =========================
.PHONY: rust-check rust-build rust-test rust-fmt rust-lint

rust-check:
	cd $(RUST_DIR) && cargo check --workspace

rust-build:
	cd $(RUST_DIR) && cargo build --workspace

rust-test:
	cd $(RUST_DIR) && cargo test --workspace

rust-fmt:
	cd $(RUST_DIR) && cargo fmt --all

rust-lint:
	cd $(RUST_DIR) && cargo clippy --workspace -- -D warnings

# =========================
# 🐍 Python
# =========================
.PHONY: python-install python-test python-fmt python-lint

python-install:
ifeq ($(OS),Windows)
	cd $(PYTHON_DIR) && poetry install
else
	cd $(PYTHON_DIR) && poetry install
endif

python-test:
	cd $(PYTHON_DIR) && poetry run pytest -v

python-fmt:
	cd $(PYTHON_DIR) && poetry run black .

python-lint:
	cd $(PYTHON_DIR) && poetry run flake8 .

# =========================
# 🧱 Infraestructura (Docker)
# =========================
.PHONY: infra-up infra-down infra-logs

infra-up:
	cd $(INFRA_DIR) && docker compose up -d

infra-down:
	cd $(INFRA_DIR) && docker compose down

infra-logs:
	cd $(INFRA_DIR) && docker compose logs -f

# =========================
# 📤 Exportación de esquemas
# =========================
.PHONY: export-schemas
export-schemas:
	@echo "🦀 Exportando esquemas JSON Schema desde sgd-common..."
	cd rust/libs/sgd-common && cargo run --example export_schema
	@echo "✅ Esquemas generados en python/schemas/"

# =========================
# 🧪 Comandos combinados
# =========================
.PHONY: build test fmt lint dev clean

build: rust-build python-install
test: rust-test python-test
fmt: rust-fmt python-fmt
lint: rust-lint python-lint

dev: infra-up build
	@echo ""
	@echo "✅ Entorno de desarrollo levantado."
	@echo " - Gateway API: http://localhost:8000"
	@echo " - RabbitMQ UI: http://localhost:15672 (sgd/sgd123)"
	@echo " - PostgreSQL: localhost:5432"

clean:
	@echo "🧹 Limpiando entorno..."
	cd $(INFRA_DIR) && docker compose down -v --remove-orphans
	cd $(RUST_DIR) && cargo clean
	find $(PYTHON_DIR) -type d -name "__pycache__" -exec rm -rf {} + 2>/dev/null || true
