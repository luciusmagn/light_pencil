extern crate pencil;

use pencil::Pencil;
use pencil::method::Get;
use pencil::{Request, PencilResult};


fn user(r: &mut Request) -> PencilResult {
    let user_id = r.view_args.get("user_id").unwrap();
    Ok(format!("user {}", user_id).into())
}


fn main() {
    let mut app = Pencil::new("/web/example");
    app.route("/user/<int:user_id>", &[Get], "user", user);
    app.run("127.0.0.1:5000");
}
