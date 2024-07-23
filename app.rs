fn main(){
    let mut x = 1;
    x += 1;
    assert_eq!(x, 2);
    println!("success");
}

// here in above code we are creating an variable that is taking a number

// let for variable mut for mutable so that can change over the period of time

// print , println and assert_eq is the built in function you can call it by name and ! or ::  

fn main(){
    let x : i32 = 10;
    {
        let y: i32 = 10;
       println!("{}",y);
    }
    println!("{} {}",x, y);
    


    
}

// {} can used to define the scope 
// if we want to print more than two varible you can you "{}" if two then "{}{}" and so on