/* PIAIC52062 (Nadia Sheikh: naadia@hotmail.com: ~Dia)
Q3. Write a Rust library. Make a library package. Make a module with suitable name in library.
    Make a sub module with another name in first module. Define a simple function in sub module.
    Make a binary package. Add your library in dependencies in cargo.toml.
    Use your library in main.rs. Call that function from fn main.
*/

use drone;

fn main() {
    println!("\n\nAssignment 7: Quest3: Modules and Libraries\n~!~!~!~!~!~!~!~!~!~!~!~!~!~!~!~!~!~!~!~!~!~");
    
    drone::military_drone::peace_time::get_ready();
    drone::military_drone::war_time::load_amunition();

    drone::commercial_drone::make_video();
}