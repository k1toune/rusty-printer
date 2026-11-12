
use printers::{get_default_printer, get_printers};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.get(1).map(String::as_str) {
        Some("list") => {
            for printer in get_printers() {
                println!("{:?}", printer);
            }
        }

        Some("print") => {
            let Some(file) = args.get(2) else {
                eprintln!("Usage: rusty-printer print <file>");
                return;
            };

            let Some(printer) = get_default_printer() else {
                eprintln!("No default printer configured.");
                return;
            };

            match printer.print_file(file, Default::default()) {
                Ok(job) => println!("Print job submitted: {:?}", job),
                Err(err) => eprintln!("Printing failed: {:?}", err),
            }
        }

        _ => {
            println!("Rusty Printer");
            println!("  list          List available printers");
            println!("  print <file>  Print a document");
        }
    }
}
