pub struct CustomCommand {
    
}

/// Lists the currently available extensions
pub fn list() {
    unsafe {
        println!("{} Aext scripts loaded.\n", crate::EXTENSIONS.len());
            for e in crate::EXTENSIONS.clone() {
                println!("--------------------------------------");
                let plugin = e.plugin.unwrap();
                println!(
                    "Name:{} Version:{}",
                    plugin.name.unwrap(),
                    plugin.version.unwrap()
                );

                // Authors (Optional)
                match plugin.authors {
                    None => {

                    }
                    Some(a) => {
                        print!("authors: ");
                        for a in a {
                            print!("{},",a);
                        }
                        println!("");
                    }
                }
                // Description (Optional)
                match plugin.description {
                    None => {}
                    Some(d) => {
                        println!("{}",d);
                    }
                }
            }
            println!("--------------------------------------");
        }
}