fn main(){
    println!("Hola rust");
    // Comentario normal
    /*
    comentarios entre varias lineas 
    */
    println!("Siempre se tiene que poner el ; para que corra varias lienas");

    //Variables:
    let mystring = "This is a text chain";
    // Variable inmutable Mystring = "Otro cambio"
    println!("Esto es un string: {} /n Esta variable es inmutable ya que no se usa en la definicion", mystring);


    println!("Este es un ejemplo de una variable mutable la cual se declara de la siguiente manera let mut");
    let mut second = "Este es el primer valor de la variable mutable";
    println!("primer valor de la variable {}",second);
    //Cambio de valor a la mutable
    second = "Este es el segundo valor de la varibale mutable";
    println!("Segundo valor de la variable{}:",second);
    //error second = 6;
    
}