// lifetimes3.rs
//
// Lifetimes are also needed when structs hold references.
//
// Execute `rustlings hint lifetimes3` or use the `hint` watch subcommand for a


struct Book<'a> {
    author: &'a str,
    title: &'a str,
}
impl<'a> Book<'a> {

    fn print_info(&self) {
        println!("{} by {}", self.title, self.author);
    }
}
fn main() {
    let name = String::from("Jill Smith");
    let title = String::from("Fish Flying");
    let book = Book { author: &name, title: &title };
    book.print_info();
}
