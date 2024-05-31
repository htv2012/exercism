#[test]
fn hello_world() {
    assert_eq!("Hello, World!", hello_world::hello());
}

#[test]
fn another_test(){
    assert_eq!("Foo", hello_world::hello());
}