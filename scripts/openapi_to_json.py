#!/usr/bin/env python3
"""Gera docs/openapi.json a partir de docs/openapi.yaml.

Uso:
    python scripts/openapi_to_json.py [origem.yaml] [destino.json]

O servidor embute o JSON no binario (`include_str!`) e o serve em
`/openapi.json`. O YAML continua disponivel em `/openapi.yaml`.
"""

import json
import sys

import yaml

origem = sys.argv[1] if len(sys.argv) > 1 else "docs/openapi.yaml"
destino = sys.argv[2] if len(sys.argv) > 2 else "docs/openapi.json"

with open(origem, encoding="utf-8") as arquivo:
    documento = yaml.safe_load(arquivo)

with open(destino, "w", encoding="utf-8", newline="\n") as arquivo:
    json.dump(documento, arquivo, ensure_ascii=False, indent=2)
    arquivo.write("\n")

print(f"ok: {destino} ({len(documento.get('paths', {}))} paths)")
