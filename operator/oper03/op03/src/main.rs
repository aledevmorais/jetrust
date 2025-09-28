fn main()
{
    let tup1: (i32, f64, bool) = (500, 6.4, true);
    let tup2: (i32, f64, bool) = (500, 6.4, true);

    //tupla: um tipo de array como uma lista só que imutável
    // a diferença entre tupla ou tuple and array é que a tupla é uma lista imutável de tipos
    //println!("Minha tupla tem: {tup2}");
    //traits on demand - tipos compostos ele não roda
    println!(" Minha tupla tem: {:?} e {:?}", tup1, tup2);

    //agora vou quebrar essa tuple para dar prn line de cada tipo
    //desestruturação ou destructuring em partes
    let (x1, y1, z1) = tup2;
    //destructuring
    println!(" Minha tupla tem desestruturada {x1} {y1} {z1}");
    //caso voce queira desestrutrar uma lista (array não mutável)
    //caso de uso para selecionar dados de uma lista
    //assim como toda array em qualquer linguagem  0 é o item 1 e 1 é o item 2...
    //selecionando
    println!("o item 0 da minha tupla é o: {:?}", tup2.0);
    //tup2.0 esse ponto zero é a posição da lista(array)
}