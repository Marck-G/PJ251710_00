use serde::{Serialize, Deserialize};
use schemars::JsonSchema;
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::models::{
    DocumentMetadata,
    DocumentVersion,
    Classification,
    RetentionRule,
    AccessLevel,
    DocumentStatus,
    UserInfo,
};

/// Representa un documento gestionado dentro del sistema SGD
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct Document {
    /// Identificador único del documento
    pub id: Uuid,

    /// Metadatos principales (ISO 23081)
    pub metadata: DocumentMetadata,

    /// Clasificación documental (serie, subserie, tipo)
    pub clasificacion: Option<Classification>,

    /// Versión actual del documento
    pub version_actual: Option<DocumentVersion>,

    /// Regla de retención documental (TRD)
    pub regla_retencion: Option<RetentionRule>,

    /// Estado general del documento (borrador, vigente, archivado)
    pub estado: DocumentStatus,

    /// Nivel de acceso (público, interno, confidencial)
    pub nivel_acceso: AccessLevel,

    /// Usuario propietario o productor principal
    pub propietario: Option<UserInfo>,

    /// Fecha de creación
    pub fecha_creacion: DateTime<Utc>,

    /// Fecha de última modificación
    pub fecha_modificacion: Option<DateTime<Utc>>,

    /// Hash global de integridad del documento (última versión)
    pub hash_integridad: Option<String>,

    /// Indica si el documento está activo o ha sido eliminado lógicamente
    pub activo: bool,
}

impl Document {
    /// Crea una nueva instancia de Document
    pub fn new(
        id: Uuid,
        metadata: DocumentMetadata,
        clasificacion: Option<Classification>,
        version_actual: Option<DocumentVersion>,
        regla_retencion: Option<RetentionRule>,
        estado: DocumentStatus,
        nivel_acceso: AccessLevel,
        propietario: Option<UserInfo>,
        fecha_creacion: DateTime<Utc>,
        fecha_modificacion: Option<DateTime<Utc>>,
        hash_integridad: Option<String>,
        activo: bool,
    ) -> Self {
        Document {
            id,
            metadata,
            clasificacion,
            version_actual,
            regla_retencion,
            estado,
            nivel_acceso,
            propietario,
            fecha_creacion,
            fecha_modificacion,
            hash_integridad,
            activo,
        }
    }
}