mod child;

mod parent {
    pub fn parent_function() {
        println!("parent function",)
    }
}

fn main() {
    println!("main function",);
    child::use_parent_function()
}
