

fn este_prim(n: i32) -> bool 
{
// Prima problema
   if n < 2 {
        return false;
    }
    let mut d=2;
    
    while d<n
    {
        if n%d==0
        {
            return false;
        }
        d+=1;
    }


    true
}
////////////////////////////
//Problema 2

fn cmmdc (a:i32 , b:i32) -> bool
{
    let i;
    let mut rsp=1;
    if a>b 
    {
        i=b;
    } else{
        i=a;
    }
    let mut d=1;
    while d<=i
    {
        if a%d==0 && b%d==0
        {
            rsp = d;
        }
        d+=1;
    }
    
    if rsp==1 
    {
        return true;
    }

    return false;
}

fn coprime(a: i32 , b: i32) -> bool
{
return cmmdc(a, b);
}

fn main() {
    // Prima problema
    let mut x = 0;
    
    while x<=100
    {
        if este_prim(x){
            println!("{}",x);
        }
        x+=1;
    }
    //////////////////////////
    // Problema 2
    
    let mut y = 0;
    

        while y<=100
        {
            let mut z = y;
            while z <= 100
            {
                if coprime(y,z)
                {
                    println!("{} cu {}",y,z);
                }
                z+=1;
            }
            y+=1;
        }


        // Problema 3

        let mut a=99;
        while a!=0
        {
            println!("{} bottles of beer on the wall,",a);
            println!("{} bottles of beer.",a);
            println!("Take one down, pass it around,");
            if a==1
            {
                println!("No bottles of beer on the wall.");
            }else {
                println!("{} bottles of beer on the wall.",a);
            }
            a-=1;
        }

}
