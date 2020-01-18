/* PIAIC52062 (Nadia Sheikh: naadia@hotmail.com: ~Dia)
Q1. Make a module with suitable name in main.rs. Make a sub module with another name in first module.
    Define a simple function in sub module. Call that function from main.
*/

mod shop {
    pub mod sales {
        pub fn record_sales() {
            println!("\n\nHere your sales for the current week are calculated.\n");
        }
    }
}

fn main() {
    println!("\n\nAssignment 7: Quest1: Modules and Libraries\n~!~!~!~!~!~!~!~!~!~!~!~!~!~!~!~!~!~!~!~!~!~");
    shop::sales::record_sales();
}
