mod order_data;
mod order_actions;
mod engine;

fn main() {
    
    let s1 = String::from("Hello");
    let res;

    {
        let s2 = String::from("World.");
        res = str_compare(&s1, &s2);
        println!("{}", res);    
    }

}

fn str_compare<'a, 'b> (string_one: &'a str, string_two: &'b str) -> &'b str {

    println!("{}", string_one);
    string_two
}