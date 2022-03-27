mod login;

fn main() {

}

pub struct AuthenticationResult {

}

fn basic_program_flow(handler: fn()){

    /*

    Basic program flow goes as follows:

    Tries to execute handler. If receives unauthenticated, will first check expiration token validity.
    If unable to get a valid token

     */



}
fn authenticate() -> AuthenticationResult {

    /*

    Initializes an authentication flow. Opens a browser in the twitter login page and once user has signed in will capture redirection
    and parse tokens from the
     */
    return AuthenticationResult {}
}

