#!/usr/bin/env bash
set -e

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
PYTHON_DIR="$ROOT_DIR/python"
SCHEMAS_DIR="$PYTHON_DIR/schemas"

# Directorios destino en los servicios Python
TARGETS=(
  "$PYTHON_DIR/gateway-api/src/gateway_api/models/"
  "$PYTHON_DIR/report-service/src/report_service/models/"
)

echo "🔄 Limpiando directorios de destino..."
for TARGET in "${TARGETS[@]}"; do
  rm -rf "$TARGET"/*
done

echo "🧩 Generando modelos Pydantic desde schemas..."

for TARGET in "${TARGETS[@]}"; do
  mkdir -p "$TARGET"
  cd $PYTHON_DIR
  echo "📁 Generando modelos en: $TARGET"
  for schema in "$SCHEMAS_DIR"/*.json; do
    name=$(basename "$schema" .json)
    poetry run datamodel-codegen \
  --input "$SCHEMAS_DIR/schema.json" \
  --input-file-type jsonschema \
  --output "$TARGET/models.py" \
  --base-class pydantic.BaseModel \
  --target-python-version 3.11 \
  --disable-timestamp \
  --reuse-model \
  --use-standard-collections \
  --snake-case-field \
  --collapse-root-models 2> /dev/null
  done
done

# Crear __init__.py dinámico
for TARGET in "${TARGETS[@]}"; do
  (
    echo "# Auto-generated imports"
    for model in "$TARGET"/*.py; do
      name=$(basename "$model" .py)
      echo "from .${name} import ${name}"
    done
    echo -e "\n__all__ = ["
    for model in "$TARGET"/*.py; do
      name=$(basename "$model" .py)
      echo "    \"${name}\","
    done
    echo "]"
  ) > "$TARGET/__init__.py"
done

echo "✅ Modelos Pydantic generados correctamente"
