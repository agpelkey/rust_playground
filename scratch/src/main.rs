
// ====================================
//          Slices
// ====================================
/*
fn main() {

    let my_string = String::from("hello world");

    // 'first_word' works on slices of 'String's, whether partial or whole
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // 'first_word' also works on references to 'String's, which are equivalent to whole
    // slices of 'String's
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // 'first_word' works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *area* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);

}

fn first_word(s: &str) -> &str {
   let bytes = s.as_bytes();

   for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
   }

   &s[..]
}
*/


/* ===========================
 *      Using Structs to
 *      Structure Related Data
  =========================== */

fn main() {
    /*
   let mut user1 = User {
        active: true,
        username: String::from("testusername123"),
        email: String::from("test@test.com"),
        sign_in_count: 1,
   };
   */

    let mut user1 = build_user(String::from("test@test.com"), String::from("apelkey"));

   println!("{:?}", user1);

   let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("anotherexample.com"),
        sign_in_count: user1.sign_in_count,
   };

   println!("{:?}", user2);
   
}

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}






