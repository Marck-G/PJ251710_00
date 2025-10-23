use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::models::enums::RelationType;



/// Relación entre dos documentos dentro del sistema
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
