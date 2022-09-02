## bevy_double_res
Straightforward implementation of double buffering of ordinary [bevy engine](https://bevyengine.org/) resources

## How to use

First, create your buffer:

```rust
fn main() {
    use bevy_double_res::prelude::*;
    
    let mut tuple = (10, 20).into_double_buf();
    
    //...
}
```
or
```rust
fn main() {
    use bevy_double_res::prelude::*;
    
    let mut tuple = DoubleBuffer::new((10, 20));
    
    //...
}
```

Second, mutate items:

```rust
fn main() {
    //...

    tuple.apply(|current, next| {
        next.0 = current.1;
        next.1 = current.0;
    });
    tuple.swap();
    
    //...
}
```
or
```rust
fn main() {
    //...
    
    let (current, next) = tuple.split_ordered();
    next.0 = current.1;
    next.1 = current.0;
    tuple.swap();
    
    //...
}
```
Don't forget about the swap!

Last, display contents of your buffer:
```rust
fn main() {
    use bevy_double_res::prelude::*;
    
    let mut tuple = (10, 20).into_double_buf();
    
    tuple.apply(|current, next| {
        next.0 = current.1;
        next.1 = current.0;
    });
    tuple.swap();
    
    println!("{:?}", tuple.current()); // outputs: (20, 10)
}
```
## Using in systems

Creating resource is the same:

```rust
fn setup(mut commands: Commands) {
    let tuple = (10, 20).into_double_buf();

    commands.insert_resource(tuple);
}
```

Accessing resource in systems:
* readonly? then use **DoubleRes** (matches **Res**)
* mutable? then use **DoubleResMut** (matches **ResMut**)

```rust
fn circular_dependent_system(mut tuple: DoubleResMut<(i32, i32)>) {
    tuple.apply(|current, next| {
        next.0 = current.1;
        next.1 = current.0;
    });
    tuple.swap();
}
```

Also see an [example](https://github.com/necromfox/bevy_double_res/blob/main/examples/simple/main.rs) of usage