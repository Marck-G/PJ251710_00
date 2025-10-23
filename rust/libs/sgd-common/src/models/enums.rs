use serde::{Serialize, Deserialize};
use strum_macros::{EnumString, Display};

/// Nivel de acceso de un documento (ISO 23081 - AccessRights)
#[derive(Debug, Clone, Serialize, Deserialize, EnumString, Display, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AccessLevel {
    Publico,
    Interno,
    Confidencial,
}

/// Estado del documento dentro de su ciclo de vida (ISO 23081 - Status)
#[derive(Debug, Clone, Serialize, Deserialize, EnumString, Display, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DocumentStatus {
    Borrador,
    Vigente,
    Archivado,
    Eliminado,
}

/// Tipo de documento o género (ISO 23081 - Type / genre)
#[derive(Debug, Clone, Serialize, Deserialize, EnumString, Display, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DocumentType {
    Informe,
    Resolucion,
    Contrato,
    Correspondencia,
    Proyecto,
    Otro,
}

/// Rol del usuario dentro del sistema (según requisitos de gestión documental)
#[derive(Debug, Clone, Serialize, Deserialize, EnumString, Display, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum UserRole {
    Administrador,
    Productor,
    UsuarioFinal,
    Auditor,
}

/// Acción registrada en auditoría (ISO 16175 - Logging & Traceability)
#[derive(Debug, Clone, Serialize, Deserialize, EnumString, Display, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AuditAction {
    Crear,
    Modificar,
    Eliminar,
    Consultar,
    Descargar,
    RestaurarVersion,
    CambiarPermisos,
    Aprobar,
    Rechazar,
}

/// Tipos posibles de relación entre documentos
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RelationType {
    /// Documento es una versión anterior o posterior
    Version,
    /// Documento es un anexo o adjunto de otro
    Anexo,
    /// Documento está vinculado por un proceso o proyecto común
    Asociado,
    /// Documento sustituye o reemplaza a otro
    Sustituye,
    /// Otro tipo de relación definida por la organización
    Otro,
}

/// Formatos admitidos para documentos
#[derive(Debug, Clone, Serialize, Deserialize, EnumString, Display, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FileFormat {
    Pdf,
    Docx,
    Xlsx,
    Txt,
    Csv,
    Xml,
    Json,
    Png,
    Jpg,
    Otro,
}