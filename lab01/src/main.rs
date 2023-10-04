fn is_prime(x:i8) -> bool
{
    if x < 2{
        return false;
    }

    let mut i :i8 = 2;
    while i*i <= x{
        if x % i == 0
        {
            return false;
        }
        i = i + 1;
    }
    return true;
}
fn coprime(mut a:i8, mut b:i8) -> bool
{
    let mut r;
    while b != 0{
        r = a % b;
        a = b;
        b = r;
    }
    return a == 1;
}
fn beer_song()
{
    let mut counter: i32 =  99;

    while counter >= 2 {
        println!("{} bottles of beer on the wall,", counter);
        println!("{} bottles of beer.", counter);
            println!("Take one down, pass it around,");
        counter = counter - 1;

        if counter == 1{
            println!("{} bottle of beer on the wall,\n", counter);
        }
        else{
            println!("{} bottles of beer on the wall,\n", counter);
        }
    }
    println!("1 bottle of beer on the wall,");
    println!("1 bottle of beer.");
    print!("Take one down, pass it around,\nNo bottles of beer on the wall.");
    

}
fn main() {
    let mut i:i8 = 0;
    while i <= 100
    {
        if is_prime(i) == true{
            println!("{} este prim", i);
        }
        i = i + 1;
    }

    i = 0;
    let mut j:i8;
    while i <= 100{

        j = 0;
        while j <= 100{
            if coprime(i,j) == true{
                println!("{} si {} sunt co-prime", i, j);
            }
            else {
                println!("{} si {} nu sunt co-prime", i, j);
            }
            j = j + 1;
        }
        i = i + 1;
    }
    beer_song();
}
