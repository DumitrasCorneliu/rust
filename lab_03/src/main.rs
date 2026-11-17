#[derive(Debug)]
enum Erori {
    Overflow,
}
#[derive(Debug)]
enum CharError {
    NotAscii,
    NotDigit,
    NotBase16Digit,
    NotLetter,
    NotPrintable,
}
// Problema 1
//////////////////////////////////////
fn este_prim(x: u16) -> bool {
    if x < 2 {
        return false
    }
    for i in 2..x {
        if x.is_multiple_of(i){
            return false
        }
    }
    true
}

fn next_prime(x: u16) -> Option<u16> {
    (x + 1..65535).find(|&i| este_prim(i))
}
//////////////////////////////////////
//////////////////////////////////////
/// Problema 2
fn check_add(x: u32 , y: u32) -> u32{
let rez: u64 = x as u64 + y as u64;
    if rez > 4294967295 {
        panic!("Rezultatul depaseste tipul u32");
    }
        rez as u32
 }
 fn check_mul(x: u32 , y: u32) -> u32{
let rez: u64 = x as u64 * y as u64;
    if rez > 4294967295 {
        panic!("Rezultatul depaseste tipul u32");
    }
        rez as u32
 }

 //////////////////////////////////////
//////////////////////////////////////
/// Problema 3
fn check_add1(x: u32 , y: u32) -> Result<u32,Erori>{
let rez: u64 = x as u64 + y as u64;
    if rez > 4294967295 {
       return Err(Erori::Overflow);
    }
       Ok(rez as u32)
 }
 fn check_mul1(x: u32 , y: u32) -> Result<u32,Erori>{
let rez: u64 = x as u64 * y as u64;
    if rez > 4294967295 {
       return Err(Erori::Overflow);
    }
        Ok(rez as u32)
 }
 //////////////////////////////////////
//////////////////////////////////////
/// Problema 4
fn to_uppercase(x: char) -> Result<char,CharError>{
    if !x.is_ascii_alphabetic() {
        return Err(CharError::NotLetter);
    }
    Ok(x.to_ascii_uppercase())
}

fn to_lowercase(x: char) -> Result<char,CharError>{
    if !x.is_ascii_alphabetic() {
        return Err(CharError::NotLetter);
    }
    Ok(x.to_ascii_lowercase())
}

fn print_char(x: char) -> Result<char,CharError>{
        if x.is_ascii_control() {
        return Err(CharError::NotPrintable);
    }

    print!("{}", x);
    Ok(x)
}

fn char_to_number(x: char) -> Result<u32,CharError>{
    if !x.is_ascii() {
        return Err(CharError::NotAscii);
    }
if x.is_ascii_digit() {
    Ok(x as u32 - '0' as u32)
} else {
    Err(CharError::NotDigit)
}
}

fn char_to_number_hex(x: char) -> Result<u32,CharError>{
    if !x.is_ascii() {
        return Err(CharError::NotAscii);
    }
   match x.to_digit(16) {
        Some(n) => Ok(n),
        None => Err(CharError::NotBase16Digit),
    }
}

fn print_error(error: CharError) {
    match error {
        CharError::NotAscii => {
            println!("Caracterul nu este ASCII");
        }
        CharError::NotDigit => {
            println!("Caracterul nu este o cifra");
        }
        CharError::NotBase16Digit => {
            println!("Caracterul nu este o cifra hexazecimala");
        }
        CharError::NotLetter => {
            println!("Caracterul nu este o litera");
        }
        CharError::NotPrintable => {
            println!("Caracterul nu poate fi printat");
        }
    }
}



fn main() {
    // Problema 1
     let mut x: u16 = 1;

while let Some(prime) = next_prime(x) {
    println!("{}", prime);
    x = prime;
}
    // Problema 2
    println!("Rezultatul 10+5 este {} \n",check_add(10,5));
    println!("Rezultatul 10*5 este {} \n",check_mul(10,5));

    // Problema 3
     println!("Rezultatul 10+5 este {:?}\n", check_add1(10, 5));
    println!("Rezultatul 10*5 este {:?}\n", check_mul1(10, 5));

    // Problema 4
    let caps = to_uppercase('a');
    println!("{:?}", caps);

    let lower = to_lowercase('A');
    println!("{:?}", lower);

       let result = print_char('A');
    println!("{:?}", result);

    let number = char_to_number('5');
    println!("{:?}", number);

    let hex = char_to_number_hex('F');
    println!("{:?}", hex);

    print_error(CharError::NotAscii);
}
