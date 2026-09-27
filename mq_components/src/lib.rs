//! # Macroquad Components ('mq_components')
//! 
//! Una suite con componentes altamente personalizables para su uso en el ecosistema Macroquad
//! 
//! ## ¿Por qué esta crate?
//! Esta crate está siendo desarrollada debido a que los widgets nativos que ofrece Macroquad, aunque funcionales, 
//! no son fácilmente personalizables ni bonitos. Esta crate viene a tratar de solventar eso, haciendo que crear
//! widgets en el ecosistema de macroquad sea mas sencillo y personalizable.
//! 
//! ## Componentes incluidos
//! - [`slider`]: Sliders personalizable para modificar valores numericos de forma sencilla
//! - [`text_field`]: Campos de entrada de texo personalizables con enfoque automatico y placeholder(personalizado o por defecto)
//! 
//! ## Uso
//! Abre el **Cargo.toml** del proyecto en el que quieras incluir la crate y pega:
//! ```toml
//! mq_components = { path = "/path de la crate en tus sistema/mq_components" }
//! macroquad = "0.4"
//! ```
//! 

pub mod slider;
pub mod text_field;