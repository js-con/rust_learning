mod drop;
mod deref;
mod r#box;
mod rc;
mod refcell;
mod tree;

fn main() {
  drop::run();
  deref::run();
  rc::run();
}

