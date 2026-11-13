// Rusty Printer
// Made by K1toune
// ──────────────────
//     K1T0UNE <3
use printers::{get_default_printer, get_printers};
use printers::common::base::job::PrinterJobOptions;

fn list_printers() {
    for printer in get_printers() {
        println!("{:?}", printer);
    }
}

fn print_file(file: &str) {
    let Some(printer) = get_default_printer() else {
        eprintln!("No default printer configured.");
        return;
    };

    match printer.print_file(file, PrinterJobOptions::none()) {
        Ok(job) => println!("Print job submitted: {:?}", job),
        Err(err) => eprintln!("Printing failed: {:?}", err),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if let Some(command) = args.get(1) {
        if command == "list" {
            list_printers();
            return;
        }

        if command == "print" {
            if let Some(file) = args.get(2) {
                print_file(file);
                return;
            }

            eprintln!("Usage: rusty-printer print <file>");
            return;
        }
    }

    println!("Rusty Printer");
    println!("  list          List available printers");
    println!("  print <file>  Print a document");
}
