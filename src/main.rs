use::std::io;

fn main() {
    //get values from user
    let current = get_user_input("How high is the value you want to grow?");
    let needed = get_user_input("How high do you need that value to be?");
    let growth = get_user_input("What's the growth rate per time unit? (in percent)");

    //calculate years
    let years = get_years(current as f64, needed as f64, growth as f64);

    //print output
    println!("Right now your value is {}. With a growth rate of {}% you will have to wait {} time units until it has reached {}.",current, growth, years, needed)
}

/**!
 * gets input from user and returns it as f32
 * takes a string to show the user as parameter
 * reruns until the user enters a valid u32
 **/
fn get_user_input(message: &str) -> u32{
    loop {
        //show message to user
        println!("{}", message);

        //get input from user
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Couldn't read line 😥.");

        //convert to u32
        let input: u32 = match input.trim().parse(){
            Ok(num) => num,
            Err(_) => {continue;},
        };
        return input;
    }
}

/**!
 * calculates how many years need to pass before you get the desired value
 * returns 0 if you have already reached it
**/
fn get_years(current: f64, needed: f64, growth: f64) -> f64{
    if current >= needed{
        println!("🦀🦀🦀 Lucky you!!! Your value is already high enough. 🦀🦀🦀");
        return 0.0;
    }
    // initialize variables

    // convert percent to decimal
    let growth_rate = &growth / 100.0 + 1.0;

    // solve: (needed = current * (growth_rate)^time) for time
    let time =
        f64::log10(needed / current) /
        f64::log10(growth_rate);

    return time;
}
