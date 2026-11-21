# Smoke test SOMENTE LEITURA contra o painel real.
#
# Pre-requisito: .env preenchido (ZENITH_URL, ZENITH_USER, ZENITH_PASS).
# Nao cria, edita nem apaga nada. Nao imprime a senha.
#
# Uso:
#   powershell -ExecutionPolicy Bypass -File .\scripts\smoke-live.ps1

$ErrorActionPreference = "Stop"
$raiz = Split-Path $PSScriptRoot -Parent
Set-Location $raiz

if (-not (Test-Path ".env")) {
    throw "Crie .env a partir de .env.example e preencha ZENITH_PASS."
}

$cfg = @{}
Get-Content ".env" | ForEach-Object {
    if ($_ -match '^\s*([A-Za-z_][A-Za-z0-9_]*)\s*=\s*(.*)$') {
        $cfg[$matches[1]] = $matches[2].Trim()
    }
}

if ([string]::IsNullOrWhiteSpace($cfg["ZENITH_URL"])) { throw "ZENITH_URL vazio no .env" }
if ([string]::IsNullOrWhiteSpace($cfg["ZENITH_USER"])) { throw "ZENITH_USER vazio no .env" }
if ([string]::IsNullOrWhiteSpace($cfg["ZENITH_PASS"])) { throw "ZENITH_PASS vazio no .env" }

$env:ZENITH_URL = $cfg["ZENITH_URL"]
$env:ZENITH_USER = $cfg["ZENITH_USER"]
$env:ZENITH_PASS = $cfg["ZENITH_PASS"]
if ($cfg["ZENITH_FRANQUIA"]) { $env:ZENITH_FRANQUIA = $cfg["ZENITH_FRANQUIA"] }
if ($cfg["ZENITH_ROLE"]) { $env:ZENITH_ROLE = $cfg["ZENITH_ROLE"] }

function Passo($titulo, $argumentos) {
    Write-Host ""
    Write-Host "=== $titulo ===" -ForegroundColor Cyan
    & cargo run -q -p merger-cli -- @argumentos
    if ($LASTEXITCODE -ne 0) { throw "falhou: $titulo" }
}

Passo "whoami (login + identidade)" @("whoami")
Passo "revendas" @("revendas")
Passo "usuarios (1 registro)" @("usuarios", "-q", "limit=1")
Passo "planos" @("get", "/merger/api/cliente/planos")
Passo "creditos" @("get", "/merger/api/cliente/creditos")
Passo "pedidos" @("pedidos")
Passo "reportes" @("reportes")
Passo "avisos" @("avisos")
Passo "stats" @("stats")
Passo "dashboard" @("dashboard")

Write-Host ""
Write-Host "Smoke test concluido (somente leitura)." -ForegroundColor Green
