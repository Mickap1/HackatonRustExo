mod book_mod {
}

fn main() {
    let my_awesome_book = book_mod::Book::new("My Incredible Book");

    book_mod::read_book(&my_awesome_book);
}
