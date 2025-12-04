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

    // *--- Nuevo Codgio--------
    // *Definiremos un arreglo de puertos comunes (Son enteros i32)
    // *[i32; 4] =  significa "Tipo i32, tamaño 4"
    let comon_ports: [i32;4]= [80,443,22,21];

    // *Accedemos ficticiamente al puerto SSH(El indice comienza en  0)
    // *Indice 0 = 8 ....
    let ssh_port = comon_ports[2];
    println!("Escaneando el puerto SSH {}", ssh_port);

    // * Ejemplo ficticio
    // Buffer de memoria llena (Muy comun en exploits)
    // Esto crea un arreglo de 5 ceros : [0,0,0,0,0]
    let buffer = [0; 5];
    println!("Buffer de memoria inicializado : {:?}", buffer);      // Aqui usamos el :? entre parentesis para poder imprimir estructuras complejas como un array


    // --- LO QUE FALTABA: TUPLAS ---
    
    // Una tupla nos permite agrupar datos de DIFERENTE tipo.
    // Ejemplo: (Dirección IP, Puerto, Protocolo Activo)
    //         (String,       i32,    bool)
    let credencials: (&str, i32, bool) = ("192.168.0.50", 800, true);

    // Para acceder a los datos  de una tupla usamos el punto "."
    println!("--Datos del servidor--");
    println!("IP Objetivo {}", credencials.0);
    println!("Puerto : {}", credencials.1);

    // Desestructuracion: Sacar todos los valores a variables sueltas
    let (ip, puerto, estado)= credencials;
    println!("El servidor {} en el puerto {} esta activo: {}", ip, puerto, estado);
}

// Aqui vamos a declarar una funcion para poder llamarla en la principal
fn calcular_riesgo(cantidad_vulns: i32) -> i32{
    let factor_riesgo = 2;

    cantidad_vulns * factor_riesgo      // Estudiar conceptos de como el ";" afecta a los resultados void
}