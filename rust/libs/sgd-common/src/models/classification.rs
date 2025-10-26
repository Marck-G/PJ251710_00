use serde::{Serialize, Deserialize};
use schemars::JsonSchema;

/// Clasificación documental según TRD (serie, subserie, tipo)
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct Classification {
    /// Serie documental
    pub serie: String,

    /// Subserie documental (opcional)
    pub subserie: Option<String>,

    /// Tipo documental (opcional)
    pub tipo_documental: Option<String>,

    /// Código TRD o identificador normativo
    pub codigo_trd: Option<String>,
}

impl Classification {
    /// Crea una nueva instancia de Classification
    pub fn new(
        serie: String,
        subserie: Option<String>,
        tipo_documental: Option<String>,
        codigo_trd: Option<String>,
    ) -> Self {
        Classification {
            serie,
            subserie,
            tipo_documental,
            codigo_trd,
        }
    }
}