//! Utilities para os exemplos.
//!
//! Este módulo agrupa utilitários compartilhados entre os exemplos,
//! atualmente provendo um `logger` configurável por *feature flags*.
//!
//! Reexportamos funções de log de `logger` para facilitar imports
//! (ex.: `use crate::utils::info;`).

//! Nota: as funções reexportadas dependem da configuração de *features*
//! do crate (`defmt`, `log-itm`, `log-semihost`).
pub mod logger;

// Reexporta helpers de logging para uso direto: `crate::utils::info()`.
// Isso simplifica imports em binários/exemplos.
#[allow(unused_imports)]
pub use logger::{info, trace, warn, debug, error};