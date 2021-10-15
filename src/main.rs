use::std::io;

fn main() {
    //get values from user
    let current = get_user_input("How high is the value you want to grow?");
    let needed = get_user_input("How high do you need that value to be?");
    let growth = get_user_input("What's the growth rate per year? (in percent)");

    //calculate years
    let years = get_years(current as f32, needed as f32, growth as f32);

    //print output
    println!("Right now your value is {}. With a growth rate of {}% you will have to wait {} years until it has reached {}.",current, growth, years, needed)
}

/*
 * gets input from user and returns it as f32
 * takes a string to show the user as parameter
 */
fn get_user_input(message: &str) -> u32{
    loop {
        //show message to user
        println!("{}", message);

        //get input from user
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Couldn't read line 😥.");

        //convert to f32
        let input: u32 = match input.trim().parse(){
            Ok(num) => num,
            Err(_) => {continue;},
        };
        return input;
    }
}

/*
 * calculates how many years need to pass before you get the desired value
 * returns 0 if you have already reached it
*/
fn get_years(current: f32, needed: f32, growth: f32) -> u32{
    if current >= needed{
        println!("Lucky you!!! Your value is already high enough. 😎");
        return 0;
    }

    let mut years: u32 = 0;
    let mut counter = current;

    // convert percent to decimal
    let growth_value = &(growth / 100.0 + 1.0);
    println!("Growth value: {}", growth_value);

    while counter < needed{
        counter *= growth_value;
        years += 1;
    }
    println!("Your value has grown to {}.",counter);
    return years;
}
