/* PIAIC52062 (Nadia Sheikh: naadia@hotmail.com: ~Dia)
Q2: Write a Rust Program. Make a library (lib.rs) alongwith main.rs.
    Make a module with suitable name in library. Make a sub module with another name in first module.
    Define a simple function in sub module. Use that library in main.rs.
    Call that function from fn main.
*/

mod fliers;

fn main() {
    println!("\n\nAssignment 7: Quest2: Modules and Libraries\n~!~!~!~!~!~!~!~!~!~!~!~!~!~!~!~!~!~!~!~!~!~");
    
    fliers::flying_object::helicopter::fly();
    fliers::flying_object::aeroplane::fly();
    fliers::flying_object::ufo::fly();
}