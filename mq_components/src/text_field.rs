//! TextField ('mq_components::text_field')
//! 
//! **Version:** 0.1
//! **Autor:** Héctor Monroy Fuertes
//! **Fecha inicio:** 20-09-2026
//! 
//! 
//! # Macroquad Input Text
//! 
//! Una crate sencilla para crear campos de entrada de texto en Macroquad
//! 
//! ## ¿Por que esta crate?
//! Esta crate ha sido desarrollada con la intención de poder crear campos de entrada de texto
//! en el ecosistema de Macroquad, ya que el nativo de Macroquad no es muy personalizable.
//! 
//! ## Características principales
//! - **Gestión centralizada:** A través de [`ListaTextFields`], se controla de forma segura qué campo de texto está escuchando el teclado,
//!  evitando interferencias si hay más de uno.
//! - **Placeholder:** Soporte para placeholders (por defecto "Escribe aquí..." si no se especifica) 
//! o personalizados mediante `Option<&str>`. Se limpian automáticamente al interactuar por primera vez con el campo de texto.
//! - **Límite de caracteres dinámico:** Calcula cuántos caracteres caben a lo ancho del campo según el ancho del mismo y el tamaño de la fuente,
//! previniendo de forma nativa que el texto se desborde del campo de texto.
//! - **Feedback visual:** El componente cambia su color de fondo a gris cuando está inactivo, blanco (o color personalizado) cuando está activo,
//! y **rojo** si el usuario alcanza el límite máximo de caracteres permitidos.
//! - **Cursor parpadeante:** Inyecta un cursor visual (`|`) en tiempo de renderizado de forma fluida sin alterar los datos de la cadena de texto real.
//! 
//! ## Ejemplo básico de uso
//! ```rust
//! use mq_components::text_field::TextField;
//! use macroquad::prelude::*;
//! 
//! #[macroquad::main("Ejemplo Text Field")]
//! async fn main() {
//!     let mut lista: ListaTextFields = ListaTextFields::nuevo();
//!     
//!     lista.agregar(TextField::text_field(50.0, 50.0, 200.0, 20.0, WHITE, BLACK, "Prueba:", None));
//!     
//!     loop {
//!         clear_background(BLUE);
//!         lista.actualizar_y_pintar();
//!         next_frame().await();
//!     }
//! }
//! ```
//!
use macroquad::prelude::*;

/// La struct TextField permite orquestar todos los parametros de los diferentes campos de texto,
/// la implementación es la misma para el usuario poniendo a true la variable -border- cuando se desea
/// que el campo de texto tenga un borde con el color personalizado
pub struct TextField {
    pub texto: String,
    pos_y: f32,
    pos_x: f32,
    ancho: f32,
    alto: f32,
    color_fondo: Color,
    color_texto: Color,
    color_borde: Option<Color>,
    etiqueta_campo: String,
    border: bool,
    activo: bool,
    primera_activacion: bool,
}

/// La implementacion de la struct del campo de texto permite crear dos componentes distintos, uno con un pequeñp borde negro por defecto
/// para dar sensación de volumen y otro con el color del borde personalizable por el usuario
impl TextField {
    /// Esta función permite crear un campo de texto con un pequeño borde negro para dar sensación de volumen
    pub fn text_field(x: f32, y: f32, ancho: f32, alto: f32, color_fondo: Color, color_texto: Color, etiqueta: &str, place_holder: Option<&str>) -> Self {
        let place_holder = place_holder.unwrap_or("Escribe aquí...");

        Self {
            pos_x: x,
            pos_y: y,
            ancho: ancho,
            alto: alto,
            color_fondo: color_fondo,
            color_texto: color_texto,
            color_borde: None,
            etiqueta_campo: etiqueta.to_string(),
            border: false,
            activo: false,
            texto: place_holder.to_string(),
            primera_activacion: true,
        }
    }

    /// Esta función permite crear un campo de texto con el borde de un color concreto
    pub fn border_text_field(x: f32, y: f32, ancho: f32, alto: f32, color_borde: Color, color_fondo: Color, color_texto: Color, etiqueta: &str, place_holder: Option<&str>) -> Self {
        let place_holder = place_holder.unwrap_or("Escribe aquí...");

        Self {
            pos_x: x,
            pos_y: y,
            ancho: ancho,
            alto: alto,
            color_borde: Some(color_borde),
            color_fondo: color_fondo,
            color_texto: color_texto,
            etiqueta_campo: etiqueta.to_string(),
            border: true,
            activo: false,
            texto: place_holder.to_string(),
            primera_activacion: true,
        }
    }

    /// Determinar si el campo de texto está ativo o inactivo, 
    /// además si es la primera activacion del mismo eliminamos el placeholder
    fn is_active(&mut self) {
        let (x_mouse, y_mouse) = mouse_position();

        // Calculamos si se ha hecho click dentro/fuera del campo de texto para marcarlo como activo/inactivo
        if x_mouse >= self.pos_x && x_mouse <= (self.pos_x + self.ancho) {
            if y_mouse >= self.pos_y && y_mouse <= (self.pos_y + self.alto){
                if is_mouse_button_down(MouseButton::Left) {
                    self.activo = true;
                    // Si es la primera vez que clicamos sobre el componente eliminamos su contenido(El placeholder)
                    if self.primera_activacion {
                        self.texto = "".to_string();
                        self.primera_activacion = false; 
                    }
                    return
                }                  
            } else {
                if is_mouse_button_down(MouseButton::Left) {
                    self.activo = false;
                }
            }
        } else {
            if is_mouse_button_down(MouseButton::Left) {
                self.activo = false;
            }
        }
    }

    /// Metodo para renderizar en pantalla el campo de texto con el string en su interior
    fn write(&mut self) {
        self.is_active();

        let tamaño_caracter: f32 = (self.alto - 6.0) * 0.6;
        let max_caracteres: usize = (self.ancho / tamaño_caracter).floor() as usize;
        let longitud_cadena: usize = self.texto.chars().count();
        let mut cursor: String = self.texto.clone();

        //Cursor parpadeante
        if self.activo && get_time() % 1.0 > 0.5 {
                cursor.push('|');
        }

        //Pintamos los diferentes bordes
        if !self.border {
            //Pequeño borde negro para sombrear el campo de texto normal
            draw_rectangle(
                self.pos_x - 1.0,
                self.pos_y - 1.0,
                self.ancho + 2.0,
                self.alto + 2.0,
            BLACK,
            );
        } else {
            //Borde mas grueso del color elegido
            draw_rectangle(
                self.pos_x - 2.0,
                self.pos_y - 2.0,
                self.ancho + 4.0,
                self.alto + 4.0,
            self.color_borde.unwrap(),
            );
        }


        //Se pinta el color de fondo en funcion de si el textfield esta activo o no y de si se han superado los caracteres máximos
        if !self.activo {

            //Fondo gris cuando esta inactivo
            let color_fondo_inactivo = Color::from_rgba(190, 190, 190, 125);
            draw_rectangle(
                self.pos_x,
                self.pos_y,
                self.ancho,
                self.alto,
                color_fondo_inactivo,
            );  
        } else if longitud_cadena < max_caracteres {

                // Fondo de color elegido por el usuario si está activo
                draw_rectangle(
                    self.pos_x,
                    self.pos_y,
                    self.ancho,
                    self.alto,
                    self.color_fondo,
                );  
            } else {

                // Fondo rojo si ha superado el máximo de caracteres que admite el componente
                draw_rectangle(
                    self.pos_x,
                    self.pos_y,
                    self.ancho,
                    self.alto,
                    Color::from_rgba(255, 20, 20, 255),
                );
            }
        // Mostramos el texto en el campo de texto
        draw_text(
                &cursor,
                self.pos_x + 4.0,
                self.pos_y + (self.alto / 2.0) + 6.0,
                self.alto - 2.0,
                self.color_texto
            );
            
        //Etiqueta del campo
        draw_text(
            &self.etiqueta_campo,
            self.pos_x,
            self.pos_y - 10.0,
            22.0,
            self.color_texto
        );
        


    }
}

/// Creamos una lista para almacenar todos los componentes TextField que creemos, 
/// esto permitirá un mejor trabajo individual sobre cada componente
pub struct ListaTextFields {
    componentes: Vec<TextField>,
}

impl ListaTextFields {
    // Método para crear la lista en si
    pub fn nuevo() -> Self {
        ListaTextFields { componentes: Vec::new() }
    }
    // Método para agregar componentes a la lista
    pub fn agregar(&mut self, componente: TextField) {
        self.componentes.push(componente);
    }

    // Este metodo permite modificar el atributo 'texto'(componente_foco.texto) del componente que está activo,
    // llamando al final del mismo al método write del propio componente para actualizarse en pantalla
    pub fn actualizar_y_pintar(&mut self) {
        let componente_activo = self.componentes.iter().any(|c| c.activo);

        if componente_activo {
            if is_key_pressed(macroquad::input::KeyCode::Backspace) {
                if let Some(componente_enfocado) = self.componentes.iter_mut().find(|c| c.activo) {
                    componente_enfocado.texto.pop();
                }
            }
            while let Some(caracter) = get_char_pressed() {
                if !caracter.is_control() {
                    if let Some(componente_foco) = self.componentes.iter_mut().find(|c| c.activo) {
                        let tamaño_caracter = (componente_foco.alto - 6.0) * 0.6;
                        let max_caracteres = (componente_foco.ancho / tamaño_caracter).floor() as usize;
                        
                        if componente_foco.texto.chars().count() <= max_caracteres {
                            componente_foco.texto.push(caracter);
                        }
                    }
                }
                
            } 
        } else {
                while get_char_pressed().is_some() {}
        }

        for componente in self.componentes.iter_mut() {
            componente.write();
        }
    }

    // Metodo para poder obtener el texto de un componente de la lista
    pub fn get_texto(&self, indice: usize) -> Option<&str> {
        self.componentes.get(indice).map(|c| c.texto.as_str())
    }
}