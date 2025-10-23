use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::models::enums::{AuditAction, UserRole};

/// Registro de auditoría y trazabilidad de acciones sobre documentos o entidades
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AuditRecord {
    /// Identificador único del registro de auditoría
    pub id: Uuid,

    /// Identificador del documento o entidad afectada
    pub entidad_id: Option<Uuid>,

    /// Tipo de acción realizada (crear, modificar, eliminar, etc.)
    pub accion: AuditAction,

    /// Usuario que ejecutó la acción
    pub usuario: String,

    /// Rol del usuario en el momento de la acción
    pub rol: UserRole,

    /// Fecha y hora exacta de la acción (UTC)
    pub timestamp: DateTime<Utc>,

    /// Descripción o detalle adicional del evento
    pub descripcion: Option<String>,

    /// Hash del registro anterior (encadenamiento)
    pub hash_anterior: Option<String>,

    /// Hash actual calculado sobre el contenido de este registro
    pub hash_actual: String,
}
