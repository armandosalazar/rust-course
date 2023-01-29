/* Variables & Constantes */
fn variables_and_const() {
    /* Variables */
    let x = 0;
    let y: u8 = 1;
    // Las variables en rust son inmutables por defecto.
    let mut z = 0;
    z = 1;
    println!("{} {} {}", x, y, z);

    /* Scope and Shadowing */
    // El shadowing puede ser util para cambiar el tipo de dato
    // o para cambiar el valor a una variable inmutable.
    let x = "Text";

    /* Constante */
    // Las constantes se tienen que asignar al declararse
    // también se les asigna su respectivo tipo de dato
    // a diferencia de las variables.
    // Las constantes pueden estar en cualquier scope,
    // como variables globales.
    const IQ: u8 = 180;
}