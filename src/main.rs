use std::collections::VecDeque;
fn main() {
    let  x = String::from("7+15");
    //assert_eq!(eval("190 + 9 "),[190,9]);
    //assert_eq!(eval("190 + 9 / 90 "),[190,9,90]);
    println!("{:?}",eval(&x[..]));
}
#[derive(Clone)]
enum Token {
    Digit,
    Operator,
    OpenParen,
    CloseParen,
    Exp,
    Var,
    Sum,
    Prod,
    Div,
    Res,
    Mon
}
#[derive(Clone)]
struct Expr {
    coef: f64,
    var : String,
    exp : f64
}
#[derive(Clone)]
enum Typed {
    Monomio(Token,Expr),
    Operador(Token,String)
}
//impl Clone for Typed {
    //fn clone(&self) -> Self {
        //*self
    //}
//}
fn eval(expression:&str)->VecDeque<String>{
    let input :Vec<char>= expression.chars().filter( |c|  !c.is_whitespace()).collect();
    let (mut values ,mut ops,mut i, mut versions) :(VecDeque<String> , VecDeque<char>,usize, Vec<&str>) = (VecDeque::new(), VecDeque::new(), 0 , Vec::new());
    let (mut aux, mut ban ):(String, bool)  = ("".to_string(), false);
    let (mut coef ,mut var , mut exp): (String,String,String)= ( "".to_string(),"".to_string(),"".to_string());
    let mut ast:VecDeque<Typed>= VecDeque::new();
    while  i < input.len() + 1{
        let mut pos = if i == input.len() {Token::Operator} else {tipo(input[i]) };
        match pos  {
            Token::Digit => {
                if !ban { coef.push(input[i]); }else { exp.push(input[i]); }
                aux.push(input[i]);
            },
            Token::Exp =>{
                ban = true;
            },
            Token::Var =>{
                var.push(input[i]);
            }
            Token::Operator =>{
                let ( a , b ,  c) ;
                if coef.is_empty() { a = 1.0 } else { a = convert(&coef,'f') }
                if exp.is_empty() {c = 1.0 } else { c = convert(&exp,'f') }
                if var.is_empty() { b = " ".to_string()} else { b = var.clone()}
                coef.clear();exp.clear(); var.clear();
                ast.push_back(Typed::Monomio(Token::Mon,Expr{
                  coef: a,
                  var : b,
                  exp : c
                }));
                ban = false;          
               if i < input.len() {ast.push_back(Typed::Operador(oper(input[i]),input[i].to_string()))}
            }
            Token::CloseParen => {
               ast.push_back(Typed::Operador(Token::CloseParen,input[i].to_string()));
            }
            Token::OpenParen => {
               ast.push_back(Typed::Operador(Token::OpenParen,input[i].to_string()));
            }
            _ => {
                ban = false;
                if !aux.is_empty() {
                    values.push_back(aux.clone());
                    aux.clear();
                }
                values.push_back(input[i].to_string());
            }
        }
        i+=1;
    }
    if !aux.is_empty() {
        values.push_back(aux);
    }
    i = 0;
    loop {
        for (idx, x)in ast.clone().iter().enumerate(){
            match x {
                Typed::Monomio(tok,expr) => println!(" {} {} ^ {}" , expr.coef,expr.var,expr.exp), 
                Typed::Operador(tok,literal) => {
                    //match tok {
                        //Token::Sum
                    //}
                }
            }
        }
        if i > 3{
           break; 
        }
        i+=1;
        println!("---");
    }
    values
}
fn tipo(actual:char) -> Token {
    match actual {
        'A'..='Z' | 'a'..='z' => Token::Var,
        '+'|'/'|'-'|'*' => Token::Operator,
        '0'..='9' | '.'| '_'=> Token::Digit,
        '^' => Token::Exp,
        '(' => Token::OpenParen,
        ')' => Token::CloseParen,
        _ => Token::Var
    }
}
fn convert(number:&String, t:char) -> f64 {
    match t{
       //'i' => number.parse::<i32>().unwrap(); 
       'f' => number.parse::<f64>().unwrap(),
       _ => -1.0
    }
}
fn oper(op:char)-> Token{
    match op {
        '-' => Token::Res,
        '+' => Token::Sum,
        '*' => Token::Prod,
        '/' => Token::Div,
        _ => Token::Res
    }
}
