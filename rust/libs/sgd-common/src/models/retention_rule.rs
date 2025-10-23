use schemars::JsonSchema;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{NaiveDate};

/// Regla de retención documental (TRD)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, JsonSchema)]
pub struct RetentionRule {
    /// Identificador único de la regla TRD
    pub id: Uuid,

    /// Serie o subserie documental a la que aplica
    pub serie_documental: String,

    /// Plazo de retención en archivo de gestión (en años)
    pub plazo_gestion: u16,

    /// Plazo de retención en archivo central o histórico (en años)
    pub plazo_central: u16,

    /// Disposición final (eliminación, conservación, transferencia)
    pub disposicion_final: String,

    /// Fecha de creación o aprobación de la regla
    pub fecha_creacion: NaiveDate,

    /// Observaciones o referencias normativas
    pub observaciones: Option<String>,
}
