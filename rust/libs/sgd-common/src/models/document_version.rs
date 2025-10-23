use schemars::JsonSchema;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Representa una versión específica de un documento en el sistema
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, JsonSchema)]
pub struct DocumentVersion {
    /// Identificador único de la versión
    pub id: Uuid,

    /// ID del documento al que pertenece esta versión
    pub documento_id: Uuid,

    /// Número o etiqueta de versión (v1, v2, etc.)
    pub numero: String,

    /// Usuario o sistema que realizó la modificación
    pub autor: String,

    /// Fecha de creación de la versión
    pub fecha: DateTime<Utc>,

    /// Comentario o descripción del cambio
    pub comentario: Option<String>,

    /// Hash de integridad de esta versión
    pub hash_contenido: String,

    /// Referencia opcional al hash de la versión anterior (para trazabilidad encadenada)
    pub hash_anterior: Option<String>,
}
