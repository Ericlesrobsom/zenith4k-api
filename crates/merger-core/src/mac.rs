//! Normalizacao de MAC por contrato de cada integracao.
//!
//! Existem **dois formatos** no mesmo painel:
//!
//! - **Provedor Ative App (apps_ativa):** 12 hexadecimais, minusculo,
//!   separados por `:` (`ab:51:6e:66:79:60`). E o formato que o Supabase
//!   aceita; mandar sem separador faz a ativacao ser recusada no provedor.
//! - **msproapp / Apps Ibo:** 12 caracteres alfanumericos (o painel aceita
//!   letras fora de A-F de proposito), maiusculo, separados por `:`.
//!
//! Por isso sao dois helpers separados — um unico normalizador quebraria um
//! dos dois lados.

use crate::error::{ApiError, Result};

/// MAC para o provedor Ative App (`apps_ativa`): hex, minusculo, `:`.
///
/// Aceita `ab516e667960`, `AB-51-6E-66-79-60`, `ab.51.6e.66.79.60` e
/// `ab 51 6e 66 79 60`. Qualquer caractere que nao seja separador conhecido
/// ou hexadecimal e recusado (`gg:...`, `abc`).
pub fn normalizar_mac_provedor(bruto: &str) -> Result<String> {
    let mut hex = String::with_capacity(12);
    for caractere in bruto.trim().chars() {
        match caractere {
            ':' | '-' | '.' | ' ' => continue,
            c if c.is_ascii_hexdigit() => hex.push(c.to_ascii_lowercase()),
            outro => {
                return Err(ApiError::InvalidMac(format!(
                    "MAC invalido {bruto:?}: caractere {outro:?} nao e hexadecimal"
                )))
            }
        }
    }
    let digitos: Vec<char> = hex.chars().collect();
    if digitos.len() != 12 {
        return Err(ApiError::InvalidMac(format!(
            "MAC invalido {bruto:?}: precisa de 12 caracteres hex"
        )));
    }
    let pares: Vec<String> = digitos
        .chunks(2)
        .map(|par| par.iter().collect::<String>())
        .collect();
    Ok(pares.join(":"))
}

/// MAC para o msproapp/Apps Ibo: 12 alfanumericos, maiusculo, `:`.
///
/// O painel do msproapp **nao exige hexadecimal** (ha aparelho com letra fora
/// de A-F). Ver `msproappNormalizarMAC` no Go.
pub fn normalizar_mac_ibo(bruto: &str) -> Result<String> {
    let limpo: String = bruto
        .trim()
        .chars()
        .filter(|caractere| !matches!(caractere, ':' | '-' | '.' | ' '))
        .collect();
    if limpo.chars().count() != 12 {
        return Err(ApiError::InvalidMac(format!(
            "MAC invalido {bruto:?}: precisa de 12 caracteres (letras ou numeros)"
        )));
    }
    if !limpo
        .chars()
        .all(|caractere| caractere.is_ascii_alphanumeric())
    {
        return Err(ApiError::InvalidMac(format!(
            "MAC invalido {bruto:?}: use apenas letras e numeros"
        )));
    }
    let maiusculo = limpo.to_ascii_uppercase();
    let pares: Vec<String> = maiusculo
        .chars()
        .collect::<Vec<char>>()
        .chunks(2)
        .map(|par| par.iter().collect())
        .collect();
    Ok(pares.join(":"))
}
