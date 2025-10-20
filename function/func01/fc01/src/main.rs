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

fn main() {
    println!("Hello, world!");
    outra_funcao();
    outra_funcao_com_parametro(5000);
}
