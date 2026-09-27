# Macroquad Components :bar_chart:  
![Visitas](https://komarev.com/ghpvc/?username=HectorCRM-simple-macroquad-slider&color=blue&style=round&label=Visitas:)  

![Ejemplo](./img/slider.gif)  

## ¿Cual es la utilidad de este proyecto? ⁉️
Este es mi primer pequeño proyecto desarrollado en Rust, como parte de un proyecto mayor.  
El caso es que necesitaba utilizar sliders para controlar el volumen y... el que ofrece macroquad me parecio feo. Asi que si algo no te gusta... ¡crea tu propia versión!  
Aun hay mucho que pulir, poco a poco ire mejorandole conforme vaya encontrando sus limites en diferentes proyectos o si recibo algun tipo de feedback.  
El proyecto crece según voy necesitando componentes para mi proyecto principal, asi que pasa a llamarse Macroquad Components, ya que va a englobar las diferentes utilidades que vaya desarrollando para mis proyectos dentro del ecosistema de Macroquad.

## Uso  :gear:
Clona este repositorio:  
```
git clone https://github.com/HectorCRM/simple_macroquad_slider.git
```

Abre el Cargo.toml del proyecto en el que quieras utilizarlo y añade:
```
[dependencies]
Slider = { path = "/ruta del crate en tu máquina" }
```

Luego incluyelo en el proyecto:
```
use slider::Slider;
```

Hecho esto, debes contar con una variable de tipo mut sobre la cual trabajara el slider. En el gif de ejemplo trabaja con volumen:
```
let mut volumen: f32 = 0.5; //A mitad, por ejemplo
```

Despues construimos el slider:
```
let mut slider_volumen: Slider = Slider::nuevo_slider(nombre_etiqueta, metrica, posicion_y, ancho_barra, alto, valor, valor_minimo, valor_maximo, color_barra, color_slider);
```
Explicación de los parametros:  
 - **nombre_etiqueta:** El nombre que se mostrará sobre el slider en pantalla, en el ejemplo "Volumen"
 - **metrica:** Hace referencia al tipo de dato con el que estamos trabajando, **%** en el ejemplo. Puede dejarse vacia("") si no queremos mostrar nada.
 - **posicion_y:** Posición del slider en el eje vertical de la pantalla.
 - **ancho_barra:** Anchura que ocupará el slider en pantalla.
 - **alto:** Altura en px que tendrán en pantalla la barra del slider y el propio slider.
 - **valor:** El valor o variable que queremos modificar con el slider.
 - **valor_minimo:** Valor minimo deseado para la variable con la que vamos a trabajar, es importante establecerlo para poder mapear el valor a pixeles al renderizar el slider.
 - **valor_maximo:** Idem, pero para el limirte superior del valor de la variable.
 - **color_barra:** Color deseado para la barra del slider.
 - **color_slider:** Color deseado para el slider y la etiqueta sobre este.  
 
Y ya podemos usarlo:
```
slider_volumen.pintar_slider(volumen);
volumen = slider_volumen.mover_slider(volumen);
```

## Requisitos :clipboard:
 - Git
 - Rustc

## Mejoras futuras :rocket:
 - Crear sliders verticales.  
 - Habilitar valores personalizados y diferentes métricas a los sliders. ✔️  
 - Desarrollar campos de entrada de texto. ✔️  
 
<!--
## Versiones :pushpin:
 [Ver CHANGELOG](./CHANGELOG.md) -->

