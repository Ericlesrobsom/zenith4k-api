# Catalogo de rotas do painel

Gerado automaticamente a partir do codigo-fonte Go do painel.

- Repositorio analisado: `C:\Users\ericl\Desktop\goiptv`
- Arquivos: 33 (`routes*.go` da raiz)
- Rotas registradas: 878
- Rotas unicas: 874

> O cliente Rust aceita **qualquer** rota `/merger/api/*` ou
> `/api/webhook/*` pelo passthrough (`MergerClient::raw`). As rotas
> abaixo sao o inventario completo; os atalhos tipados cobrem os
> fluxos principais.

## Indice

- [cliente](#cliente) (390)
- [admin](#admin) (285)
- [api](#api) (25)
- [lb](#lb) (16)
- [creditos-pacotes](#creditos-pacotes) (15)
- [tmdb](#tmdb) (9)
- [merger](#merger) (7)
- [static](#static) (7)
- [stats](#stats) (5)
- [canal_24h](#canal-24h) (5)
- [pix](#pix) (5)
- [push](#push) (4)
- [login](#login) (4)
- [config](#config) (4)
- [portal](#portal) (4)
- [pedidos](#pedidos) (4)
- [fonte_reserva](#fonte-reserva) (3)
- [loja](#loja) (3)
- [reportes](#reportes) (3)
- [link_reserva](#link-reserva) (2)
- [conexoes](#conexoes) (2)
- [avisos](#avisos) (2)
- [setup](#setup) (2)
- [theme](#theme) (2)
- [aparencia](#aparencia) (2)
- [sigma](#sigma) (2)
- [renovacao_painel](#renovacao-painel) (2)
- [r](#r) (2)
- [webplayer](#webplayer) (2)
- [(raiz)](#-raiz-) (1)
- [apps_ativa](#apps-ativa) (1)
- [available_categories](#available-categories) (1)
- [sync_parcial](#sync-parcial) (1)
- [dashboard_financeiro](#dashboard-financeiro) (1)
- [search](#search) (1)
- [duplicados](#duplicados) (1)
- [bloquear](#bloquear) (1)
- [desbloquear](#desbloquear) (1)
- [bloquear_todos_duplicados](#bloquear-todos-duplicados) (1)
- [desbloquear_todos_duplicados](#desbloquear-todos-duplicados) (1)
- [categorias](#categorias) (1)
- [canais_cat](#canais-cat) (1)
- [bloquear_cat](#bloquear-cat) (1)
- [desbloquear_cat](#desbloquear-cat) (1)
- [editar_icone](#editar-icone) (1)
- [renomear](#renomear) (1)
- [editar_url](#editar-url) (1)
- [episodios](#episodios) (1)
- [original_url](#original-url) (1)
- [server_stats](#server-stats) (1)
- [direto_on](#direto-on) (1)
- [direto_off](#direto-off) (1)
- [24h_on](#24h-on) (1)
- [24h_off](#24h-off) (1)
- [24h_ffmpeg_toggle](#24h-ffmpeg-toggle) (1)
- [canal_viewers](#canal-viewers) (1)
- [proxy_sessions](#proxy-sessions) (1)
- [episode](#episode) (1)
- [disparos](#disparos) (1)
- [manutencao.mp4](#manutencao-mp4) (1)
- [capa_manutencao](#capa-manutencao) (1)
- [manifest.webmanifest](#manifest-webmanifest) (1)
- [sw.js](#sw-js) (1)
- [icons](#icons) (1)
- [apple-touch-icon.png](#apple-touch-icon-png) (1)
- [apple-touch-icon-precomposed.png](#apple-touch-icon-precomposed-png) (1)
- [logout](#logout) (1)
- [cliente_login](#cliente-login) (1)
- [whoami](#whoami) (1)
- [mcp](#mcp) (1)
- [equipe](#equipe) (1)
- [agenda](#agenda) (1)
- [jogos](#jogos) (1)
- [webhook](#webhook) (1)
- [mensalidade](#mensalidade) (1)
- [troca_ok.mp4](#troca-ok-mp4) (1)
- [troca_indisponivel.mp4](#troca-indisponivel-mp4) (1)
- [troca_atualize_lista.mp4](#troca-atualize-lista-mp4) (1)
- [tickets](#tickets) (1)
- [player_api.php](#player-api-php) (1)
- [xmltv.php](#xmltv-php) (1)
- [get.php](#get-php) (1)
- [sigma_stats](#sigma-stats) (1)
- [painel](#painel) (1)
- [metrics](#metrics) (1)

## cliente

| Rota | Metodo/escopo | Handler | Fonte |
|---|---|---|---|
| `/merger/api/cliente/2fa` | cliente | `mergerAPICliente2FAEstado` | routes_merger_core.go:71 |
| `/merger/api/cliente/2fa/confirmar` | cliente | `mergerAPICliente2FAConfirmar` | routes_merger_core.go:73 |
| `/merger/api/cliente/2fa/desativar` | cliente | `mergerAPICliente2FADesativar` | routes_merger_core.go:74 |
| `/merger/api/cliente/2fa/enviar` | cliente | `mergerAPICliente2FAEnviar` | routes_merger_core.go:72 |
| `/merger/api/cliente/agenda/jogos` | cliente | `mergerAPIClienteAgendaJogos` | routes_merger_core.go:186 |
| `/merger/api/cliente/apps_ativa/ativar` | cliente | `mergerClienteAppsAtivaAtivar` | routes_apps_ativa.go:35 |
| `/merger/api/cliente/apps_ativa/comprar_creditos` | cliente | `mergerClienteAppsAtivaComprarCreditos` | routes_apps_ativa.go:40 |
| `/merger/api/cliente/apps_ativa/comprar_status` | cliente | `mergerClienteAppsAtivaComprarStatus` | routes_apps_ativa.go:41 |
| `/merger/api/cliente/apps_ativa/historico` | cliente | `mergerClienteAppsAtivaHistorico` | routes_apps_ativa.go:26 |
| `/merger/api/cliente/apps_ativa/historico_ativacoes` | cliente | `mergerClienteAppsAtivaHistoricoLocal` | routes_apps_ativa.go:27 |
| `/merger/api/cliente/apps_ativa/saldo` | cliente | `mergerClienteAppsAtivaSaldo` | routes_apps_ativa.go:24 |
| `/merger/api/cliente/apps_ativa/transferir_creditos` | cliente | `mergerClienteAppsAtivaTransferir` | routes_apps_ativa.go:25 |
| `/merger/api/cliente/apps_parceiros/catalogo` | cliente | `mergerClienteAppsParceirosCatalogo` | routes_apps_parceiros.go:29 |
| `/merger/api/cliente/apps_parceiros/prefs` | cliente | `mergerClienteAppsParceirosPrefs` | routes_apps_parceiros.go:30 |
| `/merger/api/cliente/avisos/criar` | cliente | `featureGate("avisos_franquia", avisosAPICriarFranquia)` | routes_loja_pwa.go:93 |
| `/merger/api/cliente/avisos/delete` | cliente | `featureGate("avisos_franquia", avisosAPIDeleteFranquia)` | routes_loja_pwa.go:99 |
| `/merger/api/cliente/avisos/leituras` | cliente | `featureGate("avisos_franquia", avisosAPILeiturasFranquia)` | routes_loja_pwa.go:97 |
| `/merger/api/cliente/avisos/listar` | cliente | `featureGate("avisos_franquia", avisosAPIListarFranquia)` | routes_loja_pwa.go:95 |
| `/merger/api/cliente/banner/imagens` | cliente | `mergerAPIClienteBannerImagens` | routes_merger_core.go:194 |
| `/merger/api/cliente/banner/img_proxy` | cliente | `mergerAPIClienteBannerImgProxy` | routes_merger_core.go:191 |
| `/merger/api/cliente/bot_teste` | cliente | `featureGate("bot_teste", mergerAPIClienteBotTesteInfo)` | routes_botteste.go:28 |
| `/merger/api/cliente/bot_teste/config` | cliente | `featureGate("bot_teste", mergerAPIClienteBotTesteConfig)` | routes_botteste.go:32 |
| `/merger/api/cliente/bot_teste/rotacionar` | cliente | `featureGate("bot_teste", mergerAPIClienteBotTesteRotacionar)` | routes_botteste.go:30 |
| `/merger/api/cliente/cat/apagar_manual` | cliente | `featureGate("catalogo_pastas_manuais", mergerAPIClienteCatApagarManual)` | routes_catalogo.go:84 |
| `/merger/api/cliente/cat/canais` | cliente | `mergerAPIClienteCatCanais` | routes_catalogo.go:81 |
| `/merger/api/cliente/cat/criar_manual` | cliente | `featureGate("catalogo_pastas_manuais", mergerAPIClienteCatCriarManual)` | routes_catalogo.go:80 |
| `/merger/api/cliente/cat/desmover_canal` | cliente | `featureGate("catalogo_pastas_manuais", mergerAPIClienteCatDesmoverCanal)` | routes_catalogo.go:83 |
| `/merger/api/cliente/cat/mover_canais` | cliente | `featureGate("catalogo_pastas_manuais", mergerAPIClienteCatMoverCanais)` | routes_catalogo.go:82 |
| `/merger/api/cliente/cat/ordem` | cliente | `mergerAPIClienteCatOrdem` | routes_catalogo.go:42 |
| `/merger/api/cliente/cat/renomear` | cliente | `mergerAPIClienteCatRenomear` | routes_catalogo.go:71 |
| `/merger/api/cliente/cat/unidas` | cliente | `mergerAPIClienteCatUnidas` | routes_catalogo.go:73 |
| `/merger/api/cliente/cat/unir` | cliente | `mergerAPIClienteCatUnir` | routes_catalogo.go:43 |
| `/merger/api/cliente/catalogo/pendencias` | cliente | `mergerAPIClienteCatalogoPendencias` | routes_catalogo.go:32 |
| `/merger/api/cliente/catalogo/pendencias/corrigir` | cliente | `mergerAPIClienteCatalogoPendenciaCorrigir` | routes_catalogo.go:36 |
| `/merger/api/cliente/catalogo/pendencias/opcoes` | cliente | `mergerAPIClienteCatalogoPendenciaOpcoes` | routes_catalogo.go:34 |
| `/merger/api/cliente/catalogo/pendencias/previa` | cliente | `mergerAPIClienteCatalogoPendenciaPrevia` | routes_catalogo.go:35 |
| `/merger/api/cliente/catalogo/pendencias/revisar` | cliente | `mergerAPIClienteCatalogoPendenciaRevisar` | routes_catalogo.go:33 |
| `/merger/api/cliente/cloudflare/apelidos` | cliente | `mergerClienteCloudflareApelidosListar` | routes_cloudflare.go:74 |
| `/merger/api/cliente/cloudflare/disponibilidade` | cliente | `mergerClienteCloudflareDisponibilidade` | routes_cloudflare.go:31 |
| `/merger/api/cliente/cloudflare/records` | cliente | `func(w http.ResponseWriter, r *http.Request) { switch r.Method { case "GET": mergerClienteCloudflareListarRecords(w, r` | routes_cloudflare.go:34 |
| `/merger/api/cliente/cloudflare/servidores_xui` | cliente | `mergerClienteCloudflareListarServidoresXUI` | routes_cloudflare.go:78 |
| `/merger/api/cliente/cloudflare/settings` | cliente | `func(w http.ResponseWriter, r *http.Request) { switch r.Method { case "GET": mergerClienteCloudflareListarSettings(w, r` | routes_cloudflare.go:50 |
| `/merger/api/cliente/cloudflare/tuneis` | cliente | `mergerClienteCloudflareTuneisListar` | routes_cloudflare.go:94 |
| `/merger/api/cliente/cloudflare/zonas` | cliente | `mergerClienteCloudflareZonas` | routes_cloudflare.go:30 |
| `/merger/api/cliente/cobranca` | cliente | `mergerAPIClienteCobranca` | routes_pagamentos.go:31 |
| `/merger/api/cliente/config` | cliente | `mergerAPIClienteConfig` | routes_catalogo.go:44 |
| `/merger/api/cliente/conta` | cliente | `mergerClienteContaHandler` | routes_merger_core.go:67 |
| `/merger/api/cliente/convite` | cliente | `featureGate("revendas_arvore", mergerAPIClienteConvite)` | routes_convite.go:32 |
| `/merger/api/cliente/convite/adesao/confirmar` | cliente | `featureGate("revendas_arvore", conviteAdesaoConfirmarHandler)` | routes_convite.go:29 |
| `/merger/api/cliente/convite/adesao/pendentes` | cliente | `featureGate("revendas_arvore", conviteAdesaoPendentesHandler)` | routes_convite.go:28 |
| `/merger/api/cliente/convite/mensalista-config` | cliente | `featureGate("revendas_arvore", conviteMensalistaConfigHandler)` | routes_convite.go:27 |
| `/merger/api/cliente/convite/regenerar` | cliente | `featureGate("revendas_arvore", mergerAPIClienteConvite)` | routes_convite.go:34 |
| `/merger/api/cliente/cred_estilo` | cliente | `mergerAPIClienteCredEstilo` | routes_revendas.go:69 |
| `/merger/api/cliente/creditos` | cliente | `mergerAPIClienteCreditos` | routes_revendas.go:71 |
| `/merger/api/cliente/criar_linha` | cliente | `featureGate("planos", mergerAPIClienteCriarLinha)` | routes_revendas.go:73 |
| `/merger/api/cliente/csv_export` | cliente | `featureGate("sigma_users", mergerClienteCSVExportHandler)` | routes_sigma.go:70 |
| `/merger/api/cliente/csv_import/cancelar` | cliente | `featureGate("sigma_users", mergerClienteCSVImportCancelarHandler)` | routes_sigma.go:63 |
| `/merger/api/cliente/csv_import/confirmar` | cliente | `featureGate("sigma_users", mergerClienteCSVImportConfirmarHandler)` | routes_sigma.go:61 |
| `/merger/api/cliente/csv_import/preview` | cliente | `featureGate("sigma_users", mergerClienteCSVImportPreviewHandler)` | routes_sigma.go:60 |
| `/merger/api/cliente/csv_import/progresso` | cliente | `featureGate("sigma_users", mergerClienteCSVImportProgressoHandler)` | routes_sigma.go:62 |
| `/merger/api/cliente/cupom_reservas` | cliente | `featureGate("telegram", mergerAPIClienteCupomReservas)` | routes_telegram.go:68 |
| `/merger/api/cliente/cupons_criar` | cliente | `featureGate("telegram", mergerAPIClienteCuponsCriar)` | routes_telegram.go:69 |
| `/merger/api/cliente/cupons_deletar` | cliente | `featureGate("telegram", mergerAPIClienteCuponsDeletar)` | routes_telegram.go:71 |
| `/merger/api/cliente/cupons_editar` | cliente | `featureGate("telegram", mergerAPIClienteCuponsEditar)` | routes_telegram.go:70 |
| `/merger/api/cliente/cupons_listar` | cliente | `featureGate("telegram", mergerAPIClienteCuponsListar)` | routes_telegram.go:66 |
| `/merger/api/cliente/cupons_page` | cliente | `featureGate("telegram", mergerAPIClienteCuponsPage)` | routes_telegram.go:72 |
| `/merger/api/cliente/cupons_stats` | cliente | `featureGate("telegram", mergerAPIClienteCuponsStats)` | routes_telegram.go:67 |
| `/merger/api/cliente/diag/armar` | cliente | `featureGate("diagnostico", diagAPIArmar)` | routes_diag.go:28 |
| `/merger/api/cliente/diag/desarmar` | cliente | `featureGate("diagnostico", diagAPIDesarmar)` | routes_diag.go:29 |
| `/merger/api/cliente/diag/export` | cliente | `featureGate("diagnostico", diagAPIExport)` | routes_diag.go:32 |
| `/merger/api/cliente/diag/geo` | cliente | `featureGate("diagnostico", diagAPIGeo)` | routes_diag.go:33 |
| `/merger/api/cliente/diag/sessao` | cliente | `featureGate("diagnostico", diagAPISessao)` | routes_diag.go:31 |
| `/merger/api/cliente/diag/sessoes` | cliente | `featureGate("diagnostico", diagAPISessoes)` | routes_diag.go:30 |
| `/merger/api/cliente/disparo/destinatarios` | cliente | `featureGate("disparos", disparoAPIDestinatarios)` | routes_disparo.go:31 |
| `/merger/api/cliente/disparo/enviar` | cliente | `featureGate("disparos", disparoAPIEnviar)` | routes_disparo.go:32 |
| `/merger/api/cliente/disparo/optout` | cliente | `featureGate("disparos", disparoAPIOptout)` | routes_disparo.go:33 |
| `/merger/api/cliente/disparo/templates` | cliente | `featureGate("disparos", disparoAPITemplates)` | routes_disparo.go:30 |
| `/merger/api/cliente/dns` | cliente | `featureGate("dns_personalizada", mergerAPIClienteDNS)` | routes_merger_core.go:103 |
| `/merger/api/cliente/eventos` | cliente | `mergerAPIClienteEventos` | routes_merger_core.go:169 |
| `/merger/api/cliente/eventos/stats` | cliente | `mergerAPIClienteEventosStats` | routes_merger_core.go:170 |
| `/merger/api/cliente/eventos/stats_avancadas` | cliente | `mergerAPIClienteEventosStatsAvancadas` | routes_merger_core.go:173 |
| `/merger/api/cliente/eventos/tipos` | cliente | `mergerAPIClienteEventosTipos` | routes_merger_core.go:171 |
| `/merger/api/cliente/formas_pagamento` | cliente | `mergerAPIClienteFormasPagamento` | routes_pagamentos.go:36 |
| `/merger/api/cliente/gateway_config` | cliente | `mergerAPIClienteGatewayConfig` | routes_pagamentos.go:41 |
| `/merger/api/cliente/gateway_testar` | cliente | `mergerAPIClienteGatewayTestar` | routes_pagamentos.go:42 |
| `/merger/api/cliente/gerar_mensagem` | cliente | `featureGate("templates_mensagem", mergerAPIClienteGerarMensagem)` | routes_revendas.go:77 |
| `/merger/api/cliente/historico/cliente` | cliente | `featureGate("historico_clientes", historicoAPICliente)` | routes_historico.go:29 |
| `/merger/api/cliente/historico/clientes` | cliente | `featureGate("historico_clientes", historicoAPIClientes)` | routes_historico.go:28 |
| `/merger/api/cliente/historico/correcao/corrigir` | cliente | `featureGate("historico_clientes", historicoAPICorrecaoCorrigir)` | routes_historico.go:30 |
| `/merger/api/cliente/historico/correcoes` | cliente | `featureGate("historico_clientes", historicoAPICorrecoes)` | routes_historico.go:31 |
| `/merger/api/cliente/historico/exportar` | cliente | `featureGate("historico_clientes", historicoAPIExportar)` | routes_historico.go:32 |
| `/merger/api/cliente/historico/top_clientes` | cliente | `featureGate("historico_clientes", historicoAPITopClientes)` | routes_historico.go:33 |
| `/merger/api/cliente/ia_afiliado` | cliente | `mergerAPIClienteIAAfiliadoHandler` | routes_whatsapp.go:137 |
| `/merger/api/cliente/ia_afiliado/sacar` | cliente | `mergerAPIClienteIAAfiliadoSacarHandler` | routes_whatsapp.go:138 |
| `/merger/api/cliente/ia_afiliado/vincular` | cliente | `mergerAPIClienteIAAfiliadoVincularHandler` | routes_whatsapp.go:139 |
| `/merger/api/cliente/ia_assinaturas` | cliente | `mergerAPIClienteIAAssinaturasMinhaHandler` | routes_whatsapp.go:121 |
| `/merger/api/cliente/ia_assinaturas/cancelar` | cliente | `mergerAPIClienteIAAssinaturasCancelarHandler` | routes_whatsapp.go:122 |
| `/merger/api/cliente/ia_assinaturas/criar` | cliente | `mergerAPIClienteIAAssinaturasCriarHandler` | routes_whatsapp.go:120 |
| `/merger/api/cliente/ia_assinaturas/mudar_plano` | cliente | `mergerAPIClienteIAAssinaturasMudarPlanoHandler` | routes_whatsapp.go:123 |
| `/merger/api/cliente/ia_assinaturas/planos` | cliente | `mergerAPIClienteIAAssinaturasPlanosHandler` | routes_whatsapp.go:119 |
| `/merger/api/cliente/ia_chave_restaurar` | cliente | `mergerAPIClienteIAChaveRestaurarHandler` | routes_whatsapp.go:98 |
| `/merger/api/cliente/ia_config` | cliente | `mergerAPIClienteIAConfigHandler` | routes_whatsapp.go:87 |
| `/merger/api/cliente/ia_cupom_validar` | cliente | `mergerAPIClienteIACupomValidarHandler` | routes_whatsapp.go:130 |
| `/merger/api/cliente/ia_fallback_config` | cliente | `mergerAPIClienteIAFallbackConfigHandler` | routes_whatsapp.go:129 |
| `/merger/api/cliente/ia_modelos_disponiveis` | cliente | `mergerAPIClienteIAModelosHandler` | routes_whatsapp.go:99 |
| `/merger/api/cliente/ia_pacote_comprar` | cliente | `mergerAPIClienteIAPacoteComprarHandler` | routes_whatsapp.go:104 |
| `/merger/api/cliente/ia_pacote_historico` | cliente | `mergerAPIClienteIAPacoteHistoricoHandler` | routes_whatsapp.go:106 |
| `/merger/api/cliente/ia_pacote_status` | cliente | `mergerAPIClienteIAPacoteStatusHandler` | routes_whatsapp.go:105 |
| `/merger/api/cliente/ia_pacotes_disponiveis` | cliente | `mergerAPIClienteIAPacotesHandler` | routes_whatsapp.go:103 |
| `/merger/api/cliente/ia_testar` | cliente | `mergerAPIClienteIATestarHandler` | routes_whatsapp.go:88 |
| `/merger/api/cliente/ia_uso` | cliente | `mergerAPIClienteIAUsoHandler` | routes_whatsapp.go:90 |
| `/merger/api/cliente/ia_uso_historico` | cliente | `mergerAPIClienteIAUsoHistoricoHandler` | routes_whatsapp.go:100 |
| `/merger/api/cliente/integracoes/html_template` | cliente | `featureGate("integracoes_webhook", integracoesHtmlTesterHandler)` | routes_webhook.go:67 |
| `/merger/api/cliente/integracoes/ips` | cliente | `featureGate("integracoes_webhook", webhookTokenIPsListarHandler)` | routes_webhook.go:73 |
| `/merger/api/cliente/integracoes/ips/autorizar` | cliente | `featureGate("integracoes_webhook", webhookTokenIPsAcaoHandler("autorizar"))` | routes_webhook.go:74 |
| `/merger/api/cliente/integracoes/ips/bloquear` | cliente | `featureGate("integracoes_webhook", webhookTokenIPsAcaoHandler("bloquear"))` | routes_webhook.go:75 |
| `/merger/api/cliente/integracoes/ips/deletar` | cliente | `featureGate("integracoes_webhook", webhookTokenIPsAcaoHandler("deletar"))` | routes_webhook.go:76 |
| `/merger/api/cliente/integracoes/postman` | cliente | `featureGate("integracoes_webhook", integracoesPostmanHandler)` | routes_webhook.go:66 |
| `/merger/api/cliente/ip/consultar` | cliente | `mergerAPIClienteIPConsultar` | routes_asn.go:55 |
| `/merger/api/cliente/jwt` | cliente | `mergerAPIClienteJWT` | routes_merger_core.go:101 |
| `/merger/api/cliente/lb_stats` | cliente | `featureGate("loadbalancers", mergerClienteLBStatsHandler)` | routes_catalogo.go:39 |
| `/merger/api/cliente/lixeira` | cliente | `featureGate("lixeira_clientes", mergerClienteLixeiraListar)` | routes_sigma.go:56 |
| `/merger/api/cliente/lixeira/restaurar` | cliente | `featureGate("lixeira_clientes", mergerClienteLixeiraRestaurar)` | routes_sigma.go:57 |
| `/merger/api/cliente/loja_desconto_auto` | cliente | `mergerAPIClienteLojaDescontoAuto` | routes_loja_pwa.go:59 |
| `/merger/api/cliente/loja_dispositivos` | cliente | `mergerAPIClienteLojaDispositivos` | routes_loja_pwa.go:54 |
| `/merger/api/cliente/loja_dispositivos/enviar` | cliente | `mergerAPIClienteLojaEnviarPush` | routes_loja_pwa.go:55 |
| `/merger/api/cliente/loja_popup` | cliente | `mergerAPIClienteLojaPopup` | routes_loja_pwa.go:57 |
| `/merger/api/cliente/maxplayer_link/deletar_wrapper` | cliente | `featureGate("maxplayer", mergerClienteMaxplayerDeletarWrapperHandler)` | routes_sigma.go:120 |
| `/merger/api/cliente/maxplayer_link/info` | cliente | `featureGate("maxplayer", mergerClienteMaxplayerArvoreInfoHandler)` | routes_sigma.go:119 |
| `/merger/api/cliente/maxplayer_link/sincronizar` | cliente | `featureGate("maxplayer", mergerClienteMaxplayerSincronizarHandler)` | routes_sigma.go:118 |
| `/merger/api/cliente/melhorias/confirmar` | cliente | `featureGate("melhorias", melhoriasAPIConfirmar)` | routes_melhorias.go:17 |
| `/merger/api/cliente/melhorias/lida` | cliente | `featureGate("melhorias", melhoriasAPILida)` | routes_melhorias.go:20 |
| `/merger/api/cliente/melhorias/listar` | cliente | `featureGate("melhorias", melhoriasAPIListarCliente)` | routes_melhorias.go:18 |
| `/merger/api/cliente/melhorias/preparar` | cliente | `featureGate("melhorias", melhoriasAPIPreparar)` | routes_melhorias.go:15 |
| `/merger/api/cliente/melhorias/rascunhos` | cliente | `featureGate("melhorias", melhoriasAPIRascunhos)` | routes_melhorias.go:16 |
| `/merger/api/cliente/melhorias/ver` | cliente | `featureGate("melhorias", melhoriasAPIVerCliente)` | routes_melhorias.go:19 |
| `/merger/api/cliente/mensalidade/comprovante` | cliente | `featureGate("mensalista", mensalidadeComprovanteUpload)` | routes_pagamentos.go:93 |
| `/merger/api/cliente/mensalidade/comprovante-ver` | cliente | `featureGate("mensalista", mensalidadeComprovanteServir)` | routes_pagamentos.go:94 |
| `/merger/api/cliente/mensalidade/confirmar` | cliente | `featureGate("mensalista", mensalidadeConfirmarManual)` | routes_pagamentos.go:91 |
| `/merger/api/cliente/mensalidade/gerar` | cliente | `featureGate("mensalista", mensalidadeGerar)` | routes_pagamentos.go:88 |
| `/merger/api/cliente/mensalidade/pendentes` | cliente | `featureGate("mensalista", mensalidadePendentesCobrador)` | routes_pagamentos.go:90 |
| `/merger/api/cliente/mensalidade/rejeitar` | cliente | `featureGate("mensalista", mensalidadeRejeitar)` | routes_pagamentos.go:92 |
| `/merger/api/cliente/mensalidade/status` | cliente | `featureGate("mensalista", mensalidadeStatus)` | routes_pagamentos.go:89 |
| `/merger/api/cliente/mensalista/converter` | cliente | `featureGate("mensalista", mensalistaConverter)` | routes_pagamentos.go:86 |
| `/merger/api/cliente/mensalista/criar` | cliente | `featureGate("mensalista", mensalistaCriar)` | routes_pagamentos.go:83 |
| `/merger/api/cliente/mensalista/editar` | cliente | `featureGate("mensalista", mensalistaEditar)` | routes_pagamentos.go:84 |
| `/merger/api/cliente/mensalista/listar` | cliente | `featureGate("mensalista", mensalistaListar)` | routes_pagamentos.go:87 |
| `/merger/api/cliente/mensalista/rebaixar` | cliente | `featureGate("mensalista", mensalistaRebaixar)` | routes_pagamentos.go:85 |
| `/merger/api/cliente/migracao_texto/preview` | cliente | `featureGate("sigma_users", mergerClienteMigracaoTextoPreviewHandler)` | routes_sigma.go:67 |
| `/merger/api/cliente/msproapp_conta/` | cliente | `featureGate("msproapp", mergerAPIClienteMsproappContaOps)` | routes_msproapp.go:35 |
| `/merger/api/cliente/msproapp_contas` | cliente | `featureGate("msproapp", mergerAPIClienteMsproappContas)` | routes_msproapp.go:30 |
| `/merger/api/cliente/msproapp/aplicativos` | cliente | `featureGate("msproapp", mergerAPIClienteMsproappApps)` | routes_msproapp.go:54 |
| `/merger/api/cliente/msproapp/backup_auto` | cliente | `featureGate("msproapp", mergerAPIClienteMsproappBackupAuto)` | routes_msproapp.go:113 |
| `/merger/api/cliente/msproapp/backup_auto/` | cliente | `featureGate("msproapp", mergerAPIClienteMsproappBackupAuto)` | routes_msproapp.go:115 |
| `/merger/api/cliente/msproapp/backup/baixar` | cliente | `featureGate("msproapp", mergerAPIClienteMsproappBackupBaixar)` | routes_msproapp.go:100 |
| `/merger/api/cliente/msproapp/backup/exportar` | cliente | `featureGate("msproapp", mergerAPIClienteMsproappBackupExportar)` | routes_msproapp.go:102 |
| `/merger/api/cliente/msproapp/backup/importar` | cliente | `featureGate("msproapp", mergerAPIClienteMsproappBackupImportar)` | routes_msproapp.go:106 |
| `/merger/api/cliente/msproapp/backup/parar` | cliente | `featureGate("msproapp", mergerAPIClienteMsproappBackupParar)` | routes_msproapp.go:110 |
| `/merger/api/cliente/msproapp/backup/previa` | cliente | `featureGate("msproapp", mergerAPIClienteMsproappBackupPrevia)` | routes_msproapp.go:104 |
| `/merger/api/cliente/msproapp/backup/status` | cliente | `featureGate("msproapp", mergerAPIClienteMsproappBackupStatus)` | routes_msproapp.go:108 |
| `/merger/api/cliente/msproapp/backups` | cliente | `featureGate("msproapp", mergerAPIClienteMsproappBackups)` | routes_msproapp.go:98 |
| `/merger/api/cliente/msproapp/cliente` | cliente | `featureGate("msproapp", mergerAPIClienteMsproappCliente)` | routes_msproapp.go:48 |
| `/merger/api/cliente/msproapp/clientes` | cliente | `featureGate("msproapp", mergerAPIClienteMsproappClientes)` | routes_msproapp.go:42 |
| `/merger/api/cliente/msproapp/clientes/atribuir` | cliente | `featureGate("msproapp", mergerAPIClienteMsproappAtribuir)` | routes_msproapp.go:44 |
| `/merger/api/cliente/msproapp/clientes/operacoes` | cliente | `featureGate("msproapp", mergerAPIClienteMsproappOperacoes)` | routes_msproapp.go:46 |
| `/merger/api/cliente/msproapp/dns/parar` | cliente | `featureGate("msproapp", mergerAPIClienteMsproappDNSParar)` | routes_msproapp.go:82 |
| `/merger/api/cliente/msproapp/dns/previa` | cliente | `featureGate("msproapp", mergerAPIClienteMsproappDNSPrevia)` | routes_msproapp.go:76 |
| `/merger/api/cliente/msproapp/dns/status` | cliente | `featureGate("msproapp", mergerAPIClienteMsproappDNSStatus)` | routes_msproapp.go:80 |
| `/merger/api/cliente/msproapp/dns/trocar` | cliente | `featureGate("msproapp", mergerAPIClienteMsproappDNSTrocar)` | routes_msproapp.go:78 |
| `/merger/api/cliente/msproapp/expirados` | cliente | `featureGate("msproapp", mergerAPIClienteMsproappExpirados)` | routes_msproapp.go:52 |
| `/merger/api/cliente/msproapp/icone` | cliente | `featureGate("msproapp", mergerAPIClienteMsproappIcone)` | routes_msproapp.go:56 |
| `/merger/api/cliente/msproapp/massa` | cliente | `featureGate("msproapp", mergerAPIClienteMsproappMassa)` | routes_msproapp.go:50 |
| `/merger/api/cliente/msproapp/migrar` | cliente | `featureGate("msproapp", mergerAPIClienteMsproappMigrar)` | routes_msproapp.go:90 |
| `/merger/api/cliente/msproapp/migrar/parar` | cliente | `featureGate("msproapp", mergerAPIClienteMsproappMigrarParar)` | routes_msproapp.go:94 |
| `/merger/api/cliente/msproapp/migrar/previa` | cliente | `featureGate("msproapp", mergerAPIClienteMsproappMigrarPrevia)` | routes_msproapp.go:88 |
| `/merger/api/cliente/msproapp/migrar/status` | cliente | `featureGate("msproapp", mergerAPIClienteMsproappMigrarStatus)` | routes_msproapp.go:92 |
| `/merger/api/cliente/msproapp/painel_clientes` | cliente | `featureGate("msproapp", mergerAPIClienteMsproappPainelClientes)` | routes_msproapp.go:68 |
| `/merger/api/cliente/msproapp/recuperar` | cliente | `featureGate("msproapp", mergerAPIClienteMsproappRecuperar)` | routes_msproapp.go:117 |
| `/merger/api/cliente/msproapp/recuperar/` | cliente | `featureGate("msproapp", mergerAPIClienteMsproappRecuperar)` | routes_msproapp.go:119 |
| `/merger/api/cliente/msproapp/servidores` | cliente | `featureGate("msproapp", mergerAPIClienteMsproappServidores)` | routes_msproapp.go:74 |
| `/merger/api/cliente/notas` | cliente | `mergerAPIClienteNotasListaHandler` | routes_whatsapp.go:145 |
| `/merger/api/cliente/notas/concluir` | cliente | `mergerAPIClienteNotasConcluirHandler` | routes_whatsapp.go:148 |
| `/merger/api/cliente/notas/config` | cliente | `mergerAPIClienteNotasConfigHandler` | routes_whatsapp.go:150 |
| `/merger/api/cliente/notas/criar` | cliente | `mergerAPIClienteNotasCriarHandler` | routes_whatsapp.go:146 |
| `/merger/api/cliente/notas/deletar` | cliente | `mergerAPIClienteNotasDeletarHandler` | routes_whatsapp.go:149 |
| `/merger/api/cliente/notas/editar` | cliente | `mergerAPIClienteNotasEditarHandler` | routes_whatsapp.go:147 |
| `/merger/api/cliente/notas/testar` | cliente | `mergerAPIClienteNotasTestarHandler` | routes_whatsapp.go:151 |
| `/merger/api/cliente/oficial/cliente_info` | cliente | `featureGate("atendimento_oficial", oficialAPIClienteInfo)` | routes_oficial.go:67 |
| `/merger/api/cliente/oficial/conversas` | cliente | `featureGate("atendimento_oficial", oficialAPIConversas)` | routes_oficial.go:56 |
| `/merger/api/cliente/oficial/destinatarios` | cliente | `featureGate("atendimento_oficial", oficialAPIDestinatarios)` | routes_oficial.go:64 |
| `/merger/api/cliente/oficial/enviar` | cliente | `featureGate("atendimento_oficial", oficialAPIEnviar)` | routes_oficial.go:62 |
| `/merger/api/cliente/oficial/enviar_midia` | cliente | `featureGate("atendimento_oficial", oficialAPIEnviarMidia)` | routes_oficial.go:60 |
| `/merger/api/cliente/oficial/etiquetas` | cliente | `featureGate("atendimento_oficial", oficialAPIEtiquetasListar)` | routes_oficial.go:79 |
| `/merger/api/cliente/oficial/etiquetas/atribuir` | cliente | `featureGate("atendimento_oficial", oficialAPIEtiquetasAtribuir)` | routes_oficial.go:83 |
| `/merger/api/cliente/oficial/etiquetas/conversa` | cliente | `featureGate("atendimento_oficial", oficialAPIEtiquetasConversa)` | routes_oficial.go:85 |
| `/merger/api/cliente/oficial/etiquetas/criar` | cliente | `featureGate("atendimento_oficial", oficialAPIEtiquetasCriar)` | routes_oficial.go:80 |
| `/merger/api/cliente/oficial/etiquetas/deletar` | cliente | `featureGate("atendimento_oficial", oficialAPIEtiquetasDeletar)` | routes_oficial.go:82 |
| `/merger/api/cliente/oficial/etiquetas/desatribuir` | cliente | `featureGate("atendimento_oficial", oficialAPIEtiquetasDesatribuir)` | routes_oficial.go:84 |
| `/merger/api/cliente/oficial/etiquetas/editar` | cliente | `featureGate("atendimento_oficial", oficialAPIEtiquetasEditar)` | routes_oficial.go:81 |
| `/merger/api/cliente/oficial/marcar_lida` | cliente | `featureGate("atendimento_oficial", oficialAPIMarcarLida)` | routes_oficial.go:59 |
| `/merger/api/cliente/oficial/mensagens` | cliente | `featureGate("atendimento_oficial", oficialAPIMensagens)` | routes_oficial.go:57 |
| `/merger/api/cliente/oficial/midia` | cliente | `featureGate("atendimento_oficial", oficialServirMidiaHandler)` | routes_oficial.go:65 |
| `/merger/api/cliente/oficial/midia/redownload` | cliente | `featureGate("atendimento_oficial", oficialMidiaRedownloadHandler)` | routes_oficial.go:66 |
| `/merger/api/cliente/oficial/recarga/pix` | cliente | `featureGate("atendimento_oficial", mergerClienteOficialRecargaPix)` | routes_oficial.go:69 |
| `/merger/api/cliente/oficial/recarga/status` | cliente | `featureGate("atendimento_oficial", mergerClienteOficialRecargaStatus)` | routes_oficial.go:70 |
| `/merger/api/cliente/oficial/responder` | cliente | `featureGate("atendimento_oficial", oficialAPIResponder)` | routes_oficial.go:58 |
| `/merger/api/cliente/oficial/respostas_rapidas` | cliente | `featureGate("atendimento_oficial", oficialAPIRRListar)` | routes_oficial.go:73 |
| `/merger/api/cliente/oficial/respostas_rapidas/criar` | cliente | `featureGate("atendimento_oficial", oficialAPIRRCriar)` | routes_oficial.go:74 |
| `/merger/api/cliente/oficial/respostas_rapidas/deletar` | cliente | `featureGate("atendimento_oficial", oficialAPIRRDeletar)` | routes_oficial.go:75 |
| `/merger/api/cliente/oficial/respostas_rapidas/enviar` | cliente | `featureGate("atendimento_oficial", oficialAPIRREnviar)` | routes_oficial.go:76 |
| `/merger/api/cliente/oficial/saldo` | cliente | `featureGate("atendimento_oficial", oficialAPISaldo)` | routes_oficial.go:61 |
| `/merger/api/cliente/oficial/templates` | cliente | `featureGate("atendimento_oficial", oficialAPITemplates)` | routes_oficial.go:63 |
| `/merger/api/cliente/pagamento_config` | cliente | `mergerAPIClientePagamentoConfig` | routes_pagamentos.go:32 |
| `/merger/api/cliente/pagamento_recebedor` | cliente | `mergerAPIClientePagamentoRecebedor` | routes_pagamentos.go:33 |
| `/merger/api/cliente/pagamentos_recebidos` | cliente | `featureGate("pagamentos_recebidos", mergerAPIClientePagamentosRecebidos)` | routes_pagamentos.go:37 |
| `/merger/api/cliente/planos` | cliente | `featureGate("planos", mergerAPIClientePlanos)` | routes_revendas.go:62 |
| `/merger/api/cliente/planos/creditos` | cliente | `featureGate("planos", mergerAPIClientePlanosCreditos)` | routes_revendas.go:66 |
| `/merger/api/cliente/planos/ocultar` | cliente | `featureGate("planos", mergerAPIClientePlanoOcultar)` | routes_revendas.go:64 |
| `/merger/api/cliente/planos/toggle` | cliente | `featureGate("planos", mergerAPIClientePlanosToggle)` | routes_revendas.go:63 |
| `/merger/api/cliente/recebimento` | cliente | `featureGate("creditos_comprar", mergerAPIClienteRecebimento)` | routes_pagamentos.go:66 |
| `/merger/api/cliente/regras` | cliente | `mergerAPIClienteRegras` | routes_catalogo.go:69 |
| `/merger/api/cliente/renovacoes_pendentes` | cliente | `featureGate("sigma_users", mergerClienteRenovacoesPendentesHandler),` | routes_renovar.go:35 |
| `/merger/api/cliente/renovacoes_pendentes/confirmar` | cliente | `featureGate("sigma_users", mergerClienteRenovacaoConfirmarHandler),` | routes_renovar.go:39 |
| `/merger/api/cliente/reserva/forcar` | cliente | `featureGate("catalogo_fonte_reserva", mergerAPIClienteReservaForcar)` | routes_catalogo.go:89 |
| `/merger/api/cliente/reserva/restaurar` | cliente | `mergerAPIClienteReservaRestaurar) // SEM gate: undo sempre permitido // Sync / stats / dashboard http.HandleFunc("/merger/api/sync", mergerPainelAPISync` | routes_catalogo.go:90 |
| `/merger/api/cliente/revenda_unlimited` | cliente | `featureGate("revendas_arvore", mergerAPIClienteRevendaUnlimited)` | routes_revendas.go:40 |
| `/merger/api/cliente/revendas` | cliente | `featureGate("revendas_arvore", mergerAPIClienteRevendas)` | routes_revendas.go:39 |
| `/merger/api/cliente/revendas/arvore` | cliente | `featureGate("revendas_arvore", mergerAPIClienteRevendasArvore)` | routes_revendas.go:46 |
| `/merger/api/cliente/revendas/nota` | cliente | `featureGate("revendas_arvore", mergerAPIClienteRevendasNota)` | routes_revendas.go:47 |
| `/merger/api/cliente/revendas/renovar` | cliente | `featureGate("revendas_arvore", mergerAPIClienteRevendasRenovarManual)` | routes_revendas.go:48 |
| `/merger/api/cliente/seguranca_edicao` | cliente | `featureGate("seguranca_subrevendas", mergerClienteSegurancaEdicaoHandler)` | routes_sigma.go:86 |
| `/merger/api/cliente/seguranca/auto_bloqueio` | cliente | `mergerAPIClienteSegAutoBloqueioConfig` | routes_merger_core.go:181 |
| `/merger/api/cliente/seguranca/bloquear_ip` | cliente | `mergerAPIClienteSegBloquearIP` | routes_merger_core.go:177 |
| `/merger/api/cliente/seguranca/desbloquear_ip` | cliente | `mergerAPIClienteSegDesbloquearIP` | routes_merger_core.go:178 |
| `/merger/api/cliente/seguranca/ips` | cliente | `mergerAPIClienteSegIPsVistos` | routes_merger_core.go:179 |
| `/merger/api/cliente/seguranca/ips_bloqueados` | cliente | `mergerAPIClienteSegIPsBloqueados` | routes_merger_core.go:176 |
| `/merger/api/cliente/senha` | cliente | `mergerClienteSenhaHandler` | routes_merger_core.go:65 |
| `/merger/api/cliente/servidor_troca_visibilidade` | cliente | `featureGate("multi_servidor", mergerClienteServidorTrocaVisibilidadeHandler)` | routes_servidores.go:36 |
| `/merger/api/cliente/servidores_disponiveis` | cliente | `featureGate("multi_servidor", mergerAPIClienteServidoresDisponiveis)` | routes_servidores.go:28 |
| `/merger/api/cliente/sigma_live_xui` | cliente | `featureGate("conexoes_ativas", mergerClienteSigmaLiveXuiHandler)` | routes_sigma.go:100 |
| `/merger/api/cliente/sigma_orfas/cancelar` | cliente | `featureGate("sigma_remotos", mergerClienteSigmaOrfasCancelarHandler)` | routes_sigma.go:97 |
| `/merger/api/cliente/sigma_orfas/progresso` | cliente | `featureGate("sigma_remotos", mergerClienteSigmaOrfasProgressoHandler)` | routes_sigma.go:96 |
| `/merger/api/cliente/sigma_orfas/sincronizar` | cliente | `featureGate("sigma_remotos", mergerClienteSigmaOrfasSincronizarHandler)` | routes_sigma.go:95 |
| `/merger/api/cliente/sigma_remoto/` | cliente | `featureGate("sigma_remotos", mergerAPIClienteSigmaRemotoOps)` | routes_sigma.go:104 |
| `/merger/api/cliente/sigma_remoto/migrar_cancelar` | cliente | `featureGate("sigma_remotos", mergerClienteSigmaRemotoMigrarCancelarHandler)` | routes_sigma.go:92 |
| `/merger/api/cliente/sigma_remoto/migrar_progresso` | cliente | `featureGate("sigma_remotos", mergerClienteSigmaRemotoMigrarProgressoHandler)` | routes_sigma.go:91 |
| `/merger/api/cliente/sigma_remoto/migrar_todos` | cliente | `featureGate("sigma_remotos", mergerClienteSigmaRemotoMigrarTodosHandler)` | routes_sigma.go:90 |
| `/merger/api/cliente/sigma_remotos` | cliente | `featureGate("sigma_remotos", mergerAPIClienteSigmaRemotos)` | routes_sigma.go:103 |
| `/merger/api/cliente/sigma_remotos/liberados` | cliente | `featureGate("sigma_remotos", mergerAPIClienteSigmaRemotosLiberados)` | routes_sigma.go:110 |
| `/merger/api/cliente/sigma_user/migrar_do_local` | cliente | `featureGate("sigma_remotos", mergerClienteSigmaUserMigrarDoLocalHandler)` | routes_sigma.go:124 |
| `/merger/api/cliente/sigma_user/migrar_para_local` | cliente | `featureGate("sigma_remotos", mergerClienteSigmaUserMigrarParaLocalHandler)` | routes_sigma.go:123 |
| `/merger/api/cliente/sigma_users` | cliente | `featureGate("sigma_users", mergerClienteSigmaUsersHandler)` | routes_sigma.go:41 |
| `/merger/api/cliente/sigma_users/adultos` | cliente | `featureGate("sigma_users", mergerClienteSigmaUsersAdultosHandler)` | routes_sigma.go:47 |
| `/merger/api/cliente/sigma_users/adultos_candidatos` | cliente | `featureGate("sigma_users", mergerClienteSigmaUsersAdultosCandidatosHandler)` | routes_sigma.go:48 |
| `/merger/api/cliente/sigma_users/contato` | cliente | `featureGate("sigma_users", mergerClienteSigmaUsersContatoHandler)` | routes_sigma.go:50 |
| `/merger/api/cliente/sigma_users/criar` | cliente | `featureGate("sigma_users", mergerClienteSigmaUsersCriarHandler)` | routes_sigma.go:43 |
| `/merger/api/cliente/sigma_users/delete` | cliente | `featureGate("sigma_users", mergerClienteSigmaUsersDeleteHandler)` | routes_sigma.go:45 |
| `/merger/api/cliente/sigma_users/editar` | cliente | `featureGate("sigma_users", mergerClienteSigmaUsersEditarHandler)` | routes_sigma.go:49 |
| `/merger/api/cliente/sigma_users/estender` | cliente | `featureGate("sigma_users", mergerClienteSigmaUsersEstenderHandler)` | routes_sigma.go:51 |
| `/merger/api/cliente/sigma_users/migrar` | cliente | `featureGate("sigma_remotos", mergerClienteSigmaUserMigrarHandler)` | routes_sigma.go:89 |
| `/merger/api/cliente/sigma_users/renovacao_qr` | cliente | `mergerClienteSigmaUsersRenovacaoQRHandler` | routes_sp800.go:46 |
| `/merger/api/cliente/sigma_users/renovar` | cliente | `featureGate("sigma_users", mergerClienteSigmaUsersRenovarHandler)` | routes_sigma.go:46 |
| `/merger/api/cliente/sigma_users/revendas` | cliente | `featureGate("sigma_users", mergerClienteSigmaUsersRevendasHandler)` | routes_sigma.go:42 |
| `/merger/api/cliente/sigma_users/servidor` | cliente | `featureGate("multi_servidor", mergerClienteSigmaUsersServidorHandler)` | routes_servidores.go:32 |
| `/merger/api/cliente/sigma_users/servidor_massa` | cliente | `featureGate("multi_servidor", mergerClienteSigmaUsersServidorMassaHandler)` | routes_servidores.go:48 |
| `/merger/api/cliente/sigma_users/telas` | cliente | `featureGate("sigma_users", mergerClienteSigmaUsersTelasHandler)` | routes_sigma.go:52 |
| `/merger/api/cliente/sigma_users/toggle` | cliente | `featureGate("sigma_users", mergerClienteSigmaUsersToggleHandler)` | routes_sigma.go:44 |
| `/merger/api/cliente/sigma_users/troca_visivel` | cliente | `featureGate("multi_servidor", mergerClienteSigmaUsersTrocaVisivelHandler)` | routes_servidores.go:38 |
| `/merger/api/cliente/sigma_users/valor` | cliente | `featureGate("sigma_users", mergerClienteSigmaUsersValorHandler)` | routes_sigma.go:53 |
| `/merger/api/cliente/sp800/aviso-venda` | cliente | `g(oryxPortalAvisoVenda)` | routes_sp800.go:35 |
| `/merger/api/cliente/sp800/chave` | cliente | `g(oryxAPIChaveListar)` | routes_sp800.go:27 |
| `/merger/api/cliente/sp800/chave/codigo` | cliente | `g(oryxPortalChaveCodigo)` | routes_sp800.go:29 |
| `/merger/api/cliente/sp800/chave/salvar` | cliente | `g(oryxAPIChaveSalvar)` | routes_sp800.go:28 |
| `/merger/api/cliente/sp800/cobranca/criar` | cliente | `g(oryxAPICobrancaCriar)` | routes_sp800.go:25 |
| `/merger/api/cliente/sp800/cobranca/status` | cliente | `g(oryxAPICobrancaStatus)` | routes_sp800.go:26 |
| `/merger/api/cliente/sp800/comprovante/` | cliente | `g(oryxAPIComprovanteCliente)` | routes_sp800.go:54 |
| `/merger/api/cliente/sp800/comprovante/recebimento/` | cliente | `g(oryxAPIComprovanteRecebimentoCliente)` | routes_sp800.go:53 |
| `/merger/api/cliente/sp800/eu` | cliente | `featureGate("sp800", oryxClienteEu)` | routes_sp800.go:20 |
| `/merger/api/cliente/sp800/limites` | cliente | `g(oryxAPILimites)` | routes_sp800.go:31 |
| `/merger/api/cliente/sp800/marca` | cliente | `g(oryxAPIMarcaVer)` | routes_sp800.go:32 |
| `/merger/api/cliente/sp800/marca/logo` | cliente | `g(oryxAPIMarcaLogo)` | routes_sp800.go:34 |
| `/merger/api/cliente/sp800/marca/salvar` | cliente | `g(oryxAPIMarcaSalvar)` | routes_sp800.go:33 |
| `/merger/api/cliente/sp800/movimentos` | cliente | `g(oryxAPIMovimentosListar)` | routes_sp800.go:22 |
| `/merger/api/cliente/sp800/pedidos` | cliente | `g(oryxAPIPedidosListar)` | routes_sp800.go:23 |
| `/merger/api/cliente/sp800/pedidos/criar` | cliente | `g(oryxAPIPedidoCriar)` | routes_sp800.go:24 |
| `/merger/api/cliente/sp800/renovacao_toggle` | cliente | `g(oryxClienteRenovacaoToggle)` | routes_sp800.go:38 |
| `/merger/api/cliente/sp800/saldo` | cliente | `g(oryxAPISaldo)` | routes_sp800.go:21 |
| `/merger/api/cliente/sp800/saque/codigo` | cliente | `g(oryxPortalSaqueCodigo)` | routes_sp800.go:30 |
| `/merger/api/cliente/suporte-ia/verificar-conta` | cliente | `featureGate("assistente", suportePainelAPIVerificarConta)` | routes_assistente.go:24 |
| `/merger/api/cliente/template` | cliente | `featureGate("templates_mensagem", mergerAPIClienteTemplate)` | routes_revendas.go:56 |
| `/merger/api/cliente/templates_plano` | cliente | `featureGate("templates_mensagem", mergerAPIClienteTemplatesPlano)` | routes_revendas.go:58 |
| `/merger/api/cliente/templates_plano/remover` | cliente | `featureGate("templates_mensagem", mergerAPIClienteTemplatesPlanoRemover)` | routes_revendas.go:59 |
| `/merger/api/cliente/tg_afiliados/painel` | cliente | `featureGate("telegram", mergerAPIClienteTGAfiliadosPublicos)` | routes_telegram.go:27 |
| `/merger/api/cliente/tg_afiliados/saque_decidir` | cliente | `featureGate("telegram", mergerAPIClienteTGAfiliadoPublicoSaqueDecidir)` | routes_telegram.go:28 |
| `/merger/api/cliente/tg_apagar_mensagem` | cliente | `featureGate("atendimento_tg", mergerAPIClienteTGApagarMsg)` | routes_telegram.go:115 |
| `/merger/api/cliente/tg_arquivo` | cliente | `featureGate("atendimento_tg", mergerAPIClienteTGArquivo)` | routes_telegram.go:100 |
| `/merger/api/cliente/tg_atendimento/assumir` | cliente | `featureGate("atendimento_tg", mergerAPIClienteTGAtendimentoAssumir)` | routes_telegram.go:110 |
| `/merger/api/cliente/tg_atendimento/devolver` | cliente | `featureGate("atendimento_tg", mergerAPIClienteTGAtendimentoDevolver)` | routes_telegram.go:111 |
| `/merger/api/cliente/tg_atendimento/status` | cliente | `featureGate("atendimento_tg", mergerAPIClienteTGAtendimentoStatus)` | routes_telegram.go:112 |
| `/merger/api/cliente/tg_autoresposta/criar` | cliente | `featureGate("telegram", mergerAPIClienteTGAutoRespCriar)` | routes_telegram.go:95 |
| `/merger/api/cliente/tg_autoresposta/deletar` | cliente | `featureGate("telegram", mergerAPIClienteTGAutoRespDeletar)` | routes_telegram.go:97 |
| `/merger/api/cliente/tg_autoresposta/editar` | cliente | `featureGate("telegram", mergerAPIClienteTGAutoRespEditar)` | routes_telegram.go:96 |
| `/merger/api/cliente/tg_autoresposta/listar` | cliente | `featureGate("telegram", mergerAPIClienteTGAutoRespListar)` | routes_telegram.go:94 |
| `/merger/api/cliente/tg_broadcast/enviar` | cliente | `featureGate("telegram", mergerAPIClienteTGBroadcastEnviar)` | routes_telegram.go:87 |
| `/merger/api/cliente/tg_broadcast/listar` | cliente | `featureGate("telegram", mergerAPIClienteTGBroadcastListar)` | routes_telegram.go:88 |
| `/merger/api/cliente/tg_broadcast/preview` | cliente | `featureGate("telegram", mergerAPIClienteTGBroadcastPreview)` | routes_telegram.go:86 |
| `/merger/api/cliente/tg_canal` | cliente | `featureGate("telegram", func(w http.ResponseWriter, r *http.Request) { if r.Method == http.MethodPost { mergerAPIClienteTGCanalSet(w, r` | routes_telegram.go:47 |
| `/merger/api/cliente/tg_cancelar` | cliente | `featureGate("telegram", mergerClienteTelegramCancelarHandler)` | routes_telegram.go:30 |
| `/merger/api/cliente/tg_comandos/listar` | cliente | `featureGate("telegram", mergerAPIClienteTGComandosListar)` | routes_telegram.go:75 |
| `/merger/api/cliente/tg_comandos/toggle` | cliente | `featureGate("telegram", mergerAPIClienteTGComandosToggle)` | routes_telegram.go:76 |
| `/merger/api/cliente/tg_conectar` | cliente | `featureGate("telegram", mergerClienteTelegramConectarHandler)` | routes_telegram.go:29 |
| `/merger/api/cliente/tg_config` | cliente | `featureGate("telegram", mergerClienteTelegramConfigHandler)` | routes_telegram.go:25 |
| `/merger/api/cliente/tg_conversas/listar` | cliente | `featureGate("atendimento_tg", mergerAPIClienteTGConversasListar)` | routes_telegram.go:79 |
| `/merger/api/cliente/tg_conversas/mensagens` | cliente | `featureGate("atendimento_tg", mergerAPIClienteTGConversasMensagens)` | routes_telegram.go:80 |
| `/merger/api/cliente/tg_conversas/responder` | cliente | `featureGate("atendimento_tg", mergerAPIClienteTGConversasResponder)` | routes_telegram.go:83 |
| `/merger/api/cliente/tg_dashboard_stats` | cliente | `featureGate("telegram", mergerAPIClienteTGDashboardStats)` | routes_telegram.go:91 |
| `/merger/api/cliente/tg_desconectar` | cliente | `featureGate("telegram", mergerClienteTelegramDesconectarHandler)` | routes_telegram.go:31 |
| `/merger/api/cliente/tg_editar_mensagem` | cliente | `featureGate("atendimento_tg", mergerAPIClienteTGEditarMsg)` | routes_telegram.go:116 |
| `/merger/api/cliente/tg_enviar/audio` | cliente | `featureGate("atendimento_tg", mergerAPIClienteTGEnviarAudio)` | routes_telegram.go:105 |
| `/merger/api/cliente/tg_enviar/documento` | cliente | `featureGate("atendimento_tg", mergerAPIClienteTGEnviarDocumento)` | routes_telegram.go:104 |
| `/merger/api/cliente/tg_enviar/foto` | cliente | `featureGate("atendimento_tg", mergerAPIClienteTGEnviarFoto)` | routes_telegram.go:103 |
| `/merger/api/cliente/tg_enviar/video` | cliente | `featureGate("atendimento_tg", mergerAPIClienteTGEnviarVideo)` | routes_telegram.go:106 |
| `/merger/api/cliente/tg_enviar/voice` | cliente | `featureGate("atendimento_tg", mergerAPIClienteTGEnviarVoice)` | routes_telegram.go:107 |
| `/merger/api/cliente/tg_link_cliente/gerar` | cliente | `featureGate("telegram", mergerAPIClienteTGLinkClienteGerar)` | routes_telegram.go:56 |
| `/merger/api/cliente/tg_status` | cliente | `featureGate("telegram", mergerClienteTelegramStatusHandler)` | routes_telegram.go:26 |
| `/merger/api/cliente/tg_vinculos` | cliente | `featureGate("telegram", mergerClienteTelegramVinculosListarHandler)` | routes_telegram.go:34 |
| `/merger/api/cliente/tg_vinculos/gerar_link` | cliente | `featureGate("telegram", mergerClienteTelegramVinculosGerarLinkHandler)` | routes_telegram.go:35 |
| `/merger/api/cliente/tg_vinculos/notif` | cliente | `featureGate("telegram", mergerClienteTelegramVinculosNotifHandler)` | routes_telegram.go:37 |
| `/merger/api/cliente/tg_vinculos/revogar` | cliente | `featureGate("telegram", mergerClienteTelegramVinculosRevogarHandler)` | routes_telegram.go:36 |
| `/merger/api/cliente/tickets/criar` | cliente | `featureGate("tickets", ticketsAPICriar)` | routes_tickets.go:21 |
| `/merger/api/cliente/tickets/fechar` | cliente | `featureGate("tickets", ticketsAPIFecharCliente)` | routes_tickets.go:29 |
| `/merger/api/cliente/tickets/listar` | cliente | `featureGate("tickets", ticketsAPIListarCliente)` | routes_tickets.go:23 |
| `/merger/api/cliente/tickets/responder` | cliente | `featureGate("tickets", ticketsAPIResponderCliente)` | routes_tickets.go:27 |
| `/merger/api/cliente/tickets/ver` | cliente | `featureGate("tickets", ticketsAPIVerCliente)` | routes_tickets.go:25 |
| `/merger/api/cliente/tmdb/videos` | cliente | `mergerAPIClienteTMDBVideos` | routes_merger_core.go:216 |
| `/merger/api/cliente/transfer_posse` | cliente | `featureGate("sigma_users", mergerClienteTransferPosseHandler)` | routes_sigma.go:73 |
| `/merger/api/cliente/transfer_revenda` | cliente | `featureGate("revendas_arvore", mergerClienteTransferRevendaHandler)` | routes_sigma.go:80 |
| `/merger/api/cliente/transfer_revenda/destinos` | cliente | `featureGate("revendas_arvore", mergerClienteTransferRevendaDestinosHandler)` | routes_sigma.go:81 |
| `/merger/api/cliente/video/baixar` | cliente | `mergerAPIClienteVideoBaixar` | routes_merger_core.go:213 |
| `/merger/api/cliente/video/processar` | cliente | `mergerAPIClienteVideoProcessar` | routes_merger_core.go:211 |
| `/merger/api/cliente/video/status` | cliente | `mergerAPIClienteVideoStatus` | routes_merger_core.go:212 |
| `/merger/api/cliente/wa_atendimento/apelido` | cliente | `featureGate("atendimento_wa", mergerClienteWAApelidoSalvarHandler)` | routes_whatsapp.go:164 |
| `/merger/api/cliente/wa_atendimento/cliente_info` | cliente | `featureGate("atendimento_wa", mergerClienteWhatsAppAtendimentoClienteInfoHandler)` | routes_whatsapp.go:70 |
| `/merger/api/cliente/wa_atendimento/conversas` | cliente | `featureGate("atendimento_wa", mergerClienteWhatsAppAtendimentoConversasHandler)` | routes_whatsapp.go:52 |
| `/merger/api/cliente/wa_atendimento/editar_contato` | cliente | `featureGate("atendimento_wa", mergerClienteWhatsAppAtendimentoEditarContatoHandler)` | routes_whatsapp.go:71 |
| `/merger/api/cliente/wa_atendimento/encerrar` | cliente | `featureGate("atendimento_wa", mergerClienteWhatsAppAtendimentoEncerrarHandler)` | routes_whatsapp.go:55 |
| `/merger/api/cliente/wa_atendimento/enviar` | cliente | `featureGate("atendimento_wa", mergerClienteWhatsAppAtendimentoEnviarHandler)` | routes_whatsapp.go:54 |
| `/merger/api/cliente/wa_atendimento/enviar_midia` | cliente | `featureGate("atendimento_wa", mergerClienteWhatsAppEnviarMidiaHandler)` | routes_whatsapp.go:67 |
| `/merger/api/cliente/wa_atendimento/exportar` | cliente | `featureGate("atendimento_wa", mergerClienteWhatsAppExportarHandler)` | routes_whatsapp.go:62 |
| `/merger/api/cliente/wa_atendimento/foto` | cliente | `featureGate("atendimento_wa", mergerClienteWhatsAppFotoHandler)` | routes_whatsapp.go:59 |
| `/merger/api/cliente/wa_atendimento/historico` | cliente | `featureGate("atendimento_wa", mergerClienteWhatsAppAtendimentoHistoricoHandler)` | routes_whatsapp.go:53 |
| `/merger/api/cliente/wa_atendimento/marcar_lida` | cliente | `featureGate("atendimento_wa", mergerClienteWhatsAppAtendimentoMarcarLidaHandler)` | routes_whatsapp.go:56 |
| `/merger/api/cliente/wa_atendimento/midia` | cliente | `featureGate("atendimento_wa", mergerClienteWhatsAppMidiaHandler)` | routes_whatsapp.go:65 |
| `/merger/api/cliente/wa_atendimento/midia/redownload` | cliente | `featureGate("atendimento_wa", mergerClienteWhatsAppMidiaRedownloadHandler)` | routes_whatsapp.go:66 |
| `/merger/api/cliente/wa_atendimento/notas` | cliente | `featureGate("atendimento_wa", mergerClienteWANotasListarHandler)` | routes_whatsapp.go:161 |
| `/merger/api/cliente/wa_atendimento/notas/resolver` | cliente | `featureGate("atendimento_wa", mergerClienteWANotasResolverHandler)` | routes_whatsapp.go:162 |
| `/merger/api/cliente/wa_atendimento/renovar_pix` | cliente | `featureGate("atendimento_wa", mergerClienteWhatsAppAtendimentoRenovarPixHandler)` | routes_whatsapp.go:72 |
| `/merger/api/cliente/wa_atendimento/silenciar_bot` | cliente | `featureGate("atendimento_wa", mergerClienteWhatsAppAtendimentoSilenciarBotHandler)` | routes_whatsapp.go:74 |
| `/merger/api/cliente/wa_avisos/config` | cliente | `featureGate("whatsapp", mergerClienteWhatsAppAvisosConfigHandler)` | routes_whatsapp.go:177 |
| `/merger/api/cliente/wa_avisos/historico` | cliente | `featureGate("whatsapp", mergerClienteWhatsAppAvisosHistoricoHandler)` | routes_whatsapp.go:179 |
| `/merger/api/cliente/wa_avisos/testar` | cliente | `featureGate("whatsapp", mergerClienteWhatsAppAvisosTestarHandler)` | routes_whatsapp.go:178 |
| `/merger/api/cliente/wa_cancelar` | cliente | `featureGate("whatsapp", mergerClienteWhatsAppCancelarHandler)` | routes_whatsapp.go:35 |
| `/merger/api/cliente/wa_conectar` | cliente | `featureGate("whatsapp", mergerClienteWhatsAppConectarHandler)` | routes_whatsapp.go:34 |
| `/merger/api/cliente/wa_config` | cliente | `featureGate("whatsapp", mergerClienteWhatsAppConfigHandler)` | routes_whatsapp.go:32 |
| `/merger/api/cliente/wa_enviar_teste` | cliente | `featureGate("whatsapp", mergerClienteWhatsAppEnviarTesteHandler)` | routes_whatsapp.go:36 |
| `/merger/api/cliente/wa_etiquetas` | cliente | `featureGate("whatsapp", mergerClienteWhatsAppEtiquetasListarHandler)` | routes_whatsapp.go:167 |
| `/merger/api/cliente/wa_etiquetas/atribuir` | cliente | `featureGate("whatsapp", mergerClienteWhatsAppEtiquetasAtribuirHandler)` | routes_whatsapp.go:171 |
| `/merger/api/cliente/wa_etiquetas/conversa` | cliente | `featureGate("whatsapp", mergerClienteWhatsAppEtiquetasConversaHandler)` | routes_whatsapp.go:173 |
| `/merger/api/cliente/wa_etiquetas/criar` | cliente | `featureGate("whatsapp", mergerClienteWhatsAppEtiquetasCriarHandler)` | routes_whatsapp.go:168 |
| `/merger/api/cliente/wa_etiquetas/deletar` | cliente | `featureGate("whatsapp", mergerClienteWhatsAppEtiquetasDeletarHandler)` | routes_whatsapp.go:170 |
| `/merger/api/cliente/wa_etiquetas/desatribuir` | cliente | `featureGate("whatsapp", mergerClienteWhatsAppEtiquetasDesatribuirHandler)` | routes_whatsapp.go:172 |
| `/merger/api/cliente/wa_etiquetas/editar` | cliente | `featureGate("whatsapp", mergerClienteWhatsAppEtiquetasEditarHandler)` | routes_whatsapp.go:169 |
| `/merger/api/cliente/wa_etiquetas/regra` | cliente | `featureGate("whatsapp", mergerClienteWhatsAppEtiquetasRegraHandler)` | routes_whatsapp.go:174 |
| `/merger/api/cliente/wa_instancias` | cliente | `featureGate("whatsapp", mergerClienteWAInstanciasListarHandler)` | routes_whatsapp.go:196 |
| `/merger/api/cliente/wa_instancias/conectar` | cliente | `featureGate("whatsapp", mergerClienteWAInstanciasConectarHandler)` | routes_whatsapp.go:201 |
| `/merger/api/cliente/wa_instancias/criar` | cliente | `featureGate("whatsapp", mergerClienteWAInstanciasCriarHandler)` | routes_whatsapp.go:197 |
| `/merger/api/cliente/wa_instancias/default` | cliente | `featureGate("whatsapp", mergerClienteWAInstanciasMarcarDefaultHandler)` | routes_whatsapp.go:200 |
| `/merger/api/cliente/wa_instancias/deletar` | cliente | `featureGate("whatsapp", mergerClienteWAInstanciasDeletarHandler)` | routes_whatsapp.go:199 |
| `/merger/api/cliente/wa_instancias/desconectar` | cliente | `featureGate("whatsapp", mergerClienteWAInstanciasDesconectarHandler)` | routes_whatsapp.go:202 |
| `/merger/api/cliente/wa_instancias/editar` | cliente | `featureGate("whatsapp", mergerClienteWAInstanciasEditarHandler)` | routes_whatsapp.go:198 |
| `/merger/api/cliente/wa_instancias/status` | cliente | `featureGate("whatsapp", mergerClienteWAInstanciasStatusHandler)` | routes_whatsapp.go:203 |
| `/merger/api/cliente/wa_liberar_todas` | cliente | `featureGate("whatsapp", mergerClienteWALiberarTodasHandler)` | routes_whatsapp.go:207 |
| `/merger/api/cliente/wa_menu` | cliente | `featureGate("whatsapp", mergerClienteWhatsAppMenuHandler)` | routes_whatsapp.go:40 |
| `/merger/api/cliente/wa_status` | cliente | `featureGate("whatsapp", mergerClienteWhatsAppStatusHandler)` | routes_whatsapp.go:33 |
| `/merger/api/cliente/wa_teste_config` | cliente | `featureGate("whatsapp", mergerClienteWhatsAppTesteConfigHandler)` | routes_whatsapp.go:43 |
| `/merger/api/cliente/wa_vinculos` | cliente | `featureGate("whatsapp", mergerClienteWhatsAppVinculosHandler)` | routes_whatsapp.go:46 |
| `/merger/api/cliente/wa_vinculos/candidatos` | cliente | `featureGate("whatsapp", mergerClienteWhatsAppVinculosCandidatosHandler)` | routes_whatsapp.go:49 |
| `/merger/api/cliente/wa_vinculos/criar` | cliente | `featureGate("whatsapp", mergerClienteWhatsAppVinculosCriarHandler)` | routes_whatsapp.go:47 |
| `/merger/api/cliente/wa_vinculos/deletar` | cliente | `featureGate("whatsapp", mergerClienteWhatsAppVinculosDeletarHandler)` | routes_whatsapp.go:48 |
| `/merger/api/cliente/xui_fonte/mesclar` | cliente | `featureGate("sigma_remotos", mergerAPIClienteXUIFonteMesclar)` | routes_sigma.go:115 |

## admin

| Rota | Metodo/escopo | Handler | Fonte |
|---|---|---|---|
| `/merger/api/admin/2fa/meu` | admin | `mergerAPIAdmin2FAMeuEstado` | routes_merger_core.go:79 |
| `/merger/api/admin/2fa/meu/confirmar` | admin | `mergerAPIAdmin2FAMeuConfirmar` | routes_merger_core.go:81 |
| `/merger/api/admin/2fa/meu/desativar` | admin | `mergerAPIAdmin2FAMeuDesativar` | routes_merger_core.go:82 |
| `/merger/api/admin/2fa/meu/enviar` | admin | `mergerAPIAdmin2FAMeuEnviar` | routes_merger_core.go:80 |
| `/merger/api/admin/2fa/resetar` | admin | `mergerAPIAdmin2FAResetar` | routes_merger_core.go:75 |
| `/merger/api/admin/admin_tg/chats` | admin | `mergerAPIAdminTGChats` | routes_telegram.go:124 |
| `/merger/api/admin/admin_tg/conectar` | admin | `mergerAPIAdminTGConectar` | routes_telegram.go:123 |
| `/merger/api/admin/admin_tg/config` | admin | `mergerAPIAdminTGConfig` | routes_telegram.go:122 |
| `/merger/api/admin/admin_tg/testar_chat` | admin | `mergerAPIAdminTGTestarChat` | routes_telegram.go:125 |
| `/merger/api/admin/admin_wa/conectar` | admin | `mergerAPIAdminWAConectar` | routes_whatsapp.go:187 |
| `/merger/api/admin/admin_wa/config` | admin | `mergerAPIAdminWAConfig` | routes_whatsapp.go:186 |
| `/merger/api/admin/admin_wa/numeros` | admin | `mergerAPIAdminWANumeros` | routes_whatsapp.go:188 |
| `/merger/api/admin/admin_wa/testar_chat` | admin | `mergerAPIAdminWATestarChat` | routes_whatsapp.go:189 |
| `/merger/api/admin/adultos_auto` | admin | `mergerAPIAdminAdultosAuto` | routes_catalogo.go:66 |
| `/merger/api/admin/agenda_config` | admin | `mergerAPIAdminAgendaConfig` | routes_merger_core.go:184 |
| `/merger/api/admin/agenda_testar` | admin | `mergerAPIAdminAgendaTestar` | routes_merger_core.go:185 |
| `/merger/api/admin/apps_ativa/config` | admin | `mergerAdminAppsAtivaConfig` | routes_apps_ativa.go:19 |
| `/merger/api/admin/apps_ativa/historico` | admin | `mergerAdminAppsAtivaHistoricoGlobal` | routes_apps_ativa.go:32 |
| `/merger/api/admin/apps_ativa/icone` | admin | `mergerAdminAppsAtivaIcone` | routes_apps_ativa.go:44 |
| `/merger/api/admin/apps_ativa/icone_upload` | admin | `mergerAdminAppsAtivaIconeUpload` | routes_apps_ativa.go:45 |
| `/merger/api/admin/apps_ativa/icones_bulk` | admin | `mergerAdminAppsAtivaIconesBulk` | routes_apps_ativa.go:47 |
| `/merger/api/admin/apps_ativa/loja_problemas` | admin | `mergerAdminAppsAtivaLojaProblemas` | routes_apps_ativa.go:57 |
| `/merger/api/admin/apps_ativa/loja_resolver` | admin | `mergerAdminAppsAtivaLojaResolver` | routes_apps_ativa.go:59 |
| `/merger/api/admin/apps_ativa/loja_retentar` | admin | `mergerAdminAppsAtivaLojaRetentar` | routes_apps_ativa.go:58 |
| `/merger/api/admin/apps_ativa/ocultar` | admin | `mergerAdminAppsAtivaOcultar` | routes_apps_ativa.go:39 |
| `/merger/api/admin/apps_ativa/precos` | admin | `mergerAdminAppsAtivaPrecos` | routes_apps_ativa.go:38 |
| `/merger/api/admin/apps_ativa/testar` | admin | `mergerAdminAppsAtivaTestar` | routes_apps_ativa.go:20 |
| `/merger/api/admin/apps_ativa/transferir_creditos` | admin | `mergerAdminAppsAtivaTransferir` | routes_apps_ativa.go:21 |
| `/merger/api/admin/apps_ativa/webhook_info` | admin | `mergerAdminAppsAtivaWebhookInfo` | routes_apps_ativa.go:50 |
| `/merger/api/admin/apps_ativa/webhook_rotate` | admin | `mergerAdminAppsAtivaWebhookRotate` | routes_apps_ativa.go:51 |
| `/merger/api/admin/apps_parceiros/apps` | admin | `mergerAdminAppsParceirosApps` | routes_apps_parceiros.go:22 |
| `/merger/api/admin/apps_parceiros/apps/` | admin | `mergerAdminAppsParceirosApp) // /apps/{id} http.HandleFunc("/merger/api/admin/apps_parceiros/sincronizar_imagens", mergerAdminAppsParceirosSincronizar` | routes_apps_parceiros.go:23 |
| `/merger/api/admin/apps_parceiros/codigo` | admin | `mergerAdminAppsParceirosCodigo` | routes_apps_parceiros.go:26 |
| `/merger/api/admin/apps_parceiros/codigos` | admin | `mergerAdminAppsParceirosCodigos` | routes_apps_parceiros.go:25 |
| `/merger/api/admin/asn/acessos` | admin | `mergerAPIAdminASNAcessos` | routes_asn.go:36 |
| `/merger/api/admin/asn/adicionar` | admin | `mergerAPIAdminASNAdicionar` | routes_asn.go:38 |
| `/merger/api/admin/asn/allowlist/adicionar` | admin | `mergerAPIAdminASNAllowlistAdd` | routes_asn.go:41 |
| `/merger/api/admin/asn/allowlist/remover` | admin | `mergerAPIAdminASNAllowlistRemove` | routes_asn.go:42 |
| `/merger/api/admin/asn/base_status` | admin | `mergerAPIAdminASNBaseStatus` | routes_asn.go:44 |
| `/merger/api/admin/asn/config` | admin | `mergerAPIAdminASNConfig` | routes_asn.go:45 |
| `/merger/api/admin/asn/listar` | admin | `mergerAPIAdminASNListar` | routes_asn.go:37 |
| `/merger/api/admin/asn/manter` | admin | `mergerAPIAdminASNManter` | routes_asn.go:40 |
| `/merger/api/admin/asn/preview` | admin | `mergerAPIAdminASNPreview` | routes_asn.go:43 |
| `/merger/api/admin/asn/remover` | admin | `mergerAPIAdminASNRemover` | routes_asn.go:39 |
| `/merger/api/admin/asn/upload_csv` | admin | `mergerAPIAdminASNUploadCSV` | routes_asn.go:46 |
| `/merger/api/admin/assistente/audio/config` | admin | `assistenteAPIAdminAudioConfig` | routes_assistente.go:20 |
| `/merger/api/admin/assistente/liberar` | admin | `assistenteAPIHandler("liberar", true)` | routes_assistente.go:17 |
| `/merger/api/admin/assistente/reportes` | admin | `assistenteAPIHandler("reportes", true)` | routes_assistente.go:18 |
| `/merger/api/admin/assistente/reportes/estado` | admin | `assistenteAPIHandler("reportes/estado", true)` | routes_assistente.go:19 |
| `/merger/api/admin/aviso_franquia/desligar` | admin | `mergerAPIAdminAvisoFranquiaDesligar` | routes_merger_core.go:91 |
| `/merger/api/admin/aviso_franquia/imagem` | admin | `mergerAPIAdminAvisoFranquiaImagem` | routes_merger_core.go:89 |
| `/merger/api/admin/aviso_franquia/ligar` | admin | `mergerAPIAdminAvisoFranquiaLigar` | routes_merger_core.go:90 |
| `/merger/api/admin/aviso_franquia/lista` | admin | `mergerAPIAdminAvisoFranquiaLista` | routes_merger_core.go:87 |
| `/merger/api/admin/aviso_franquia/status` | admin | `mergerAPIAdminAvisoFranquiaStatus` | routes_merger_core.go:88 |
| `/merger/api/admin/avisos/criar` | admin | `avisosAPICriarAdmin` | routes_loja_pwa.go:85 |
| `/merger/api/admin/avisos/delete` | admin | `pwaAvisoDeleteHandler` | routes_loja_pwa.go:88 |
| `/merger/api/admin/avisos/franquias` | admin | `avisosAPIFranquiasAdmin` | routes_loja_pwa.go:86 |
| `/merger/api/admin/avisos/leituras` | admin | `avisosAPILeiturasAdmin` | routes_loja_pwa.go:87 |
| `/merger/api/admin/backup/status` | admin | `backupStatusHandler` | routes_merger_core.go:57 |
| `/merger/api/admin/backup/verificar` | admin | `backupVerificarHandler` | routes_merger_core.go:58 |
| `/merger/api/admin/cat/apagar_manual` | admin | `mergerAPIAdminCatApagarManual` | routes_catalogo.go:58 |
| `/merger/api/admin/cat/canais` | admin | `mergerAPIAdminCatCanais` | routes_catalogo.go:55 |
| `/merger/api/admin/cat/colchete` | admin | `mergerAPIAdminCatColchete` | routes_catalogo.go:64 |
| `/merger/api/admin/cat/criar_manual` | admin | `mergerAPIAdminCatCriarManual` | routes_catalogo.go:54 |
| `/merger/api/admin/cat/data` | admin | `mergerAPIAdminCatData` | routes_catalogo.go:62 |
| `/merger/api/admin/cat/desmover_canal` | admin | `mergerAPIAdminCatDesmoverCanal` | routes_catalogo.go:57 |
| `/merger/api/admin/cat/fixadas` | admin | `mergerAPIAdminCatFixadas` | routes_catalogo.go:60 |
| `/merger/api/admin/cat/lista` | admin | `mergerAPIAdminCatLista` | routes_catalogo.go:52 |
| `/merger/api/admin/cat/mover_canais` | admin | `mergerAPIAdminCatMoverCanais` | routes_catalogo.go:56 |
| `/merger/api/admin/cat/ordem` | admin | `mergerAPIAdminCatOrdem` | routes_catalogo.go:48 |
| `/merger/api/admin/cat/renomear` | admin | `mergerAPIAdminCatRenomear` | routes_catalogo.go:50 |
| `/merger/api/admin/cat/unidas` | admin | `mergerAPIAdminCatUnidas` | routes_catalogo.go:51 |
| `/merger/api/admin/cat/unir` | admin | `mergerAPIAdminCatUnir` | routes_catalogo.go:49 |
| `/merger/api/admin/cloudflare/apelidos` | admin | `func(w http.ResponseWriter, r *http.Request) { switch r.Method { case "GET": mergerAdminCloudflareApelidosListar(w, r` | routes_cloudflare.go:62 |
| `/merger/api/admin/cloudflare/atribuir` | admin | `func(w http.ResponseWriter, r *http.Request) { // Mesma URL atende POST (atribuir) e DELETE (remover). if r.Method == "DELETE" { mergerAdminCloudflareRemoverAtribuicao(w, r` | routes_cloudflare.go:17 |
| `/merger/api/admin/cloudflare/config` | admin | `mergerAdminCloudflareConfig` | routes_cloudflare.go:14 |
| `/merger/api/admin/cloudflare/franquias` | admin | `mergerAdminCloudflareFranquias` | routes_cloudflare.go:27 |
| `/merger/api/admin/cloudflare/testar` | admin | `mergerAdminCloudflareTestar` | routes_cloudflare.go:15 |
| `/merger/api/admin/cloudflare/tuneis` | admin | `func(w http.ResponseWriter, r *http.Request) { switch r.Method { case "GET": mergerAdminCloudflareTuneisListar(w, r` | routes_cloudflare.go:82 |
| `/merger/api/admin/cloudflare/tunnel_config` | admin | `mergerAdminCloudflareTunnelConfig` | routes_cloudflare.go:97 |
| `/merger/api/admin/cloudflare/tunnel_criar` | admin | `mergerAdminCloudflareTunnelCriar` | routes_cloudflare.go:101 |
| `/merger/api/admin/cloudflare/tunnel_importar_lote` | admin | `mergerAdminCloudflareTunnelImportarLote` | routes_cloudflare.go:104 |
| `/merger/api/admin/cloudflare/tunnel_listar_cf` | admin | `mergerAdminCloudflareTunnelListarCF` | routes_cloudflare.go:102 |
| `/merger/api/admin/cloudflare/tunnel_testar` | admin | `mergerAdminCloudflareTunnelTestar` | routes_cloudflare.go:98 |
| `/merger/api/admin/cloudflare/tunnel_token` | admin | `mergerAdminCloudflareTunnelTokenCF` | routes_cloudflare.go:103 |
| `/merger/api/admin/cloudflare/zonas` | admin | `mergerAdminCloudflareZonas` | routes_cloudflare.go:16 |
| `/merger/api/admin/cobranca_tiers_default` | admin | `mergerAPIAdminCobrancaTiersDefault` | routes_revendas.go:36 |
| `/merger/api/admin/creditos-pacotes` | admin | `creditosPacotesAdminAPI` | routes_pagamentos.go:77 |
| `/merger/api/admin/creditos-pacotes/remover` | admin | `creditosPacotesAdminRemover` | routes_pagamentos.go:78 |
| `/merger/api/admin/criar_teste` | admin | `mergerAdminCriarTeste` | routes_revendas.go:80 |
| `/merger/api/admin/diag/armar` | admin | `diagSoAdmin(diagAPIArmar)` | routes_diag.go:37 |
| `/merger/api/admin/diag/desarmar` | admin | `diagSoAdmin(diagAPIDesarmar)` | routes_diag.go:38 |
| `/merger/api/admin/diag/export` | admin | `diagSoAdmin(diagAPIExport)` | routes_diag.go:41 |
| `/merger/api/admin/diag/geo` | admin | `diagSoAdmin(diagAPIGeo)` | routes_diag.go:42 |
| `/merger/api/admin/diag/sessao` | admin | `diagSoAdmin(diagAPISessao)` | routes_diag.go:40 |
| `/merger/api/admin/diag/sessoes` | admin | `diagSoAdmin(diagAPISessoes)` | routes_diag.go:39 |
| `/merger/api/admin/disparo/config` | admin | `disparoSoAdmin(disparoAPIConfig)` | routes_disparo.go:41 |
| `/merger/api/admin/disparo/destinatarios` | admin | `disparoSoAdmin(disparoAPIDestinatarios)` | routes_disparo.go:38 |
| `/merger/api/admin/disparo/enviar` | admin | `disparoSoAdmin(disparoAPIEnviar)` | routes_disparo.go:39 |
| `/merger/api/admin/disparo/numeros` | admin | `disparoSoAdmin(disparoAPINumeros)` | routes_disparo.go:40 |
| `/merger/api/admin/disparo/templates` | admin | `disparoSoAdmin(disparoAPITemplates)` | routes_disparo.go:37 |
| `/merger/api/admin/dns_trava` | admin | `mergerAPIAdminDNSTravaEstado` | routes_dns_trava.go:16 |
| `/merger/api/admin/dns_trava/dns_add` | admin | `mergerAPIAdminDNSTravaAdd` | routes_dns_trava.go:18 |
| `/merger/api/admin/dns_trava/dns_del` | admin | `mergerAPIAdminDNSTravaDel` | routes_dns_trava.go:19 |
| `/merger/api/admin/dns_trava/toggle` | admin | `mergerAPIAdminDNSTravaToggle` | routes_dns_trava.go:17 |
| `/merger/api/admin/equipe/abas` | admin | `mergerAPIAdminEquipeAbas` | routes_merger_core.go:138 |
| `/merger/api/admin/equipe/criar` | admin | `mergerAPIAdminEquipeCriar` | routes_merger_core.go:137 |
| `/merger/api/admin/equipe/excluir` | admin | `mergerAPIAdminEquipeExcluir` | routes_merger_core.go:141 |
| `/merger/api/admin/equipe/listar` | admin | `mergerAPIAdminEquipeListar` | routes_merger_core.go:136 |
| `/merger/api/admin/equipe/senha` | admin | `mergerAPIAdminEquipeSenha` | routes_merger_core.go:139 |
| `/merger/api/admin/equipe/toggle` | admin | `mergerAPIAdminEquipeToggle` | routes_merger_core.go:140 |
| `/merger/api/admin/eventos` | admin | `mergerAPIAdminEventos` | routes_merger_core.go:156 |
| `/merger/api/admin/eventos/exportar` | admin | `mergerAdminAtividadeExportar` | routes_merger_core.go:157 |
| `/merger/api/admin/eventos/franquia` | admin | `mergerAPIAdminEventosResumoFranquia` | routes_merger_core.go:162 |
| `/merger/api/admin/eventos/stats` | admin | `mergerAPIAdminEventosStats` | routes_merger_core.go:158 |
| `/merger/api/admin/eventos/stats_avancadas` | admin | `mergerAPIAdminEventosStatsAvancadas` | routes_merger_core.go:161 |
| `/merger/api/admin/eventos/tipos` | admin | `mergerAPIAdminEventosTipos` | routes_merger_core.go:159 |
| `/merger/api/admin/franquia_features` | admin | `mergerAPIAdminFeatureFlags` | routes_merger_core.go:128 |
| `/merger/api/admin/franquia_features/aplicar_todas` | admin | `mergerAPIAdminFeatureFlagsAplicarTodas` | routes_merger_core.go:132 |
| `/merger/api/admin/franquia_features/massa` | admin | `mergerAPIAdminFeatureFlagsMassa` | routes_merger_core.go:130 |
| `/merger/api/admin/franquia_features/regras` | admin | `mergerAPIAdminFranquiaRegras` | routes_merger_core.go:129 |
| `/merger/api/admin/franquia/max_telas` | admin | `mergerAPIAdminFranquiaMaxTelas` | routes_revendas.go:45 |
| `/merger/api/admin/gateway_config` | admin | `mergerAPIAdminGatewayConfig` | routes_pagamentos.go:43 |
| `/merger/api/admin/gateway_saude` | admin | `mergerAPIAdminGatewaySaude` | routes_pagamentos.go:46 |
| `/merger/api/admin/gateway_testar` | admin | `mergerAPIAdminGatewayTestar` | routes_pagamentos.go:44 |
| `/merger/api/admin/gerar_mensagem` | admin | `mergerAPIGerarMensagem` | routes_revendas.go:76 |
| `/merger/api/admin/health` | admin | `mergerAdminHealthHandler` | routes_whatsapp.go:154 |
| `/merger/api/admin/health/alertas` | admin | `mergerAdminHealthAlertasHandler` | routes_whatsapp.go:158 |
| `/merger/api/admin/health/backup` | admin | `mergerAdminHealthBackupForceHandler` | routes_whatsapp.go:156 |
| `/merger/api/admin/health/backup/download` | admin | `mergerAdminHealthBackupDownloadHandler` | routes_whatsapp.go:157 |
| `/merger/api/admin/health/backups` | admin | `mergerAdminHealthBackupsHandler` | routes_whatsapp.go:155 |
| `/merger/api/admin/historico/cliente` | admin | `histSoAdmin(historicoAPICliente)` | routes_historico.go:37 |
| `/merger/api/admin/historico/clientes` | admin | `histSoAdmin(historicoAPIClientes)` | routes_historico.go:36 |
| `/merger/api/admin/historico/correcao/corrigir` | admin | `histSoAdmin(historicoAPICorrecaoCorrigir)` | routes_historico.go:38 |
| `/merger/api/admin/historico/correcoes` | admin | `histSoAdmin(historicoAPICorrecoes)` | routes_historico.go:39 |
| `/merger/api/admin/historico/exportar` | admin | `histSoAdmin(historicoAPIExportar)` | routes_historico.go:40 |
| `/merger/api/admin/historico/top_clientes` | admin | `histSoAdmin(historicoAPITopClientes)` | routes_historico.go:41 |
| `/merger/api/admin/ia_afiliados` | admin | `mergerAdminIAAfiliadosListaHandler` | routes_whatsapp.go:140 |
| `/merger/api/admin/ia_afiliados/aprovar_saque` | admin | `mergerAdminIAAfiliadosAprovarSaqueHandler` | routes_whatsapp.go:142 |
| `/merger/api/admin/ia_afiliados/saques` | admin | `mergerAdminIAAfiliadosSaquesListaHandler` | routes_whatsapp.go:141 |
| `/merger/api/admin/ia_anomalias` | admin | `mergerAdminIAAnomaliasListaHandler` | routes_whatsapp.go:116 |
| `/merger/api/admin/ia_assinaturas` | admin | `mergerAdminIAAssinaturasListaHandler` | routes_whatsapp.go:126 |
| `/merger/api/admin/ia_cupons` | admin | `mergerAdminIACuponsListaHandler` | routes_whatsapp.go:131 |
| `/merger/api/admin/ia_cupons/delete` | admin | `mergerAdminIACuponsDeleteHandler` | routes_whatsapp.go:134 |
| `/merger/api/admin/ia_cupons/save` | admin | `mergerAdminIACuponsSaveHandler` | routes_whatsapp.go:132 |
| `/merger/api/admin/ia_cupons/toggle` | admin | `mergerAdminIACuponsToggleHandler` | routes_whatsapp.go:133 |
| `/merger/api/admin/ia_dashboard` | admin | `mergerAdminIADashboardHandler` | routes_whatsapp.go:114 |
| `/merger/api/admin/ia_free_tier` | admin | `mergerAdminIAFreeTierListaHandler` | routes_whatsapp.go:93 |
| `/merger/api/admin/ia_free_tier/bulk` | admin | `mergerAdminIAFreeTierBulkHandler` | routes_whatsapp.go:95 |
| `/merger/api/admin/ia_free_tier/set` | admin | `mergerAdminIAFreeTierSetHandler` | routes_whatsapp.go:94 |
| `/merger/api/admin/ia_pacotes_catalogo` | admin | `mergerAdminIAPacotesCatalogoListaHandler` | routes_whatsapp.go:109 |
| `/merger/api/admin/ia_pacotes_catalogo/delete` | admin | `mergerAdminIAPacotesCatalogoDeleteHandler` | routes_whatsapp.go:112 |
| `/merger/api/admin/ia_pacotes_catalogo/reset` | admin | `mergerAdminIAPacotesCatalogoResetHandler` | routes_whatsapp.go:113 |
| `/merger/api/admin/ia_pacotes_catalogo/save` | admin | `mergerAdminIAPacotesCatalogoSaveHandler` | routes_whatsapp.go:110 |
| `/merger/api/admin/ia_pacotes_catalogo/toggle` | admin | `mergerAdminIAPacotesCatalogoToggleHandler` | routes_whatsapp.go:111 |
| `/merger/api/admin/identidade/bootstrap` | admin | `identSoAdmin(identidadeAPIBootstrap)` | routes_identidade.go:38 |
| `/merger/api/admin/identidade/destriar` | admin | `identSoAdmin(identidadeAPIDestriar)` | routes_identidade.go:33 |
| `/merger/api/admin/identidade/emissao` | admin | `identSoAdmin(identidadeAPIEmissaoStatus)` | routes_identidade.go:37 |
| `/merger/api/admin/identidade/emissao/definir` | admin | `identSoAdmin(identidadeAPIEmissaoDefinir)` | routes_identidade.go:39 |
| `/merger/api/admin/identidade/ensaio` | admin | `identSoAdmin(identidadeAPIEnsaioListar)` | routes_identidade.go:25 |
| `/merger/api/admin/identidade/ensaio/criar` | admin | `identSoAdmin(identidadeAPIEnsaioCriar)` | routes_identidade.go:26 |
| `/merger/api/admin/identidade/ensaio/excluir` | admin | `identSoAdmin(identidadeAPIEnsaioExcluir)` | routes_identidade.go:27 |
| `/merger/api/admin/identidade/excluir` | admin | `identSoAdmin(identidadeAPIExcluirDeVerdade)` | routes_identidade.go:47 |
| `/merger/api/admin/identidade/manutencao` | admin | `identSoAdmin(identidadeAPIManutencaoListar)` | routes_identidade.go:46 |
| `/merger/api/admin/identidade/painel` | admin | `identSoAdmin(identidadePainelHTML)` | routes_identidade.go:30 |
| `/merger/api/admin/identidade/relatorio` | admin | `identSoAdmin(identidadeAPIRelatorio)` | routes_identidade.go:23 |
| `/merger/api/admin/identidade/resetar` | admin | `identSoAdmin(identidadeAPIResetar)` | routes_identidade.go:28 |
| `/merger/api/admin/identidade/rodar` | admin | `identSoAdmin(identidadeAPIRodar)` | routes_identidade.go:24 |
| `/merger/api/admin/identidade/triar` | admin | `identSoAdmin(identidadeAPITriar)` | routes_identidade.go:32 |
| `/merger/api/admin/identidade/unir` | admin | `identSoAdmin(identidadeAPIUnir)` | routes_identidade.go:31 |
| `/merger/api/admin/ip/consultar` | admin | `mergerAPIAdminIPConsultar` | routes_asn.go:48 |
| `/merger/api/admin/ip/liberados` | admin | `mergerAPIAdminIPLiberados` | routes_asn.go:52 |
| `/merger/api/admin/ip/liberar` | admin | `mergerAPIAdminIPLiberar` | routes_asn.go:53 |
| `/merger/api/admin/ip/revogar` | admin | `mergerAPIAdminIPRevogar` | routes_asn.go:54 |
| `/merger/api/admin/ips` | admin | `mergerAPIAdminIPs` | routes_revendas.go:83 |
| `/merger/api/admin/ips_ocultos/listar` | admin | `mergerAdminIPsOcultosListar` | routes_merger_core.go:60 |
| `/merger/api/admin/ips_ocultos/salvar` | admin | `mergerAdminIPsOcultosSalvar` | routes_merger_core.go:61 |
| `/merger/api/admin/ips/bloquear` | admin | `mergerAPIAdminIPBloquear` | routes_revendas.go:84 |
| `/merger/api/admin/ips/bloquear_manual` | admin | `mergerAPIAdminIPBloquearManual` | routes_revendas.go:87 |
| `/merger/api/admin/ips/desbloquear` | admin | `mergerAPIAdminIPDesbloquear` | routes_revendas.go:85 |
| `/merger/api/admin/ips/remover_cliente` | admin | `mergerAPIAdminIPRemoverCliente` | routes_revendas.go:86 |
| `/merger/api/admin/jogos` | admin | `lojaAPIAdminJogos` | routes_loja_pwa.go:48 |
| `/merger/api/admin/jogos_do_dia/adicionar_canal_extra` | admin | `mergerAPIAdminJogosManualCanalExtra` | routes_merger_core.go:203 |
| `/merger/api/admin/jogos_do_dia/agenda_disponivel` | admin | `mergerAPIAdminJogosManualAgendaDisponivel` | routes_merger_core.go:208 |
| `/merger/api/admin/jogos_do_dia/atualizar` | admin | `mergerAPIAdminJogosDoDiaAtualizar` | routes_merger_core.go:199 |
| `/merger/api/admin/jogos_do_dia/capa_custom` | admin | `mergerAPIAdminJogosManualCapaCustom` | routes_merger_core.go:205 |
| `/merger/api/admin/jogos_do_dia/criar_manual` | admin | `mergerAPIAdminJogosManualCriar` | routes_merger_core.go:201 |
| `/merger/api/admin/jogos_do_dia/ocultar` | admin | `mergerAPIAdminJogosManualOcultar` | routes_merger_core.go:207 |
| `/merger/api/admin/jogos_do_dia/regerar_capa` | admin | `mergerAPIAdminJogosManualRegerarCapa` | routes_merger_core.go:204 |
| `/merger/api/admin/jogos_do_dia/remover` | admin | `mergerAPIAdminJogosManualRemover` | routes_merger_core.go:206 |
| `/merger/api/admin/jogos_do_dia/trocar_canal` | admin | `mergerAPIAdminJogosManualTrocarCanal` | routes_merger_core.go:202 |
| `/merger/api/admin/jogos/auto-gerar` | admin | `mergerAPIAdminLojaJogosAutoGerar` | routes_loja_pwa.go:50 |
| `/merger/api/admin/limpeza_inativos/executar` | admin | `mergerAdminLimpezaInativosExecutar` | routes_merger_core.go:153 |
| `/merger/api/admin/limpeza_inativos/previa` | admin | `mergerAdminLimpezaInativosPrevia` | routes_merger_core.go:152 |
| `/merger/api/admin/links_quebrados` | admin | `mergerAPIAdminLinksQuebrados` | routes_catalogo.go:122 |
| `/merger/api/admin/links_quebrados/limpar` | admin | `mergerAPIAdminLinksQuebradosLimpar` | routes_catalogo.go:123 |
| `/merger/api/admin/loja/franquias` | admin | `lojaAPIAdminFranquias` | routes_loja_pwa.go:46 |
| `/merger/api/admin/loja/toggle-subrevenda` | admin | `lojaAPIAdminToggleSubrevenda` | routes_loja_pwa.go:45 |
| `/merger/api/admin/melhorias/confirmar` | admin | `melhoriasAPIConfirmarMestre` | routes_melhorias.go:23 |
| `/merger/api/admin/melhorias/listar` | admin | `melhoriasAPIListarAdmin` | routes_melhorias.go:21 |
| `/merger/api/admin/melhorias/preparar` | admin | `melhoriasAPIPrepararMestre` | routes_melhorias.go:22 |
| `/merger/api/admin/melhorias/rascunhos` | admin | `melhoriasAPIRascunhosMestre` | routes_melhorias.go:24 |
| `/merger/api/admin/melhorias/status` | admin | `melhoriasAPIStatus` | routes_melhorias.go:26 |
| `/merger/api/admin/melhorias/ver` | admin | `melhoriasAPIVerAdmin` | routes_melhorias.go:25 |
| `/merger/api/admin/mensalidade-curva` | admin | `conviteMensalistaBaseHandler` | routes_convite.go:30 |
| `/merger/api/admin/migrations` | admin | `mergerAPIAdminMigrations` | routes_merger_core.go:166 |
| `/merger/api/admin/mp_config` | admin | `mergerAPIAdminMPConfig` | routes_pagamentos.go:27 |
| `/merger/api/admin/msproapp_logos` | admin | `mergerAPIAdminMsproappLogos` | routes_msproapp.go:62 |
| `/merger/api/admin/oficial/conversas` | admin | `disparoSoAdmin(oficialAdminConversasHandler)` | routes_oficial.go:102 |
| `/merger/api/admin/oficial/extrato` | admin | `disparoSoAdmin(oficialAdminExtratoHandler)` | routes_oficial.go:96 |
| `/merger/api/admin/oficial/franquias` | admin | `disparoSoAdmin(oficialAdminFranquiasHandler)` | routes_oficial.go:108 |
| `/merger/api/admin/oficial/mensagens` | admin | `disparoSoAdmin(oficialAdminMensagensHandler)` | routes_oficial.go:103 |
| `/merger/api/admin/oficial/midia` | admin | `disparoSoAdmin(oficialAdminMidiaHandler)` | routes_oficial.go:104 |
| `/merger/api/admin/oficial/numero` | admin | `disparoSoAdmin(oficialAdminNumeroHandler)` | routes_oficial.go:89 |
| `/merger/api/admin/oficial/numeros` | admin | `disparoSoAdmin(oficialAdminNumerosHandler)` | routes_oficial.go:92 |
| `/merger/api/admin/oficial/numeros_waba` | admin | `disparoSoAdmin(oficialAdminNumerosWABAHandler)` | routes_oficial.go:91 |
| `/merger/api/admin/oficial/preco_disparo` | admin | `disparoSoAdmin(oficialAdminPrecoDisparoHandler)` | routes_oficial.go:98 |
| `/merger/api/admin/oficial/recarga` | admin | `disparoSoAdmin(oficialAdminRecargaHandler)` | routes_oficial.go:88 |
| `/merger/api/admin/oficial/recarga_mp` | admin | `disparoSoAdmin(oficialAdminRecargaMPHandler)` | routes_oficial.go:94 |
| `/merger/api/admin/oficial/responder` | admin | `disparoSoAdmin(oficialAdminResponderHandler)` | routes_oficial.go:105 |
| `/merger/api/admin/operadoras` | admin | `mergerAPIAdminOperadoras` | routes_revendas.go:90 |
| `/merger/api/admin/operadoras/adicionar` | admin | `mergerAPIAdminOperadoraAdicionar` | routes_revendas.go:91 |
| `/merger/api/admin/operadoras/remover` | admin | `mergerAPIAdminOperadoraRemover` | routes_revendas.go:92 |
| `/merger/api/admin/pedidos/creditos` | admin | `mergerAPIAdminPedidosCreditos` | routes_revendas.go:99 |
| `/merger/api/admin/planos` | admin | `mergerAPIAdminPlanos` | routes_revendas.go:51 |
| `/merger/api/admin/planos/toggle` | admin | `mergerAPIAdminPlanosToggle` | routes_revendas.go:52 |
| `/merger/api/admin/push/broadcast` | admin | `pwaPushBroadcastHandler` | routes_loja_pwa.go:79 |
| `/merger/api/admin/push/test` | admin | `pwaPushTestHandler` | routes_loja_pwa.go:78 |
| `/merger/api/admin/quem-e` | admin | `mergerAdminQuemEHandler` | routes_merger_core.go:148 |
| `/merger/api/admin/restaurar` | admin | `mergerAdminRestaurarHandler` | routes_merger_core.go:55 |
| `/merger/api/admin/revendas` | admin | `mergerAPIAdminRevendas` | routes_revendas.go:28 |
| `/merger/api/admin/revendas/arvore` | admin | `mergerAPIAdminRevendasArvore` | routes_revendas.go:43 |
| `/merger/api/admin/revendas/cobranca` | admin | `mergerAPIAdminRevendasCobranca` | routes_revendas.go:35 |
| `/merger/api/admin/revendas/edit` | admin | `mergerAPIAdminRevendasEdit` | routes_revendas.go:29 |
| `/merger/api/admin/revendas/perms` | admin | `mergerAPIAdminRevendasPerms` | routes_revendas.go:31 |
| `/merger/api/admin/revendas/remove` | admin | `mergerAPIAdminRevendasRemove` | routes_revendas.go:32 |
| `/merger/api/admin/revendas/renovar` | admin | `mergerAPIAdminRevendasRenovarManual` | routes_revendas.go:34 |
| `/merger/api/admin/revendas/sigma_sync` | admin | `mergerAPIAdminRevendasSigmaSync` | routes_sigma.go:130 |
| `/merger/api/admin/revendas/toggle` | admin | `mergerAPIAdminRevendasToggle` | routes_revendas.go:30 |
| `/merger/api/admin/revendas/vencimento` | admin | `mergerAPIAdminRevendasVencimento` | routes_revendas.go:33 |
| `/merger/api/admin/saques_aprovar` | admin | `mergerAPIAdminSaquesAprovar` | routes_telegram.go:61 |
| `/merger/api/admin/saques_listar` | admin | `mergerAPIAdminSaquesListar` | routes_telegram.go:59 |
| `/merger/api/admin/saques_page` | admin | `mergerAPIAdminSaquesPage` | routes_telegram.go:63 |
| `/merger/api/admin/saques_rejeitar` | admin | `mergerAPIAdminSaquesRejeitar` | routes_telegram.go:62 |
| `/merger/api/admin/saques_stats` | admin | `mergerAPIAdminSaquesStats` | routes_telegram.go:60 |
| `/merger/api/admin/servidores` | admin | `mergerAPIAdminServidoresListar` | routes_servidores.go:16 |
| `/merger/api/admin/servidores/apagar` | admin | `mergerAPIAdminServidorApagar` | routes_servidores.go:19 |
| `/merger/api/admin/servidores/criar` | admin | `mergerAPIAdminServidorCriar` | routes_servidores.go:17 |
| `/merger/api/admin/servidores/editar` | admin | `mergerAPIAdminServidorEditar` | routes_servidores.go:18 |
| `/merger/api/admin/servidores/franquias` | admin | `mergerAPIAdminServidorFranquias` | routes_servidores.go:21 |
| `/merger/api/admin/servidores/liberar` | admin | `mergerAPIAdminServidorLiberar` | routes_servidores.go:22 |
| `/merger/api/admin/servidores/padrao` | admin | `mergerAPIAdminServidorPadrao` | routes_servidores.go:24 |
| `/merger/api/admin/servidores/revogar` | admin | `mergerAPIAdminServidorRevogar` | routes_servidores.go:23 |
| `/merger/api/admin/servidores/troca` | admin | `mergerAPIAdminServidorTroca` | routes_servidores.go:25 |
| `/merger/api/admin/sigma_corretor/aplicar` | admin | `mergerAdminSigmaCorretorAplicar` | routes_sigma.go:141 |
| `/merger/api/admin/sigma_corretor/preview` | admin | `mergerAdminSigmaCorretorPreview` | routes_sigma.go:140 |
| `/merger/api/admin/sigma_corretor/reverter` | admin | `mergerAdminSigmaCorretorReverter` | routes_sigma.go:142 |
| `/merger/api/admin/sigma_corretor/runs` | admin | `mergerAdminSigmaCorretorRuns` | routes_sigma.go:143 |
| `/merger/api/admin/sigma_corretor/servidores` | admin | `mergerAdminSigmaCorretorServidores` | routes_sigma.go:139 |
| `/merger/api/admin/sigma_diag/capturar` | admin | `mergerAPIAdminSigmaDiagCapturar` | routes_sigma.go:133 |
| `/merger/api/admin/sigma_diag/detalhe` | admin | `mergerAPIAdminSigmaDiagDetalhe` | routes_sigma.go:136 |
| `/merger/api/admin/sigma_diag/historico` | admin | `mergerAPIAdminSigmaDiagHistorico` | routes_sigma.go:135 |
| `/merger/api/admin/sigma_diag/inspecionar_linha` | admin | `mergerAPIAdminSigmaDiagInspecionar` | routes_sigma.go:134 |
| `/merger/api/admin/sigma-actions-catalog` | admin | `mergerAPIAdminSigmaActionsCatalog` | routes_sigma.go:132 |
| `/merger/api/admin/sync-log` | admin | `mergerAPIAdminSyncLog` | routes_sigma.go:131 |
| `/merger/api/admin/templates` | admin | `mergerAPIAdminTemplates` | routes_revendas.go:55 |
| `/merger/api/admin/tg_canal` | admin | `func(w http.ResponseWriter, r *http.Request) { if r.Method == http.MethodPost { mergerAPIAdminTGCanalSet(w, r` | routes_telegram.go:40 |
| `/merger/api/admin/tickets/listar` | admin | `ticketsAPIListarAdmin` | routes_tickets.go:33 |
| `/merger/api/admin/tickets/responder` | admin | `ticketsAPIResponderAdmin` | routes_tickets.go:35 |
| `/merger/api/admin/tickets/status` | admin | `ticketsAPIStatusAdmin` | routes_tickets.go:36 |
| `/merger/api/admin/tickets/ver` | admin | `ticketsAPIVerAdmin` | routes_tickets.go:34 |
| `/merger/api/admin/transfer_posse` | admin | `mergerAdminTransferPosseHandler` | routes_sigma.go:76 |
| `/merger/api/admin/transfer_posse/linhas` | admin | `mergerAdminTransferPosseLinhasHandler` | routes_sigma.go:77 |
| `/merger/api/admin/transfer_revenda` | admin | `mergerAdminTransferRevendaHandler` | routes_sigma.go:82 |
| `/merger/api/admin/transfer_revenda/destinos` | admin | `mergerAdminTransferRevendaDestinosHandler` | routes_sigma.go:83 |
| `/merger/api/admin/usuarios/revendas` | admin | `adminUsuariosRevendasHandler` | routes_xtream_admin.go:29 |
| `/merger/api/admin/wa_config` | admin | `mergerAdminWhatsAppConfigHandler` | routes_whatsapp.go:29 |
| `/merger/api/admin/wa_ia/config` | admin | `mergerAdminWAIAConfigHandler` | routes_whatsapp.go:77 |
| `/merger/api/admin/wa_ia/correcoes/acao` | admin | `mergerAdminWAIACorrecoesAcaoHandler` | routes_whatsapp.go:81 |
| `/merger/api/admin/wa_ia/extrair_padroes` | admin | `mergerAdminWAIAExtrairPadroesHandler` | routes_whatsapp.go:80 |
| `/merger/api/admin/wa_ia/simular_chat` | admin | `mergerAdminWAIASimularChatHandler` | routes_whatsapp.go:79 |
| `/merger/api/admin/wa_ia/testar` | admin | `mergerAdminWAIATestarHandler` | routes_whatsapp.go:78 |
| `/merger/api/admin/wa_menu` | admin | `mergerAdminWhatsAppMenuHandler` | routes_whatsapp.go:39 |
| `/merger/api/admin/webplayer/franquias` | admin | `mergerAPIAdminWebplayerFranquias` | routes_webplayer.go:24 |
| `/merger/api/admin/webplayer/toggle-subrevenda` | admin | `mergerAPIAdminWebplayerToggleSub` | routes_webplayer.go:23 |

## api

| Rota | Metodo/escopo | Handler | Fonte |
|---|---|---|---|
| `/api/cadastro-adesao/gerar` | sessao | `conviteAdesaoGerarHandler` | routes_convite.go:24 |
| `/api/cadastro-adesao/status` | sessao | `conviteAdesaoStatusHandler` | routes_convite.go:25 |
| `/api/cadastro/` | sessao | `conviteAPIRouter` | routes_convite.go:23 |
| `/api/chatbot/` | sessao | `botTestePublicoHandler` | routes_botteste.go:25 |
| `/api/mcp` | sessao | `mcpHandler` | routes_merger_core.go:96 |
| `/api/painel/delete` | sessao | `apiDeletePainelHandler` | routes_xtream_admin.go:35 |
| `/api/renovar/` | sessao | `renovarAPIRouter` | routes_renovar.go:31 |
| `/api/tg/admin/webhook` | sessao | `telegramAdminWebhookHandler` | routes_telegram.go:127 |
| `/api/tg/webhook/` | sessao | `telegramWebhookHandler` | routes_telegram.go:119 |
| `/api/usuario/criar` | sessao | `apiCriarUsuarioHandler` | routes_xtream_admin.go:32 |
| `/api/usuario/delete` | sessao | `apiDeleteHandler` | routes_xtream_admin.go:31 |
| `/api/usuario/limpar-expirados` | sessao | `apiDeleteExpiredHandler` | routes_xtream_admin.go:34 |
| `/api/usuario/renovar` | sessao | `apiRenovarUsuarioHandler` | routes_xtream_admin.go:33 |
| `/api/usuario/toggle` | sessao | `apiToggleHandler` | routes_xtream_admin.go:30 |
| `/api/usuarios` | sessao | `apiUsuariosHandler` | routes_xtream_admin.go:28 |
| `/api/wa/admin/webhook` | sessao | `whatsappAdminWebhookHandler` | routes_whatsapp.go:191 |
| `/api/wa/webhook` | sessao | `whatsappWebhookHandler` | routes_whatsapp.go:183 |
| `/api/webhook/apps_ativa/` | jwt | `appsAtivaWebhookHandler` | routes_apps_ativa.go:54 |
| `/api/webhook/customer` | jwt | `webhookCORS(webhookCustomer)` | routes_webhook.go:51 |
| `/api/webhook/customer/adultos` | jwt | `webhookCORS(webhookCustomerAdultos)` | routes_webhook.go:55 |
| `/api/webhook/customer/create` | jwt | `webhookCORS(webhookCustomerCreate)` | routes_webhook.go:52 |
| `/api/webhook/customer/renew` | jwt | `webhookCORS(webhookCustomerRenew)` | routes_webhook.go:53 |
| `/api/webhook/customer/status` | jwt | `webhookCORS(webhookCustomerStatus)` | routes_webhook.go:54 |
| `/api/webhook/package` | jwt | `webhookCORS(webhookPackageList)` | routes_webhook.go:57 |
| `/api/webhook/user` | jwt | `webhookCORS(webhookUserList)` | routes_webhook.go:58 |

## lb

| Rota | Metodo/escopo | Handler | Fonte |
|---|---|---|---|
| `/lb/sync/canal24h` | sessao | `lbSyncCanal24hHandler` | routes_catalogo.go:168 |
| `/lb/sync/canal24h_reset` | sessao | `lbAPICanal24hReset) // ↺ reset de hub vindo do Main (2026-07-10` | routes_catalogo.go:169 |
| `/merger/api/lb/add` | sessao | `mergerAPILBAdd` | routes_catalogo.go:152 |
| `/merger/api/lb/canal/assign` | sessao | `mergerAPILBCanalAssign` | routes_catalogo.go:164 |
| `/merger/api/lb/canal/assignments` | sessao | `mergerAPILBCanalAssignments` | routes_catalogo.go:165 |
| `/merger/api/lb/canal/distribute` | sessao | `mergerAPILBCanalDistribute` | routes_catalogo.go:166 |
| `/merger/api/lb/deploy` | sessao | `mergerAPILBDeploy` | routes_catalogo.go:156 |
| `/merger/api/lb/deploy_todos` | sessao | `mergerAPILBDeployTodos) // 🚀 1 clique, todos os LBs (2026-07-11` | routes_catalogo.go:157 |
| `/merger/api/lb/deploy_todos/progresso` | sessao | `mergerAPILBDeployTodosProgresso) // progresso do job http.HandleFunc("/merger/api/lb/route", mergerAPILBSetRoute` | routes_catalogo.go:158 |
| `/merger/api/lb/edit` | sessao | `mergerAPILBEdit` | routes_catalogo.go:153 |
| `/merger/api/lb/list` | sessao | `mergerAPILBList` | routes_catalogo.go:151 |
| `/merger/api/lb/reboot` | sessao | `mergerAPILBReboot` | routes_catalogo.go:161 |
| `/merger/api/lb/remove` | sessao | `mergerAPILBRemove` | routes_catalogo.go:154 |
| `/merger/api/lb/routes` | sessao | `mergerAPILBGetRoutes` | routes_catalogo.go:160 |
| `/merger/api/lb/soft_restart` | sessao | `mergerAPILBSoftRestart` | routes_catalogo.go:162 |
| `/merger/api/lb/toggle` | sessao | `mergerAPILBToggle` | routes_catalogo.go:155 |

## creditos-pacotes

| Rota | Metodo/escopo | Handler | Fonte |
|---|---|---|---|
| `/merger/api/creditos-pacotes` | sessao | `featureGate("creditos_comprar", creditosPacotesAPI)` | routes_pagamentos.go:54 |
| `/merger/api/creditos-pacotes/cancelar` | sessao | `featureGate("creditos_comprar", creditosPacotesCancelar)` | routes_pagamentos.go:61 |
| `/merger/api/creditos-pacotes/comprar` | sessao | `featureGate("creditos_comprar", creditosPacotesComprar)` | routes_pagamentos.go:57 |
| `/merger/api/creditos-pacotes/comprovante` | sessao | `featureGate("creditos_comprar", mergerAPIComprovanteUpload)` | routes_pagamentos.go:68 |
| `/merger/api/creditos-pacotes/comprovante-ver` | sessao | `featureGate("creditos_comprar", mergerAPIComprovanteServir)` | routes_pagamentos.go:70 |
| `/merger/api/creditos-pacotes/confirmar-manual` | sessao | `featureGate("creditos_comprar", mergerAPIConfirmarManual)` | routes_pagamentos.go:72 |
| `/merger/api/creditos-pacotes/historico` | sessao | `featureGate("creditos_comprar", creditosPacotesHistorico)` | routes_pagamentos.go:59 |
| `/merger/api/creditos-pacotes/importar-templates` | sessao | `featureGate("creditos_comprar", creditosPacotesImportar)` | routes_pagamentos.go:62 |
| `/merger/api/creditos-pacotes/meu-preco` | sessao | `featureGate("creditos_comprar", creditosPacotePrecoSalvar)` | routes_pagamentos.go:64 |
| `/merger/api/creditos-pacotes/pedido` | sessao | `featureGate("creditos_comprar", creditosPacotesPedido)` | routes_pagamentos.go:60 |
| `/merger/api/creditos-pacotes/pendentes-vendedor` | sessao | `featureGate("creditos_comprar", mergerAPIPendentesVendedor)` | routes_pagamentos.go:74 |
| `/merger/api/creditos-pacotes/rejeitar-manual` | sessao | `featureGate("creditos_comprar", mergerAPIRejeitarManual)` | routes_pagamentos.go:73 |
| `/merger/api/creditos-pacotes/remover` | sessao | `featureGate("creditos_comprar", creditosPacotesRemover)` | routes_pagamentos.go:55 |
| `/merger/api/creditos-pacotes/reordenar` | sessao | `featureGate("creditos_comprar", creditosPacotesReordenar)` | routes_pagamentos.go:56 |
| `/merger/api/creditos-pacotes/status` | sessao | `featureGate("creditos_comprar", creditosPacotesStatus)` | routes_pagamentos.go:58 |

## tmdb

| Rota | Metodo/escopo | Handler | Fonte |
|---|---|---|---|
| `/merger/api/tmdb/apply` | sessao | `mergerAPITMDBApply` | routes_catalogo.go:175 |
| `/merger/api/tmdb/apply_episode` | sessao | `mergerAPITMDBApplyEpisode` | routes_catalogo.go:177 |
| `/merger/api/tmdb/batch_fill` | sessao | `mergerAPITMDBBatchFill` | routes_catalogo.go:179 |
| `/merger/api/tmdb/batch_status` | sessao | `mergerAPITMDBBatchStatus` | routes_catalogo.go:178 |
| `/merger/api/tmdb/config` | sessao | `mergerAPITMDBConfig` | routes_catalogo.go:172 |
| `/merger/api/tmdb/match` | sessao | `mergerAPITMDBMatch` | routes_catalogo.go:176 |
| `/merger/api/tmdb/search` | sessao | `mergerAPITMDBSearch` | routes_catalogo.go:173 |
| `/merger/api/tmdb/sem_capa` | sessao | `mergerAPITMDBSemCapa` | routes_catalogo.go:180 |
| `/merger/api/tmdb/series_episodes` | sessao | `mergerAPITMDBSeriesEpisodes` | routes_catalogo.go:174 |

## merger

| Rota | Metodo/escopo | Handler | Fonte |
|---|---|---|---|
| `/merger` | sessao | `mergerPainelPage` | routes_merger_core.go:26 |
| `/merger/cliente/sigma-remotos` | sessao | `featureGate("sigma_remotos", mergerClienteSigmaRemotosUI)` | routes_sigma.go:127 |
| `/merger/static/merger_panel.css` | sessao | `servirMergerPainelCSS` | routes_merger_core.go:28 |
| `/merger/static/merger_panel.js` | sessao | `servirMergerPainelJS` | routes_merger_core.go:29 |
| `/merger/webhook/convite-adesao` | sessao | `conviteAdesaoWebhook` | routes_convite.go:26 |
| `/merger/webhook/creditos-pacotes` | sessao | `creditosPacotesWebhook` | routes_pagamentos.go:80 |
| `/merger/webhook/mensalidade` | sessao | `mensalidadeWebhook` | routes_pagamentos.go:95 |

## static

| Rota | Metodo/escopo | Handler | Fonte |
|---|---|---|---|
| `/static/aparencia_fundo.js` | sessao | `servirAparenciaFundoJS` | routes_merger_core.go:38 |
| `/static/aparencia_login.css` | sessao | `servirAparenciaLoginCSS` | routes_merger_core.go:52 |
| `/static/aparencia_login.js` | sessao | `servirAparenciaLoginJS` | routes_merger_core.go:51 |
| `/static/copa_fundo.js` | sessao | `servirCopaFundoJS` | routes_merger_core.go:36 |
| `/static/file_custom.js` | sessao | `servirFileCustomJS` | routes_merger_core.go:34 |
| `/static/select_custom.js` | sessao | `servirSelectCustomJS` | routes_merger_core.go:32 |
| `/static/tooltip_custom.js` | sessao | `servirTooltipCustomJS` | routes_merger_core.go:33 |

## stats

| Rota | Metodo/escopo | Handler | Fonte |
|---|---|---|---|
| `/merger/api/stats` | sessao | `mergerPainelAPIStats` | routes_catalogo.go:95 |
| `/stats` | sessao | `sigmaStatsHandler) // Painel admin usa fetch('/stats') pra dados dos painéis Sigma http.HandleFunc("/stats/system", statsHandler) // Stats de sistema (CPU, RAM, goroutines` | routes_xtream_admin.go:37 |
| `/stats/detailed` | sessao | `statsDetailedHandler` | routes_xtream_admin.go:39 |
| `/stats/extended` | sessao | `statsExtendedHandler` | routes_xtream_admin.go:40 |
| `/stats/simple` | sessao | `statsSimpleHandler` | routes_xtream_admin.go:41 |

## canal_24h

| Rota | Metodo/escopo | Handler | Fonte |
|---|---|---|---|
| `/merger/api/canal_24h/auto_failover` | sessao | `mergerAPICanal24hAutoFailover) // 🛡️ toggle do vigia (2026-07-10` | routes_catalogo.go:140 |
| `/merger/api/canal_24h/kill` | sessao | `mergerAPICanal24hKill` | routes_catalogo.go:137 |
| `/merger/api/canal_24h/orfaos` | sessao | `mergerAPICanal24hOrfaos` | routes_catalogo.go:141 |
| `/merger/api/canal_24h/reset` | sessao | `mergerAPICanal24hReset) // ↺ botão do dashboard (2026-07-10` | routes_catalogo.go:138 |
| `/merger/api/canal_24h/trocar_fonte` | sessao | `mergerAPICanal24hTrocarFonte) // 🛡️ botões P/1/2 do dashboard (2026-07-10` | routes_catalogo.go:139 |

## pix

| Rota | Metodo/escopo | Handler | Fonte |
|---|---|---|---|
| `/merger/api/pix/gerar` | sessao | `mergerAPIPixGerar` | routes_pagamentos.go:28 |
| `/merger/api/pix/historico` | sessao | `mergerAPIPixHistorico` | routes_pagamentos.go:47 |
| `/merger/api/pix/pendente` | sessao | `mergerAPIPixPendente` | routes_pagamentos.go:48 |
| `/merger/api/pix/status` | sessao | `mergerAPIPixStatus` | routes_pagamentos.go:29 |
| `/merger/api/pix/webhook` | sessao | `mergerAPIPixWebhook` | routes_pagamentos.go:30 |

## push

| Rota | Metodo/escopo | Handler | Fonte |
|---|---|---|---|
| `/merger/api/push/log_received` | sessao | `pwaPushReceivedHandler` | routes_loja_pwa.go:77 |
| `/merger/api/push/subscribe` | sessao | `pwaSubscribeHandler` | routes_loja_pwa.go:75 |
| `/merger/api/push/unsubscribe` | sessao | `pwaUnsubscribeHandler` | routes_loja_pwa.go:76 |
| `/merger/api/push/vapid_public` | sessao | `pwaVapidPublicHandler` | routes_loja_pwa.go:74 |

## login

| Rota | Metodo/escopo | Handler | Fonte |
|---|---|---|---|
| `/merger/api/login` | publico | `mergerLoginAPIHandler` | routes_merger_core.go:39 |
| `/merger/api/login/assistindo` | publico | `loginAssistindoHandler` | routes_merger_core.go:50 |
| `/merger/api/login/jogos` | publico | `loginJogosHandler` | routes_merger_core.go:49 |
| `/merger/api/login/vitrine` | publico | `loginVitrineHandler` | routes_merger_core.go:45 |

## config

| Rota | Metodo/escopo | Handler | Fonte |
|---|---|---|---|
| `/merger/api/config/fontes` | sessao | `mergerAPIConfigFontes` | routes_merger_core.go:119 |
| `/merger/api/config/geral` | sessao | `mergerAPIConfigGeral` | routes_merger_core.go:118 |
| `/merger/api/config/regras` | sessao | `mergerAPIConfigRegras` | routes_merger_core.go:120 |
| `/merger/api/config/regras/delete_orfa` | sessao | `mergerAPIConfigRegrasDeleteOrfa` | routes_merger_core.go:121 |

## portal

| Rota | Metodo/escopo | Handler | Fonte |
|---|---|---|---|
| `/portal/marca-logo/` | sessao | `oryxServirLogoRevenda` | routes_oryx_publico.go:73 |
| `/portal/pagar/` | sessao | `oryxServirPaginaPagamento` | routes_oryx_publico.go:71 |
| `/portal/qr/` | sessao | `oryxServirQRCobranca` | routes_oryx_publico.go:72 |
| `/portal/static/logo.png` | sessao | `oryxServirLogo` | routes_oryx_publico.go:74 |

## pedidos

| Rota | Metodo/escopo | Handler | Fonte |
|---|---|---|---|
| `/merger/api/pedidos/creditos/transferir` | sessao | `featureGate("pedidos_conteudo", mergerAPIPedidosCreditosTransferir)` | routes_revendas.go:98 |
| `/merger/api/pedidos/criar` | sessao | `featureGate("pedidos_conteudo", mergerAPIPedidosCriar)` | routes_revendas.go:95 |
| `/merger/api/pedidos/listar` | sessao | `featureGate("pedidos_conteudo", mergerAPIPedidosListar)` | routes_revendas.go:96 |
| `/merger/api/pedidos/status` | sessao | `featureGate("pedidos_conteudo", mergerAPIPedidosStatus)` | routes_revendas.go:97 |

## fonte_reserva

| Rota | Metodo/escopo | Handler | Fonte |
|---|---|---|---|
| `/merger/api/fonte_reserva/canais` | sessao | `mergerAPIFonteReservaCanais` | routes_catalogo.go:119 |
| `/merger/api/fonte_reserva/sincronizar` | sessao | `mergerAPIFonteReservaSincronizar` | routes_catalogo.go:118 |
| `/merger/api/fonte_reserva/vincular` | sessao | `mergerAPIFonteReservaVincular` | routes_catalogo.go:120 |

## loja

| Rota | Metodo/escopo | Handler | Fonte |
|---|---|---|---|
| `/loja/api/` | sessao | `lojaAPIRouter` | routes_loja_pwa.go:38 |
| `/loja/webhook/pix` | sessao | `lojaWebhookPIX` | routes_loja_pwa.go:63 |
| `/merger/api/loja/` | sessao | `featureGate("loja", lojaAPIPainelRouter)` | routes_loja_pwa.go:44 |

## reportes

| Rota | Metodo/escopo | Handler | Fonte |
|---|---|---|---|
| `/merger/api/reportes/criar` | sessao | `featureGate("reportes", mergerAPIReportesCriar)` | routes_revendas.go:102 |
| `/merger/api/reportes/listar` | sessao | `featureGate("reportes", mergerAPIReportesListar)` | routes_revendas.go:103 |
| `/merger/api/reportes/status` | sessao | `featureGate("reportes", mergerAPIReportesStatus)` | routes_revendas.go:104 |

## link_reserva

| Rota | Metodo/escopo | Handler | Fonte |
|---|---|---|---|
| `/merger/api/link_reserva` | sessao | `mergerPainelAPILinkReserva` | routes_catalogo.go:115 |
| `/merger/api/link_reserva/aplicar` | sessao | `mergerPainelAPILinkReservaAplicar` | routes_catalogo.go:116 |

## conexoes

| Rota | Metodo/escopo | Handler | Fonte |
|---|---|---|---|
| `/merger/api/conexoes/config` | sessao | `featureGate("conexoes_ativas", mergerAPIConexoesConfig)` | routes_catalogo.go:148 |
| `/merger/api/conexoes/listar` | sessao | `featureGate("conexoes_ativas", mergerAPIConexoesListar)` | routes_catalogo.go:147 |

## avisos

| Rota | Metodo/escopo | Handler | Fonte |
|---|---|---|---|
| `/merger/api/avisos` | sessao | `avisosAPIListar` | routes_loja_pwa.go:83 |
| `/merger/api/avisos/lida` | sessao | `avisosAPIMarcarLida` | routes_loja_pwa.go:84 |

## setup

| Rota | Metodo/escopo | Handler | Fonte |
|---|---|---|---|
| `/merger/api/setup` | publico | `mergerSetupAPIHandler` | routes_merger_core.go:40 |
| `/merger/api/setup/restaurar` | publico | `mergerSetupRestaurarHandler` | routes_merger_core.go:54 |

## theme

| Rota | Metodo/escopo | Handler | Fonte |
|---|---|---|---|
| `/merger/api/theme` | sessao | `featureGate("branding_aparencia", themeAPIHandler)` | routes_merger_core.go:107 |
| `/merger/api/theme/logo` | sessao | `featureGate("branding_aparencia", themeLogoHandler)` | routes_merger_core.go:108 |

## aparencia

| Rota | Metodo/escopo | Handler | Fonte |
|---|---|---|---|
| `/merger/api/aparencia` | sessao | `mergerAPIAparencia` | routes_merger_core.go:112 |
| `/merger/api/aparencia/imagem` | sessao | `aparenciaImagemHandler` | routes_merger_core.go:113 |

## sigma

| Rota | Metodo/escopo | Handler | Fonte |
|---|---|---|---|
| `/merger/api/sigma/paineis` | sessao | `mergerAPISigmaPaineis` | routes_merger_core.go:122 |
| `/sigma/` | sessao | `sigmaRevendaHandler` | routes_sigma.go:38 |

## renovacao_painel

| Rota | Metodo/escopo | Handler | Fonte |
|---|---|---|---|
| `/merger/api/renovacao_painel/pagar-login` | sessao | `renovacaoPainelPagarLogin` | routes_pagamentos.go:100 |
| `/merger/api/renovacao_painel/status-login` | sessao | `renovacaoPainelStatusLogin` | routes_pagamentos.go:101 |

## r

| Rota | Metodo/escopo | Handler | Fonte |
|---|---|---|---|
| `/r/` | sessao | `renovarPaginaHandler` | routes_renovar.go:26 |
| `/r/qr/` | sessao | `renovarQRPublicoHandler` | routes_renovar.go:30 |

## webplayer

| Rota | Metodo/escopo | Handler | Fonte |
|---|---|---|---|
| `/merger/api/webplayer/config` | sessao | `featureGate("webplayer", mergerAPIWebplayerConfig)` | routes_webplayer.go:22 |
| `/webplayer/api/vitrine` | sessao | `loginVitrineHandler` | routes_webplayer.go:17 |

## (raiz)

| Rota | Metodo/escopo | Handler | Fonte |
|---|---|---|---|
| `/` | sessao | `playHandler)` é // SEMPRE o último registro — qualquer rota nova adicionada depois // vira sombra silenciosa. Por isso ele mora aqui no orquestrador, // não dentro de um routes_*.go específico. // // Quando criar rota nova: // 1. Identificar o subdomínio (sigma, revendas, pagamentos, ...). // 2. Adicionar 1 linha em routes_<subdominio>.go. // 3. NÃO mexer aqui salvo se for um subdomínio totalmente novo // (aí cria routes_<novo>.go + adiciona 1 linha em registrarRotas). // // Subdivisão atual (Lote 6 — refactor 2026-05-07): // // routes_xtream_admin.go → Xtream público + /painel + /stats // routes_merger_core.go → auth + theme + config + MCP + ui_hide // routes_sigma.go → Sigma in/out + Maxplayer // routes_revendas.go → revendas + planos + IPs + pedidos // routes_pagamentos.go → PIX + créditos-pacotes // routes_catalogo.go → sync + canais + 24h + LB + TMDB + Spy // routes_loja_pwa.go → loja pública + PWA + push + avisos // routes_webhook.go → /api/webhook/* (Sigma compat layer` | routes.go:15 |

## apps_ativa

| Rota | Metodo/escopo | Handler | Fonte |
|---|---|---|---|
| `/merger/api/apps_ativa/aplicativos` | sessao | `mergerAppsAtivaAplicativos` | routes_apps_ativa.go:30 |

## available_categories

| Rota | Metodo/escopo | Handler | Fonte |
|---|---|---|---|
| `/merger/api/available_categories` | sessao | `mergerAPIAvailableCategories` | routes_catalogo.go:74 |

## sync_parcial

| Rota | Metodo/escopo | Handler | Fonte |
|---|---|---|---|
| `/merger/api/sync_parcial` | sessao | `mergerPainelAPISyncParcial` | routes_catalogo.go:94 |

## dashboard_financeiro

| Rota | Metodo/escopo | Handler | Fonte |
|---|---|---|---|
| `/merger/api/dashboard_financeiro` | sessao | `featureGate("dashboard", mergerDashboardFinanceiro)` | routes_catalogo.go:96 |

## search

| Rota | Metodo/escopo | Handler | Fonte |
|---|---|---|---|
| `/merger/api/search` | sessao | `mergerPainelAPISearch` | routes_catalogo.go:97 |

## duplicados

| Rota | Metodo/escopo | Handler | Fonte |
|---|---|---|---|
| `/merger/api/duplicados` | sessao | `mergerPainelAPIDuplicados` | routes_catalogo.go:98 |

## bloquear

| Rota | Metodo/escopo | Handler | Fonte |
|---|---|---|---|
| `/merger/api/bloquear` | sessao | `mergerPainelAPIBloquear` | routes_catalogo.go:101 |

## desbloquear

| Rota | Metodo/escopo | Handler | Fonte |
|---|---|---|---|
| `/merger/api/desbloquear` | sessao | `mergerPainelAPIDesbloquear` | routes_catalogo.go:102 |

## bloquear_todos_duplicados

| Rota | Metodo/escopo | Handler | Fonte |
|---|---|---|---|
| `/merger/api/bloquear_todos_duplicados` | sessao | `mergerPainelAPIBloquearTodosDuplicados` | routes_catalogo.go:103 |

## desbloquear_todos_duplicados

| Rota | Metodo/escopo | Handler | Fonte |
|---|---|---|---|
| `/merger/api/desbloquear_todos_duplicados` | sessao | `mergerPainelAPIDesbloquearTodosDuplicados` | routes_catalogo.go:104 |

## categorias

| Rota | Metodo/escopo | Handler | Fonte |
|---|---|---|---|
| `/merger/api/categorias` | sessao | `mergerPainelAPICategorias` | routes_catalogo.go:107 |

## canais_cat

| Rota | Metodo/escopo | Handler | Fonte |
|---|---|---|---|
| `/merger/api/canais_cat` | sessao | `mergerPainelAPICanaisDaCategoria` | routes_catalogo.go:108 |

## bloquear_cat

| Rota | Metodo/escopo | Handler | Fonte |
|---|---|---|---|
| `/merger/api/bloquear_cat` | sessao | `mergerPainelAPIBloquearCat` | routes_catalogo.go:109 |

## desbloquear_cat

| Rota | Metodo/escopo | Handler | Fonte |
|---|---|---|---|
| `/merger/api/desbloquear_cat` | sessao | `mergerPainelAPIDesbloquearCat` | routes_catalogo.go:110 |

## editar_icone

| Rota | Metodo/escopo | Handler | Fonte |
|---|---|---|---|
| `/merger/api/editar_icone` | sessao | `mergerPainelAPIEditarIcone` | routes_catalogo.go:111 |

## renomear

| Rota | Metodo/escopo | Handler | Fonte |
|---|---|---|---|
| `/merger/api/renomear` | sessao | `mergerPainelAPIRenomear` | routes_catalogo.go:112 |

## editar_url

| Rota | Metodo/escopo | Handler | Fonte |
|---|---|---|---|
| `/merger/api/editar_url` | sessao | `mergerPainelAPIEditarURL` | routes_catalogo.go:113 |

## episodios

| Rota | Metodo/escopo | Handler | Fonte |
|---|---|---|---|
| `/merger/api/episodios` | sessao | `mergerPainelAPIEpisodios` | routes_catalogo.go:124 |

## original_url

| Rota | Metodo/escopo | Handler | Fonte |
|---|---|---|---|
| `/merger/api/original_url` | sessao | `mergerPainelAPIOriginalURL` | routes_catalogo.go:125 |

## server_stats

| Rota | Metodo/escopo | Handler | Fonte |
|---|---|---|---|
| `/merger/api/server_stats` | sessao | `mergerPainelAPIServerStats` | routes_catalogo.go:126 |

## direto_on

| Rota | Metodo/escopo | Handler | Fonte |
|---|---|---|---|
| `/merger/api/direto_on` | sessao | `mergerPainelAPIDiretoOn` | routes_catalogo.go:129 |

## direto_off

| Rota | Metodo/escopo | Handler | Fonte |
|---|---|---|---|
| `/merger/api/direto_off` | sessao | `mergerPainelAPIDiretoOff` | routes_catalogo.go:130 |

## 24h_on

| Rota | Metodo/escopo | Handler | Fonte |
|---|---|---|---|
| `/merger/api/24h_on` | sessao | `mergerPainelAPI24hOn` | routes_catalogo.go:133 |

## 24h_off

| Rota | Metodo/escopo | Handler | Fonte |
|---|---|---|---|
| `/merger/api/24h_off` | sessao | `mergerPainelAPI24hOff` | routes_catalogo.go:134 |

## 24h_ffmpeg_toggle

| Rota | Metodo/escopo | Handler | Fonte |
|---|---|---|---|
| `/merger/api/24h_ffmpeg_toggle` | sessao | `mergerPainelAPI24hFFmpegToggle) // FASE 9 http.HandleFunc("/merger/api/24h_status", mergerPainelAPI24hStatus` | routes_catalogo.go:135 |

## canal_viewers

| Rota | Metodo/escopo | Handler | Fonte |
|---|---|---|---|
| `/merger/api/canal_viewers` | sessao | `mergerPainelAPICanaisViewers` | routes_catalogo.go:142 |

## proxy_sessions

| Rota | Metodo/escopo | Handler | Fonte |
|---|---|---|---|
| `/merger/api/proxy_sessions` | sessao | `mergerPainelAPIProxySessions` | routes_catalogo.go:143 |

## episode

| Rota | Metodo/escopo | Handler | Fonte |
|---|---|---|---|
| `/merger/api/episode/rename` | sessao | `mergerAPIEpisodeRename` | routes_catalogo.go:181 |

## disparos

| Rota | Metodo/escopo | Handler | Fonte |
|---|---|---|---|
| `/disparos` | sessao | `disparoPaginaHandler` | routes_disparo.go:27 |

## manutencao.mp4

| Rota | Metodo/escopo | Handler | Fonte |
|---|---|---|---|
| `/manutencao.mp4` | sessao | `identidadeServirSlate` | routes_identidade.go:44 |

## capa_manutencao

| Rota | Metodo/escopo | Handler | Fonte |
|---|---|---|---|
| `/capa_manutencao/` | sessao | `identidadeServirCapaManutencao` | routes_identidade.go:45 |

## manifest.webmanifest

| Rota | Metodo/escopo | Handler | Fonte |
|---|---|---|---|
| `/manifest.webmanifest` | sessao | `pwaManifestGlobal` | routes_loja_pwa.go:67 |

## sw.js

| Rota | Metodo/escopo | Handler | Fonte |
|---|---|---|---|
| `/sw.js` | sessao | `pwaServiceWorkerHandler` | routes_loja_pwa.go:68 |

## icons

| Rota | Metodo/escopo | Handler | Fonte |
|---|---|---|---|
| `/icons/` | sessao | `pwaIconHandler` | routes_loja_pwa.go:69 |

## apple-touch-icon.png

| Rota | Metodo/escopo | Handler | Fonte |
|---|---|---|---|
| `/apple-touch-icon.png` | sessao | `pwaIconHandler` | routes_loja_pwa.go:70 |

## apple-touch-icon-precomposed.png

| Rota | Metodo/escopo | Handler | Fonte |
|---|---|---|---|
| `/apple-touch-icon-precomposed.png` | sessao | `pwaIconHandler` | routes_loja_pwa.go:71 |

## logout

| Rota | Metodo/escopo | Handler | Fonte |
|---|---|---|---|
| `/merger/api/logout` | sessao | `mergerLogoutHandler` | routes_merger_core.go:41 |

## cliente_login

| Rota | Metodo/escopo | Handler | Fonte |
|---|---|---|---|
| `/merger/api/cliente_login` | sessao | `mergerClienteLoginHandler` | routes_merger_core.go:63 |

## whoami

| Rota | Metodo/escopo | Handler | Fonte |
|---|---|---|---|
| `/merger/api/whoami` | sessao | `mergerWhoamiHandler` | routes_merger_core.go:64 |

## mcp

| Rota | Metodo/escopo | Handler | Fonte |
|---|---|---|---|
| `/merger/api/mcp/painel` | sessao | `mcpPainelHandler` | routes_merger_core.go:98 |

## equipe

| Rota | Metodo/escopo | Handler | Fonte |
|---|---|---|---|
| `/merger/api/equipe/trocar_senha` | sessao | `mergerAPIEquipeTrocarSenha` | routes_merger_core.go:143 |

## agenda

| Rota | Metodo/escopo | Handler | Fonte |
|---|---|---|---|
| `/agenda/img` | sessao | `agendaImgProxyHandler` | routes_merger_core.go:188 |

## jogos

| Rota | Metodo/escopo | Handler | Fonte |
|---|---|---|---|
| `/jogos/capa/` | sessao | `servirCapaJogo` | routes_merger_core.go:197 |

## webhook

| Rota | Metodo/escopo | Handler | Fonte |
|---|---|---|---|
| `/webhook/meta/oficial` | sessao | `oficialWebhookHandler` | routes_oficial.go:53 |

## mensalidade

| Rota | Metodo/escopo | Handler | Fonte |
|---|---|---|---|
| `/merger/api/mensalidade/pagar-login` | sessao | `mensalidadePagarLogin` | routes_pagamentos.go:97 |

## troca_ok.mp4

| Rota | Metodo/escopo | Handler | Fonte |
|---|---|---|---|
| `/troca_ok.mp4` | sessao | `trocaServirClipeOK` | routes_servidores.go:43 |

## troca_indisponivel.mp4

| Rota | Metodo/escopo | Handler | Fonte |
|---|---|---|---|
| `/troca_indisponivel.mp4` | sessao | `trocaServirClipeIndisponivel` | routes_servidores.go:44 |

## troca_atualize_lista.mp4

| Rota | Metodo/escopo | Handler | Fonte |
|---|---|---|---|
| `/troca_atualize_lista.mp4` | sessao | `trocaServirClipeAtualizeLista` | routes_servidores.go:45 |

## tickets

| Rota | Metodo/escopo | Handler | Fonte |
|---|---|---|---|
| `/merger/api/tickets/anexo` | sessao | `featureGate("tickets", ticketsAPIAnexo)` | routes_tickets.go:42 |

## player_api.php

| Rota | Metodo/escopo | Handler | Fonte |
|---|---|---|---|
| `/player_api.php` | sessao | `xtreamAPIHandler` | routes_xtream_admin.go:22 |

## xmltv.php

| Rota | Metodo/escopo | Handler | Fonte |
|---|---|---|---|
| `/xmltv.php` | sessao | `xtreamAPIHandler` | routes_xtream_admin.go:23 |

## get.php

| Rota | Metodo/escopo | Handler | Fonte |
|---|---|---|---|
| `/get.php` | sessao | `gerarListaM3U` | routes_xtream_admin.go:24 |

## sigma_stats

| Rota | Metodo/escopo | Handler | Fonte |
|---|---|---|---|
| `/sigma_stats` | sessao | `sigmaStatsHandler` | routes_xtream_admin.go:25 |

## painel

| Rota | Metodo/escopo | Handler | Fonte |
|---|---|---|---|
| `/painel` | sessao | `painelAdminHandler` | routes_xtream_admin.go:27 |

## metrics

| Rota | Metodo/escopo | Handler | Fonte |
|---|---|---|---|
| `/metrics` | sessao | `metricsPrometheusHandler) // Prometheus text exposition (metrics_prometheus.go` | routes_xtream_admin.go:42 |
