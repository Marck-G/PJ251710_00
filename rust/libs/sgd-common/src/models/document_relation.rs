use schemars::JsonSchema;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::models::enums::RelationType;



/// Relación entre dos documentos dentro del sistema
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, JsonSchema)]
pub struct DocumentRelation {
    /// Identificador único de la relación
    pub id: Uuid,

    /// ID del documento principal
    pub documento_id: Uuid,

    /// ID del documento relacionado
    pub documento_relacionado_id: Uuid,

    /// Tipo de relación (anexo, versión, etc.)
    pub tipo: RelationType,

    /// Descripción opcional o nota sobre la relación
    pub descripcion: Option<String>,
}

impl DocumentRelation {
    /// Crea una nueva instancia de DocumentRelation
    pub fn new(
        id: Uuid,
        documento_id: Uuid,
        documento_relacionado_id: Uuid,
        tipo: RelationType,
        descripcion: Option<String>,
    ) -> Self {
        DocumentRelation {
            id,
            documento_id,
            documento_relacionado_id,
            tipo,
            descripcion,
        }
    }
}