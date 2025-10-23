use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::models::enums::UserRole;

/// Información básica del usuario, sincronizada con el microservicio de usuarios
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserInfo {
    /// Identificador único del usuario
    pub id: Uuid,

    /// Nombre completo o alias
    pub nombre: String,

    /// Correo electrónico institucional
    pub correo: String,

    /// Rol del usuario dentro del sistema
    pub rol: UserRole,

    /// Estado de la cuenta (activo, suspendido, etc.)
    pub activo: bool,

    /// Fecha de último acceso al sistema
    pub ultimo_acceso: Option<DateTime<Utc>>,
}
