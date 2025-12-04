fn main() {
    // Variables
    // Se define una variable inmutable para el ID del objetivo
    let id_objetivo: i32 = 101;

    // Definimos una variable MUTABLE para el estado de la conexion 
    // Inicialmente sera un "false"
    let mut conectado: bool = false;

    println!("Objetivo : Sistema #{}", id_objetivo);
    println!("Estado inicial de conexion: {}", conectado);

    // 2 Cambiando valores
    // Como 'conectado' es 'mut' podemos cambiarla
    println!("Iniciando conexion -----------");
    conectado = true;
    println!("Estado actual de conexion: {}", conectado);

    // 3.- Usando funciones
    // Llamamos un funcion que calcule el el riesgo
    let vulnerabilidades = 5;
    let riesgo =  calcular_riesgo(vulnerabilidades);

    println!("Vulnerabilidades detectadas {}", vulnerabilidades);
    println!("Nivel de riesgo calculado {}", riesgo);

}

// Aqui vamos a declarar una funcion para poder llamarla en la principal
fn calcular_riesgo(cantidad_vulns: i32) -> i32{
    let factor_riesgo = 2;

    cantidad_vulns * factor_riesgo      // Estudiar conceptos de como el ";" afecta a los resultados void
}