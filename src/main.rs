use std::collections::VecDeque;
fn main() {
    let  x = String::from("(189 - 90)/2");
    assert_eq!(eval("190 + 9 "),[190,9]);
    assert_eq!(eval("190 + 9 / 90 "),[190,9,90]);
    println!("{:?}",eval(&x[..]));
}
enum Token {
    Digit,
    Operator,
    OpenParen,
    CloseParen
}
fn eval(expression:&str)->VecDeque<i32>{
    let input :Vec<char>= expression.chars().filter( |c|  !c.is_whitespace()).collect();
    let (mut values ,mut ops,mut i, mut versions) :(VecDeque<i32> , VecDeque<char>,usize, Vec<&str>) = (VecDeque::new(), VecDeque::new(), 0 , Vec::new());
    let (mut aux, mut ban ):(String, bool)  = ("".to_string(), true);
    loop {
        if i >= input.len(){
            break;
        }
        match tipo(input[i]) {
            Token::Digit => {
                aux.push(input[i]);
                ban = true;
            },
            Token::Operator => {
                ops.push_back(input[i]);       
                ban = false;
            },
            Token::OpenParen => {
                ban = false;
            },
            Token::CloseParen =>{
                ban = false;
            }
        }
        if !aux.is_empty() && !ban {
            values.push_back(convert(&aux));
            aux.clear();
        }
        i+=1;
    }
    if !aux.is_empty() {
        values.push_back(convert(&aux));
        aux.clear();
    }
    
    values
}
fn tipo(actual:char) -> Token {
    //let Operatores = vec!['+','\\','-','*'];
    return if actual.is_digit(10) || actual == '.' {
        Token::Digit 
    } else { 
        if vec!['+','/','-','*'].contains(&actual) {
            Token::Operator
        } else {
            if actual == '('{
                Token::OpenParen
            }else{
                Token::CloseParen
            }
        }
    }
}
fn convert(number:&String) -> i32 {
   return number.parse::<i32>().unwrap(); 
}
