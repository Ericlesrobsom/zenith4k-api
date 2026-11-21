#!/usr/bin/env node
// Gera a lista de rotas conhecidas do painel para validar `/v1/upload`.
//
// Uso:
//   node scripts/extract_paths.mjs <repo-goiptv> [saida.txt]
//
// A saida e embutida no servidor (`include_str!`). Rotas novas do painel podem
// ser liberadas sem recompilar via ZENITH_UPLOAD_PATHS (separadas por virgula).

import fs from "node:fs";
import path from "node:path";

const repo = process.argv[2];
if (!repo) {
  console.error("uso: node scripts/extract_paths.mjs <repo-goiptv> [saida.txt]");
  process.exit(1);
}
const outFile = process.argv[3] || "crates/merger-server/src/panel_paths.txt";

const files = fs
  .readdirSync(repo)
  .filter((f) => f.startsWith("routes") && f.endsWith(".go") && !f.endsWith("_test.go"));

const paths = new Set();
for (const file of files) {
  const text = fs.readFileSync(path.join(repo, file), "utf8");
  const re = /HandleFunc\(\s*"([^"]+)"\s*,\s*([\s\S]*?)\)\s*\r?\n/g;
  let m;
  while ((m = re.exec(text))) {
    if (m[1].startsWith("/")) paths.add(m[1]);
  }
}

const linhas = [...paths].sort();
fs.mkdirSync(path.dirname(outFile), { recursive: true });
fs.writeFileSync(outFile, linhas.join("\n") + "\n");
console.log(`ok: ${linhas.length} rotas -> ${outFile}`);
