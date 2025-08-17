fn main() {
    println!("Inicio do programa");
    const X: i32 = 5;
    let y: i32 = 6;
    let mut z: i32 = 7;
    z = z + 1;
    println!("No inicio os valores são: x{X}, y{y}, z{z}");
    //no run o compiler irá mostrar a macro x5, y6, z8 (identação z+1)
    //Vamos criar um block interno e ver a ordem de execução sendo alterada
    //pois o block é executado em primeiro lugar
    {
        const X: i32 = 55;
        let y: i32 = 66;
        let mut z: i32 = 77;
        z = z + 1;
        println!("Dentro do Block interno os valores são esse x{X}, y{y}, z{z}");
    }
    
    println!("after internal Block os valores são esses: x{X}, y{y}, z{z}");
    
    println!("resumindo os valores do bloco interno das variáveis podem ser alterados sem perderem o seu valor original")
    //isso gera problemas pois poderemos manipular os valores das variáveis 
}

