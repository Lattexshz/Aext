pub struct CustomCommand {}

pub fn info() {
    println!("Aext - A hackable build tool with Rust {}",env!("CARGO_PKG_RUST_VERSION"));
}

/// Lists the currently available extensions
pub fn list() {
    unsafe {
        println!("{} Aext scripts loaded.\n", crate::EXTENSIONS.len());
        for e in crate::EXTENSIONS.clone() {
            println!("--------------------------------------");
            println!("Name:{} Version:{}", e.name, e.version);

            // Authors (Optional)
            match e.authors.len() == 0 {
                true => {}
                false => {
                    print!("authors: ");
                    for a in e.authors {
                        print!("{},", a);
                    }
                    println!("");
                }
            }
            // Description (Optional)
            match e.description.is_empty() == true {
                true => {}
                false => {
                    println!("{}", e.description);
                }
            }
        }
        println!("--------------------------------------");
    }
}
