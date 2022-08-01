mod drop;
mod deref;
mod r#box;
mod rc;

fn main() {
  drop::run();
  deref::run();
  rc::run();
}

