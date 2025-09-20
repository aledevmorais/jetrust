fn main()
{
    let t: bool = true;
    let f: bool = false;

    let x: bool = t && f;
    //operadores lógicos and &&, ou seja, t mais f
    let y: bool = t || f;
    //operadores lógicos or ||, ou seja, esse(t) ou este(f)
    //obs.: pra não dar redundância vc viu que o compilador permitiu a sentença
    //olha o crash lógico: verdadeiro(t) e falso(f). Mesmo rust tendo segurança de compilação
    //ele identifica como válida a sentença pois não enxerga a lógica por trás dessa sentença
    //e NENHUMA OUTRA LINGUAGEM NO MUNDO ENXERGA ISSO

}
