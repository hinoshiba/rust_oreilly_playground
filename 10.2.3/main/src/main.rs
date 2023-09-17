pub struct Account {
    pub name: String,
    pub language: String,
    pub id: i64,
}

fn main() {
    println!("Hello, world!");

    let account = Account{
        name: "john".to_string(),
        language: "jp".to_string(),
        id: 1
    };

    match account {
        //Account { name, language, .. } => {
        //Account { & name, & language, .. } => { //cant
        Account { ref name, ref language, .. } => {
            println!("{}, {}", name, language); // need ref
            println!("{:p}", &account);
        }
    }
}
