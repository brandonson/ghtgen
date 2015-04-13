extern crate getpass;
extern crate gitoken;

use gitoken::GithubToken;

use std::error::Error;
use std::io::Write;

fn main() {
    let mut stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    print!("Username: ");
    let _ = stdout.flush();
    let mut unameline = String::new();
    let res = stdin.read_line(&mut unameline);

    if let Err(_) = res {
      println!("Failed to get username.");
      return;
    }

    let password = getpass::get_pass("Password: ").expect("Failed to get password");

    print!("Note: ");
    let _ = stdout.flush();
    let mut noteline = String::new();
    let res = stdin.read_line(&mut noteline);

    if let Err(_) = res {
      println!("Failed to get note");
      return;
    }

    let token = GithubToken::create_with_note(unameline.trim(), AsRef::<str>::as_ref(&password), &[gitoken::Scope::Repo, gitoken::Scope::User], noteline.trim());
    match token {
      Ok(ghtok) => println!("Token: {:?}", ghtok.token),
      Err(e) => {
        println!("Error!  Description: {:?}", e.description());
        if let Some(other_err) = e.cause() {
          println!("Cause: {:?}", other_err.description());
        }
      }
    }
}
