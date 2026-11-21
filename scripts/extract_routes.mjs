#!/usr/bin/env node
// Extrai o catalogo de rotas do painel goiptv a partir do codigo-fonte Go.
//
// Uso:
//   node scripts/extract_routes.mjs <caminho-do-repo-goiptv> [docs/ENDPOINTS.md]
//
// O script le apenas os arquivos `routes*.go` da raiz do repositorio (fonte
// canonica do registro de rotas) e ignora testes, worktrees e `output/`.

import fs from "node:fs";
import path from "node:path";

const repo = process.argv[2];
if (!repo) {
  console.error("uso: node scripts/extract_routes.mjs <repo-goiptv> [saida.md]");
  process.exit(1);
}
const outFile = process.argv[3] || "docs/ENDPOINTS.md";

const files = fs
  .readdirSync(repo)
  .filter((f) => f.startsWith("routes") && f.endsWith(".go") && !f.endsWith("_test.go"))
  .sort();

const rows = [];
for (const file of files) {
  const text = fs.readFileSync(path.join(repo, file), "utf8");
  // Aceita registros de uma linha e registros quebrados em duas linhas
  // (path na primeira, handler na segunda), como em routes_tickets.go.
  const re = /HandleFunc\(\s*"([^"]+)"\s*,\s*([\s\S]*?)\)\s*\r?\n/g;
  let m;
  while ((m = re.exec(text))) {
    if (!m[1].startsWith("/")) continue;
    const line = text.slice(0, m.index).split(/\r?\n/).length;
    rows.push({
      path: m[1],
      expr: m[2].trim().replace(/\s+/g, " "),
      file,
      line,
    });
  }
}

const seen = new Set();
const unique = rows.filter((r) => {
  if (seen.has(r.path)) return false;
  seen.add(r.path);
  return true;
});

const scopeOf = (p) => {
  if (p.startsWith("/merger/api/admin/") || p.startsWith("/api/admin/")) return "admin";
  if (p.startsWith("/merger/api/cliente/")) return "cliente";
  if (p.startsWith("/api/webhook/")) return "jwt";
  if (p.includes("/login") || p.includes("/setup")) return "publico";
  return "sessao";
};

const groups = new Map();
for (const r of unique) {
  const clean = r.path.replace(/^\/merger\/api\//, "").replace(/^\/api\//, "api/");
  const seg = clean.split("/").filter(Boolean)[0] || "(raiz)";
  if (!groups.has(seg)) groups.set(seg, []);
  groups.get(seg).push(r);
}

const esc = (s) => String(s).replace(/\|/g, "\\|");
const lines = [];
lines.push("# Catalogo de rotas do painel");
lines.push("");
lines.push("Gerado automaticamente a partir do codigo-fonte Go do painel.");
lines.push("");
lines.push(`- Repositorio analisado: \`${repo}\``);
lines.push(`- Arquivos: ${files.length} (\`routes*.go\` da raiz)`);
lines.push(`- Rotas registradas: ${rows.length}`);
lines.push(`- Rotas unicas: ${unique.length}`);
lines.push("");
lines.push("> O cliente Rust aceita **qualquer** rota `/merger/api/*` ou");
lines.push("> `/api/webhook/*` pelo passthrough (`MergerClient::raw`). As rotas");
lines.push("> abaixo sao o inventario completo; os atalhos tipados cobrem os");
lines.push("> fluxos principais.");
lines.push("");
lines.push("## Indice");
lines.push("");
for (const [seg, list] of [...groups.entries()].sort((a, b) => b[1].length - a[1].length)) {
  lines.push(`- [${seg}](#${seg.replace(/[^a-z0-9-]/gi, "-").toLowerCase()}) (${list.length})`);
}
lines.push("");

for (const [seg, list] of [...groups.entries()].sort((a, b) => b[1].length - a[1].length)) {
  lines.push(`## ${seg}`);
  lines.push("");
  lines.push("| Rota | Metodo/escopo | Handler | Fonte |");
  lines.push("|---|---|---|---|");
  for (const r of list.sort((a, b) => a.path.localeCompare(b.path))) {
    lines.push(
      `| \`${esc(r.path)}\` | ${scopeOf(r.path)} | \`${esc(r.expr)}\` | ${esc(r.file)}:${r.line} |`
    );
  }
  lines.push("");
}

fs.mkdirSync(path.dirname(outFile), { recursive: true });
fs.writeFileSync(outFile, lines.join("\n"));
console.log(`ok: ${unique.length} rotas unicas -> ${outFile}`);
