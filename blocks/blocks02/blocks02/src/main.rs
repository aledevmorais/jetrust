fn main()
{
    println!("Inicio do Programa");
    let x = 5;
    println!("O valor de x inicial é: {x}");
    //x + 1; this variable its nothing a mute por isso não compila em rust
    //agora iremos fazer o seguinte: vou criar a new variable x e é sacada bem legal em rust pois a anterior sai de escopo pra entrar a nova atribuição sem null point excpetion
    let x: i32 = x + 1;
    //olha que sacada interessante o valor não é esquecido e nem fica vago  mais é transitado dentro do escopo
    //rust movimenta o valor de x + 1 que é o que eu pedi em uma nova atribuição sem null point expection
    println!("this new result off x is : {x}");
    println!("é uma redefinação de x através do let e após indentação é: {x} ");

    {
        let x = x * 2;
        println!("Agora x no escopo ou bloco interno é: {x}");
        //Aqui ocorreu que o sistema tinha o valor armazenado de x da sua ultima definição no escopo externo
        //a expressão x * 2 assume que x é 6 x 2 diferente da primeira atribuição que era 5
        //então existe atribuição da reatribuição do escopo interno sem null point exception
        //se voce faz isso sem cuidados em c, c++, c#, javascript, c#... ele compila mas em rust não
        //agora vou fechar o escopo interno e nesse momento a variable irá retornar o seu valor de atribuição que era 6
    }
    println!("Valor de de x após destruição da atribuição no escopo interno será {x}");

    //posso criar a mesma variavel dentro do bloco interno com valores diferentes do que foi atribuido no bloco externo
    //e logo em seguida usa-las no escopo interno em tempo de execução, logo após sair de escopo volta os valores originais.
    // alem de não ser recomendado pode criar "nulls" o rust não permite rodar mas a sacada de sair do escopo apagar ajuda a mitigar isso.
    //porem é bom entender disso pois a alocação de memoria pra outro valor pode gerar falhas de segurança.
    //o ideal seria não usar o shadowing ou criar uma expressão temporally para deixar claro
    //que o valor da variável é temporario.
    //tem como bloquear o shadowing através do Clippy mostrando warnings no uso de shadowing.
    let spaces: &str = "   ";
    let spaces: usize = spaces.len();
    println!("O valor de espaços é : {spaces} ");
    //o que aconteceu é que let spaces é uma string com 3 espaços depois eu redefini o tipo "entre aspas" para usize só que a sacada aqui pra não confundir
    //é spaces.len é referencia de tamanho e não de tipo disse apenas que o tamanho é de 3 ai no teste lógico o resultado foi 3 pode confudir o let redefine tipo do spaces

    let mut spaces2: &str = "   ";
    println!(" O valor da let mutável spaces2 é : {spaces2}");
    // a variavel spaces2 é mutável então posso alterar o seu tipo
    spaces2 = "qwerty";
    println!("Agora o novo valor da let mutável speces2 é : {spaces2}");

}
