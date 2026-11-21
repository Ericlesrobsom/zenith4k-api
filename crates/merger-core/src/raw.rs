use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Pedido cru para qualquer rota autenticada do painel.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawRequest {
    pub method: String,
    pub path: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub query: Vec<(String, String)>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<Value>,
    /// Precisa ser `true` para metodos/rotas destrutivas.
    #[serde(default)]
    pub confirm: bool,
}

impl RawRequest {
    pub fn new(method: impl Into<String>, path: impl Into<String>) -> Self {
        Self {
            method: method.into().to_ascii_uppercase(),
            path: path.into(),
            query: Vec::new(),
            body: None,
            confirm: false,
        }
    }

    pub fn get(path: impl Into<String>) -> Self {
        Self::new("GET", path)
    }

    pub fn post(path: impl Into<String>, body: impl Into<Value>) -> Self {
        let mut r = Self::new("POST", path);
        r.body = Some(body.into());
        r
    }

    pub fn put(path: impl Into<String>, body: impl Into<Value>) -> Self {
        let mut r = Self::new("PUT", path);
        r.body = Some(body.into());
        r
    }

    pub fn delete(path: impl Into<String>) -> Self {
        Self::new("DELETE", path)
    }

    pub fn query<K: Into<String>, V: Into<String>>(mut self, key: K, value: V) -> Self {
        self.query.push((key.into(), value.into()));
        self
    }

    pub fn confirm(mut self, yes: bool) -> Self {
        self.confirm = yes;
        self
    }
}

/// Resposta normalizada do painel.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawResponse {
    pub status: u16,
    pub ok: bool,
    /// Corpo JSON quando a resposta e JSON; `Value::Null` caso contrario.
    pub body: Value,
    /// Corpo textual sempre presente (vazio quando nao houve corpo).
    pub text: String,
    pub content_type: Option<String>,
    pub elapsed_ms: u64,
}

impl RawResponse {
    /// `body` quando JSON, senao o texto cru.
    pub fn json_or_text(&self) -> Value {
        if self.body.is_null() {
            Value::String(self.text.clone())
        } else {
            self.body.clone()
        }
    }
}

/// Arquivo enviado em um request `multipart/form-data`.
#[derive(Debug, Clone)]
pub struct UploadFile {
    /// Nome do campo no formulario (ex.: `imagem`, `logo`, `arquivo`).
    pub field: String,
    pub filename: String,
    /// MIME type; default `application/octet-stream`.
    pub mime: String,
    pub bytes: Vec<u8>,
}

impl UploadFile {
    pub fn new(
        field: impl Into<String>,
        filename: impl Into<String>,
        bytes: impl Into<Vec<u8>>,
    ) -> Self {
        Self {
            field: field.into(),
            filename: filename.into(),
            mime: "application/octet-stream".to_string(),
            bytes: bytes.into(),
        }
    }

    pub fn mime(mut self, mime: impl Into<String>) -> Self {
        self.mime = mime.into();
        self
    }
}

/// Pedido de upload multipart para o painel.
#[derive(Debug, Clone, Default)]
pub struct RawUpload {
    pub path: String,
    /// Campos texto do formulario.
    pub fields: Vec<(String, String)>,
    pub files: Vec<UploadFile>,
    /// Precisa ser `true` quando a rota e destrutiva.
    pub confirm: bool,
}

impl RawUpload {
    pub fn new(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            ..Self::default()
        }
    }

    pub fn field(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.fields.push((key.into(), value.into()));
        self
    }

    pub fn file(mut self, file: UploadFile) -> Self {
        self.files.push(file);
        self
    }

    pub fn confirm(mut self, yes: bool) -> Self {
        self.confirm = yes;
        self
    }
}
