use std::{fs};


fn main() {
/////////////////////////////////
//Exercitiul 1
/////////////////////////////////
let s = fs::read_to_string("ex1.txt").unwrap();

let mut lgmax = "";
let mut lgmaxb = "";

for linie in s.lines() {
    if linie.len() > lgmaxb.len() {
        lgmaxb=linie;
    }
    if linie.chars().count() > lgmax.chars().count() {
        lgmax = linie;
    } 

}

println!("(ex1)Cea mai lunga propozitie dupa bytes : {} \n",lgmaxb);
println!("(ex1)Cea mai lunga propozitie dupa caractere : {} \n",lgmax);

///////////////////////////////////
//Exercitiul 2
/////////////////////////////////
let mut rez = String::new();
let a = fs::read_to_string("ex2.txt").unwrap();

for c in a.chars() {
    let rezc;
    if c.is_ascii_uppercase() {
        rezc = ((((c as u8 - b'A') + 13)%26)+b'A') as char;
        rez.push(rezc);
    } else if c.is_ascii_lowercase(){
        rezc = ((((c as u8 - b'a') + 13)%26)+b'a') as char;
        rez.push(rezc);
    }  else {
    println!("Eroare: caracterul nu este ASCII");
    break;
    }
}

println!("(ex2)Textul dupa ROT13 este : {} \n",rez);


/////////////////////////////////
// Exercitiul 3
/////////////////////////////////
let abrevieri = [
    ("pt", "pentru"),
    ("ptr", "pentru"),
    ("dl", "domnul"),
    ("dna", "doamna"),
];
let x = "Am fost la dl Matei pt              ca m-a invitat";

for cuvant in x.split(' ') {
    let mut cuvant_nou = cuvant;

    for (abreviere,forma_completa) in abrevieri {
        if abreviere == cuvant_nou {
            cuvant_nou = forma_completa;
        }
    }
    print!("{} ",cuvant_nou);

}
println!(" ");
println!(" /////Problema 4 :");
///////////////////////////////////
// Exercitiul 4
/////////////////////////////////
let s = fs::read_to_string("/etc/hosts").unwrap();
let mut nume="";
let mut ip="";
for linie in s.lines() {
    if linie.trim().starts_with('#') {
    continue;
}
    for (nr,cuvant) in linie.split_whitespace().enumerate(){
        if nr==0 {
           ip=cuvant;
        }else if nr==1 {
            nume=cuvant;
        }
    }
    println!("{} => {}",ip,nume);
}

/////////////////////////////////
////////////////////////////////////


}
