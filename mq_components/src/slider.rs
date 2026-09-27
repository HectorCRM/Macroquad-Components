//! # Macroquad Utils
//! 
//! //! Version: 0.1.1
//! Autor: Héctor Monroy Fuertes
//! Fecha inicio: 20-09-2026
//! 
//! Una crate sencilla de utilizar para generar sliders y campos de texto en Macroquad
//! 
//! ## ¿Por qué esta crate?
//! Esta crate está siendo desarrollada debido a que los widgets nativos que ofrece Macroquad, aunque funcionales, 
//! no son fácilmente personalizables ni bonitos. Esta crate viene a tratar de solventar eso, haciendo que crear
//! widgets en el ecosistema de macroquad sea mas sencillo y personalizable.
//! 
//! ## Caracteristicas de Slider
//! - **Rango dinamico:** Admite valores lógicos(en f32) personalizados (Ej: 0.0 a 1.0, 0.0 a 255.0).
//! - **Métricas personalizadas:** Permite añadir un sufijo al valor mostrado por el slider (Ej: "%", "px", ""... ).
//! 
//! ### Ejemplo básico de uso de Slider
//! ```rust
//! use mq_components::slider::Slider;
//! use macroquad::prelude::*;
//! 
//! #[macroquad::main("Ejemplo")]
//! async fn main() {
//!     // 1. Inicializamos una variable mutable y el componente con el rango deseado
//!     let mut volumen = 50.0;
//!     let mut slider_volumen = Slider::nuevo_slider("Volumen", "%", 100.0, 400.0, 20.0, volumen, 0.0, 100.0, WHITE, RED);
//!
//!     loop {
//!         clear_background(GRAY);
//!
//!         // 2. Renderizamos y actualizamos el estado con el ratón
//!         slider_volumen.pintar_slider(volumen);
//!         volumen = slider_volumen.mover_slider(volumen);
//!
//!         next_frame().await;
//!     }
//! }
//! ```


use macroquad::prelude::*;

///
/// Slider
/// 
/// Struct con todos los datos del slider
pub struct Slider {
    pos_x_barra: f32,
    pos_y_barra: f32,
    ancho_barra: f32,
    alto: f32,
    ancho_slider: f32,
    pos_x_slider: f32,
    color_barra: Color,
    color_slider: Color,
    arrastrando: bool,
    nombre : String,
    metrica: String,
    min_valor: f32,
    max_valor: f32,
}

impl Slider {

    pub fn nuevo_slider(nombre: &str, metrica: &str, pos_y: f32, ancho: f32, alto: f32, valor: f32, min_valor: f32, max_valor: f32, color_barra: Color, color_slider: Color) -> Self {
        let x: f32 = (screen_width() / 2.0) - (ancho / 2.0);
        
        Self{
            pos_x_barra: x,
            pos_y_barra: pos_y,
            ancho_barra: ancho,
            alto: alto,
            ancho_slider: 15.0,
            pos_x_slider: valor,
            color_barra: color_barra,
            color_slider: color_slider,
            arrastrando: false,
            nombre: nombre.to_string(),
            metrica: metrica.to_string(),
            min_valor: min_valor,
            max_valor: max_valor,
        }
    }

    /// Método para poder obtener las coordenadas del mouse de forma sencilla
    pub fn posicion_mouse() -> (f32, f32) {
        let (raton_x, raton_y) = mouse_position();
        
        (raton_x, raton_y)
    }

    /// Método para mapear el valor recibido a los px de la barra del slider
    fn map_valor(valor: f32, in_min: f32, in_max: f32, out_min: f32, out_max: f32) -> f32 {
        (valor - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
    }

    /// Posicionamos el slider en la barra mapeando su valor entre los valores min y max y el inicio en x de la barra y el final de la misma
    fn posicionar_slider(&self, valor: f32) -> f32 {
        let ancho_util_barra: f32 = self.pos_x_barra + self.ancho_barra - self.ancho_slider;

        Slider::map_valor(valor, self.min_valor, self.max_valor, self.pos_x_barra, ancho_util_barra)
    }

    /// Método para modificar el valor de la variable con la que estamos trabajando
    pub fn mover_slider(&mut self, valor: f32) -> f32 {
    let (x_mouse, y_mouse) = Slider::posicion_mouse();
    
    // Obtener la posición del slider sincronizada con el main
    self.pos_x_slider = self.posicionar_slider(valor);

    // Si no se presiona el clic, soltamos el slider
    if !is_mouse_button_down(MouseButton::Left) {
        self.arrastrando = false;
        return valor;
    } 

    if x_mouse >= self.pos_x_slider && x_mouse <= (self.pos_x_slider + self.ancho_slider) {
        if y_mouse >= self.pos_y_barra && y_mouse <= self.pos_y_barra + self.alto {
            self.arrastrando = true;
        }
    }
    

    // Si ya estamos arrastrando el cursor, modificamos el valor 
    if self.arrastrando {
        let ancho_util_barra = self.pos_x_barra + self.ancho_barra - self.ancho_slider;
        let click_relativo_x = x_mouse - (self.ancho_slider / 2.0);
        let nuevo_valor: f32 = Slider::map_valor(click_relativo_x, self.pos_x_barra, ancho_util_barra, self.min_valor, self.max_valor);
        //Limitamos el valor a los valores definidos por el usuario al crear el slider
        let nuevo_clamped = nuevo_valor.clamp(self.min_valor, self.max_valor);
        
        // Actualizar la nueva posición del slider
        self.pos_x_slider = self.posicionar_slider(nuevo_clamped);
        return nuevo_clamped; 
    }

    return valor;
    }


    /// Método para pintarlo todo en pantalla: etiqueta, valor, metrica, barra y slider
    pub fn pintar_slider(&self, valor: f32) {
        let x_slider = self.posicionar_slider(valor);

        let texto_eqtiqueta = format!("{}: {:.0}{}", self.nombre, valor, self.metrica);
        let pos_y_eqtiqueta = self.pos_y_barra - 10.0;
        let pos_x_etiqueta = self.ancho_barra / 2.0;
        //Etiqueta
        draw_text(&texto_eqtiqueta, pos_x_etiqueta, pos_y_eqtiqueta, 24.0, self.color_slider);
        //Barra
        draw_rectangle(self.pos_x_barra, self.pos_y_barra, self.ancho_barra, self.alto, self.color_barra);
        //Slider
        draw_rectangle(x_slider, self.pos_y_barra, self.ancho_slider, self.alto, self.color_slider);
    }

}