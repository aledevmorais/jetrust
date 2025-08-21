fn main()
{
    println!("Inicio do Programa");
    let x = 5;
    println!("O valor de x inicial é: {x}");
    println!("x + 1; this variable its nothing a mute por isso não compila em rust");
    println!("agora iremos fazer o seguinte: vou criar a new variable x e é sacada bem legal em rust pois a anterior sai de escopo pra entrar a nova atribuição sem null point excpetion");
    let x :i32 = x + 1;
    println!("olha que sacada interessante o valor não é esquecido e nem fica vago  mais é transitado dentro do escopo");
    println!(" rust movimenta o valor de x + 1 que é o que eu pedi em uma nova atribuição sem null point expection");
    println!("this new result off x is : {x}");
    println!("é uma redefinação de x através do let e após indentação é: {x} ");

    {
        let x = x * 2;
        println!("Agora x no escopo ou bloco interno é: {x}");
        println!("Aqui ocorreu que o sistema tinha o valor armazenado de x da sua ultima definição no escopo externo");
        println!("a expressão x * 2 assume que x é 6 x 2 diferente da primeira atribuição que era 5");
        println!("então existe atribuição da reatribuição do escopo interno sem null point exception");
        println!("se voce faz isso sem cuidados em c, c++, c#, javascript, c#... ele compila mas em rust não");
        println!("agora vou fechar o escopo interno e nesse momento a variable irá retornar o seu valor de atribuição que era 6");
    }
    println!("Valor de de x após destruição da atribuição no escopo interno será {x}");
}
