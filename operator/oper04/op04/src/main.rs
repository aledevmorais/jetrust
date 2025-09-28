fn main()
{
    let aa: [i32; 5] = [1, 2, 3, 4, 5];
    //um array de 5 posições
    let meses: [&str; 12] = ["Janeiro", "Fevereiro", "Março", "Abril", "maio", "Junho", "Julho", "Agosto", "Setembro", "Outubro", "Novembro", "Dezembro"];
    let bb: [i32; 5] = [1, 2, 3, 4, 5];

    let cc: [i32; 5] = [3; 5];
    // nesse array tambem contem 5 elementos pois o ";" significa numero 3 repetido 5 vezes então 5 elementos
    let dd: [i32; 2] = [3, 5];
    //nesse array tem 2 elementos o inteiro 3 e o inteiro 5 ou seja 2 elementos i32
    //println!("O itens desse array sao os {aa}");
    // seu eu fizer isso vai dar erro pois não estou implementando traits display
    // vamos fazer uma forma truncada anotação traits debug {:?} ok
    println!("Os itens desse array são os: {:?}", cc);
    println!("Os itens desse array são os: {:?}", dd);
    // quando gero cargo run -> Os itens desse array são os: [3, 3, 3, 3, 3]
    // Os itens desse array são os: [3, 5]
    println!("O elemento 2 do array meses é o : {:?}", meses[2]);
    //praticamente igual para qualquer javascriptero kkk
    //o elemento 2 da array meses partindo do 0 "janeiro" é março
}
