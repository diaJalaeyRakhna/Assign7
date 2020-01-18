/*#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}*/

pub mod military_drone {
    pub mod war_time {
        pub fn load_amunition() {
            println!("\nLoad required amunition.");
        }

        pub fn fire_target(latitude:f32, longitude:f32) {
            println!("\nFire amunition on target at location ({}, {}).", latitude, longitude);
        }
    }

    pub mod peace_time {
        pub fn get_ready() {
            println!("\nMend broken/affected parts and re-fuel.");
        }
    }
}

pub mod commercial_drone {
    pub fn deliver_package(latitude:f32, longitude:f32) {
        println!("\nDeliver package at prescribed location ({}, {}).", latitude, longitude);
    }

    pub fn make_video() {
        println!("\nCommercial drones are also used to make videos of events.");
    }
}
