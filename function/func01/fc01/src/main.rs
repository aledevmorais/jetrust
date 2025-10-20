//Funções em Rust Comentado

fn outra_funcao() {
    println!("outra_funcao a macro vai executar só que não atribui parâmetro nenhum.");
}
//mesmo rust sendo de tipagem forte ele executou no cargo run uma funçao sem tipo
//alem de não ter parâmetro eu não atribui tipo. mas ele executou a macro
//na função main aonde executo esta macro ela assumiu o valor de "nada"
//o tipo de retorno é implicitamente () unit, que é o equivalente a "nada" em rust
fn outra_funcao_com_parametro(x: i32) {
    println!("outra_funcao_com_parametro agora vem com a tipagem i32 inteiro")
}
fn print_label_measurement(valor: f64, unidade: char) {
    //na hora que eu chamar a função no main terei que passar 2 parametros "tipagem forte"
    println!("A medida é: {valor} {unidade}.");
}
fn somar(x:i32, y:i32) -> i32 {
    //-> esse sinal pede comando return tenho que digitar return senão da erro. e  return é i32
    return x + y;
}

fn main() {
    println!("Hello, world!");
    outra_funcao();
    outra_funcao_com_parametro(5000);
    print_label_measurement(123.4,'m');

    let x = 999;
    print_label_measurement(x as f64,'v');
    let sumxy = somar(33, 34);
    println!("O valor da somar é: {sumxy}");

}
