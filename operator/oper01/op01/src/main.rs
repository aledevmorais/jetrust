fn main()
{
    let chassi: i32 = 123456;
    let acel_max: f64 = 3.0;
    let acel_min: f64 = 10.0;
    let comprimento: i32 = 4;
    let posicao_atual: f64 = -100.0;
    let vel_atual: f64 = 0.0;
    let acel_atual: f64 = 0.0;

    //operadores aritméticos simples
    //+ ou adição

    let sum: f64 = posicao_atual + 10.0;
    // eu posso reatribuir o tipo e adicionar um 1 inteiro ao invés de float
    let sum: i32 = posicao_atual as i32 + 10;
    // aqui de forma lógica eu reatribui um novo tipo as duas let dentro do mesmo escopo
    // em tempo de execução. Mesmo rust oferencendo segurança de tipos na memória o compiler não consegue
    // identificar a redundância lógica apenas a literal de tipos.
    // perguntei para UMA LLM acerca do que fiz ela identifica a função shadowing até ai tudo bem
    // mas ela não identificou um bug lógico "não deixe llm codar" Ela ainda erra em tipos primitivos.


}
