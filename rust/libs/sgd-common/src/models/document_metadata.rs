use schemars::JsonSchema;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::models::enums::{AccessLevel, DocumentStatus, DocumentType};

/// Metadatos de un documento según ISO 23081
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, JsonSchema)]
pub struct DocumentMetadata {
    /// Identificador único e inalterable del documento
    pub id: Uuid,

    /// Título o asunto del documento
    pub titulo: String,

    /// Productor o unidad responsable
    pub productor: String,

    /// Fecha de creación o registro
    pub fecha_creacion: DateTime<Utc>,

    /// Tipo documental o género
    pub tipo: DocumentType,

    /// Descripción opcional del contenido
    pub descripcion: Option<String>,

    /// Palabras clave o etiquetas
    pub palabras_clave: Option<Vec<String>>,

    /// Estado del documento en su ciclo de vida
    pub estado: DocumentStatus,

    /// Nivel de acceso (público, interno, confidencial)
    pub nivel_acceso: AccessLevel,

    /// Formato del archivo (PDF, DOCX, etc.)
    pub formato: String,

    /// Relación con otros documentos (anexos, versiones, etc.)
    pub relacionados: Option<Vec<Uuid>>,
}
