#!/usr/bin/env node
// Painel goiptv falso para testes locais do cliente/servidor Rust.
//
// Uso:
//   node scripts/fake-panel.mjs [porta]
//
// Credenciais: teste / teste (franquia) ou admin / admin (admin).
// Nao substitui o painel real: cobre apenas login, whoami, leituras e o
// contrato multipart de tickets para validar o transporte de ponta a ponta.

import http from "node:http";

const port = Number(process.argv[2] || 18999);
const SESSION = "sessao-fake-123";

// Revendas criadas durante o teste (memoria do processo).
const revendas = [
  {
    id: 1,
    username: "revenda1",
    usuario_acesso: "revenda1",
    credits: 10,
    unlimited: 0,
    enabled: 1,
  },
];
let proximoId = 2;

const whoami = {
  username: "teste",
  usuario_acesso: "teste",
  franquia: "teste",
  role: "cliente",
  unlimited_credits: 1,
  vencimento_painel: "2030-01-01",
  tfa_ativo: false,
};

function json(res, status, body) {
  const text = JSON.stringify(body);
  res.writeHead(status, {
    "content-type": "application/json",
    "content-length": Buffer.byteLength(text),
  });
  res.end(text);
}

function readBody(req) {
  return new Promise((resolve) => {
    const chunks = [];
    req.on("data", (c) => chunks.push(c));
    req.on("end", () => resolve(Buffer.concat(chunks).toString("utf8")));
  });
}

function authed(req) {
  return (req.headers.cookie || "").includes(`merger_session=${SESSION}`);
}

const server = http.createServer(async (req, res) => {
  const url = new URL(req.url, `http://127.0.0.1:${port}`);
  const path = url.pathname;
  const method = req.method || "GET";

  if (path === "/merger/api/cliente_login" && method === "POST") {
    const body = JSON.parse((await readBody(req)) || "{}");
    if (body.username === "teste" && body.password === "teste") {
      res.setHeader(
        "set-cookie",
        `merger_session=${SESSION}; Path=/; HttpOnly; SameSite=Strict`
      );
      return json(res, 200, { ok: true });
    }
    return json(res, 200, { ok: false, erro: "Credenciais incorretas" });
  }

  if (path === "/merger/api/login" && method === "POST") {
    const body = JSON.parse((await readBody(req)) || "{}");
    if (body.user === "admin" && body.pass === "admin") {
      res.setHeader(
        "set-cookie",
        `merger_session=${SESSION}; Path=/; HttpOnly; SameSite=Strict`
      );
      return json(res, 200, { ok: true });
    }
    return json(res, 200, { ok: false, erro: "Credenciais incorretas" });
  }

  if (!authed(req)) {
    return json(res, 401, { erro: "sessao_expirada" });
  }

  if (path === "/merger/api/whoami") {
    return json(res, 200, whoami);
  }

  if (path === "/merger/api/stats") {
    return json(res, 200, {
      canais: 10,
      filmes: 20,
      series: 5,
      bloqueados: 0,
      fontes: [],
      servidores: [],
    });
  }

  if (path === "/merger/api/cliente/revendas" && method === "GET") {
    return json(res, 200, revendas);
  }

  if (path === "/merger/api/cliente/revendas" && method === "POST") {
    const body = JSON.parse((await readBody(req)) || "{}");
    if (!body.username || String(body.password || "").length < 4) {
      return json(res, 200, {
        ok: false,
        erro: "Usuário e senha (min 4 chars) obrigatórios",
      });
    }
    if (revendas.some((r) => r.username === body.username)) {
      return json(res, 200, {
        ok: false,
        erro: "Essa identificação de revenda já está em uso",
      });
    }
    const unlimited = Number(body.unlimited) === 1 ? 1 : 0;
    const credits = unlimited ? 0 : Number(body.credits || 0);
    if (!unlimited && credits < 10) {
      return json(res, 200, { ok: false, erro: "Mínimo 10 créditos para criar revenda" });
    }
    const nova = {
      id: proximoId++,
      username: body.username,
      usuario_acesso: body.username,
      credits,
      unlimited,
      enabled: 1,
      pode_criar_revenda: Number(body.pode_criar_revenda ?? 1),
      pode_criar_ilimitado: unlimited ? Number(body.pode_criar_ilimitado || 0) : 0,
      pode_conectar_whatsapp: Number(body.pode_conectar_whatsapp || 0),
    };
    revendas.push(nova);
    return json(res, 200, {
      ok: true,
      id: nova.id,
      username: nova.username,
      usuario_acesso: nova.usuario_acesso,
    });
  }

  if (path === "/merger/api/cliente/revendas" && method === "PATCH") {
    const body = JSON.parse((await readBody(req)) || "{}");
    const alvo = revendas.find((r) => r.id === Number(body.id));
    if (!alvo) return json(res, 200, { ok: false, erro: "Não autorizado" });
    alvo.enabled = Number(body.enabled) === 1 ? 1 : 0;
    return json(res, 200, { ok: true, afetados: 1 });
  }

  if (path === "/merger/api/cliente/revendas" && method === "DELETE") {
    const id = Number(url.searchParams.get("id"));
    const indice = revendas.findIndex((r) => r.id === id);
    if (indice < 0) return json(res, 200, { ok: false, erro: "Não autorizado" });
    revendas.splice(indice, 1);
    return json(res, 200, { ok: true, removidos: 1, linhas_apagadas: 0 });
  }

  if (path === "/merger/api/cliente/revendas" && method === "PUT") {
    const body = JSON.parse((await readBody(req)) || "{}");
    const alvo = revendas.find((r) => r.id === Number(body.id));
    if (!alvo) return json(res, 200, { ok: false, erro: "Não autorizado" });
    if (body.usuario_acesso) alvo.usuario_acesso = body.usuario_acesso;
    return json(res, 200, { ok: true });
  }

  if (path === "/merger/api/cliente/revendas/renovar" && method === "POST") {
    const body = JSON.parse((await readBody(req)) || "{}");
    const alvo = revendas.find((r) => r.id === Number(body.id));
    if (!alvo) return json(res, 200, { ok: false, erro: "Revenda nao encontrada" });
    return json(res, 200, {
      ok: true,
      id: alvo.id,
      username: alvo.username,
      vencimento_painel: "2027-01-20",
      ultima_renovacao_manual: Math.floor(Date.now() / 1000),
    });
  }

  if (path === "/merger/api/cliente/revenda_unlimited" && method === "POST") {
    const body = JSON.parse((await readBody(req)) || "{}");
    const alvo = revendas.find((r) => r.id === Number(body.id));
    if (!alvo) return json(res, 200, { ok: false, erro: "Revenda nao encontrada" });
    alvo.unlimited = Number(body.novo_estado) === 1 ? 1 : 0;
    if (Number(body.novo_estado) === 0) {
      alvo.credits = Number(body.saldo_rebaixamento ?? 0);
    }
    return json(res, 200, {
      ok: true,
      unlimited: alvo.unlimited,
      credits: alvo.credits,
      saldo_aplicado: Number(body.saldo_rebaixamento ?? 0),
    });
  }

  if (path === "/merger/api/cliente/transfer_revenda/destinos" && method === "GET") {
    return json(res, 200, { destinos: revendas.map((r) => r.username) });
  }

  if (path === "/merger/api/cliente/transfer_revenda" && method === "POST") {
    const body = JSON.parse((await readBody(req)) || "{}");
    if (!body.origem || !body.novo_pai) {
      return json(res, 400, { erro: "origem_novo_pai_obrigatorios" });
    }
    return json(res, 200, { ok: true, origem: body.origem, novo_pai: body.novo_pai });
  }

  if (path === "/merger/api/cliente/convite" && method === "GET") {
    return json(res, 200, {
      ok: true,
      codigo: "convite-fake",
      link: "http://127.0.0.1:18999/cadastro/convite-fake",
    });
  }

  if (path === "/merger/api/cliente/convite/regenerar" && method === "POST") {
    return json(res, 200, {
      ok: true,
      codigo: "convite-novo",
      link: "http://127.0.0.1:18999/cadastro/convite-novo",
    });
  }

  if (path === "/merger/api/cliente/convite/adesao/pendentes" && method === "GET") {
    return json(res, 200, {
      ok: true,
      pedidos: [
        {
          pedido_id: "pedido-1",
          mensalista_user: "mensalista1",
          valor_centavos: 10000,
          criado_em: 1700000000,
        },
      ],
    });
  }

  if (path === "/merger/api/cliente/convite/adesao/confirmar" && method === "POST") {
    const body = JSON.parse((await readBody(req)) || "{}");
    if (!body.pedido_id) {
      return json(res, 400, { ok: false, erro: "pedido obrigatório" });
    }
    return json(res, 200, { ok: true, pedido_id: body.pedido_id, status: "CONFIRMADO" });
  }

  if (path === "/merger/api/cliente/convite/mensalista-config") {
    if (method === "GET") {
      return json(res, 200, {
        ok: true,
        config: { habilitado: false, curva: { tarifas_centavos: [] } },
        piso: [500, 400],
        aviso: "",
        apto_novos_cadastros: false,
      });
    }
    const body = JSON.parse((await readBody(req)) || "{}");
    return json(res, 200, { ok: true, config: body });
  }

  if (path === "/merger/api/cliente/sigma_users/estender" && method === "POST") {
    return json(res, 200, {
      ok: false,
      erro: "Trial nao pode ser estendido - use renovar com plano pago",
    });
  }

  if (path === "/merger/api/cliente/tickets/fechar" && method === "POST") {
    const id = Number(url.searchParams.get("id"));
    if (!id) return json(res, 200, { ok: false, erro: "sem_acesso" });
    return json(res, 200, { ok: true });
  }

  // Rota valida no catalogo que simula o catch-all vazio do painel.
  if (path === "/merger/api/cliente/melhorias/listar" && method === "POST") {
    res.writeHead(200, { "content-type": "text/plain" });
    res.end("");
    return;
  }

  if (path === "/merger/api/cliente/revendas/nota" && method === "POST") {
    const body = JSON.parse((await readBody(req)) || "{}");
    const alvo = revendas.find(
      (r) => r.username === body.username || r.id === Number(body.id)
    );
    if (!alvo) return json(res, 200, { ok: false, erro: "forbidden" });
    alvo.vencimento_painel = body.vencimento || alvo.vencimento_painel || "";
    return json(res, 200, { ok: true });
  }

  if (path === "/merger/api/cliente/sigma_users") {
    return json(res, 200, {
      contagens: { ativo: 1, bloqueado: 0, expirado: 0, teste: 0, todos: 1 },
      data: [
        {
          id: 7,
          username: "cliente1",
          status: "ativo",
          exp_date: 1900000000,
        },
      ],
      page: 1,
      pages: 1,
      per_page: 20,
      total: 1,
    });
  }

  if (path === "/merger/api/pedidos/listar") {
    return json(res, 200, {
      ok: true,
      creditos_pedidos: 5,
      pedidos: [
        {
          id: 3,
          tipo: "movie",
          titulo: "Filme Teste",
          status: "pendente",
        },
      ],
    });
  }

  if (path === "/merger/api/pedidos/criar" && method === "POST") {
    const body = JSON.parse((await readBody(req)) || "{}");
    return json(res, 200, { ok: true, id: 4, pedido: body });
  }

  if (
    (path === "/merger/api/cliente/tickets/criar" ||
      path === "/merger/api/cliente/tickets/responder") &&
    method === "POST"
  ) {
    const body = await readBody(req);
    const tipo = (req.headers["content-type"] || "").split(";")[0];
    if (tipo !== "multipart/form-data") {
      return json(res, 400, { ok: false, erro: "upload_invalido" });
    }
    const temAssunto = body.includes('name="assunto"') || body.includes('name="id"');
    return json(res, 200, { ok: true, id: 9, multipart_ok: temAssunto });
  }

  if (path === "/merger/api/aparencia/imagem" && method === "POST") {
    const body = await readBody(req);
    return json(res, 200, {
      ok: true,
      recebido: body.length,
      multipart: (req.headers["content-type"] || "").startsWith("multipart/"),
    });
  }

  return json(res, 404, { ok: false, erro: "rota_nao_encontrada", path });
});

server.listen(port, "127.0.0.1", () => {
  console.log(`fake-panel ouvindo em http://127.0.0.1:${port}`);
});
