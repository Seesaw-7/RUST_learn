#RUST Learning Notes
## 1. Getting started
### 1.2 Hello, World!
`rustc main.rs`
`rustfmt`
If you’re more familiar with a dynamic language, such as Ruby, Python, or JavaScript, you might not be used to compiling and running a program as separate steps. Rust is an ahead-of-time compiled language

### 1.3 Hello, Cargo!
The first line, [package], is a section heading that indicates that the following statements are configuring a package.
In Rust, packages of code are referred to as crates.
Cargo expects your source files to live inside the src directory. The top-level project directory is just for README files, license information, configuration files, and anything else not related to your code.

`cargo new hello_cargo`

`cargo build` 
This command creates an executable file in `target/debug/hello_cargo` rather than in your current directory. Because the default build is a debug build, Cargo puts the binary in a directory named debug. You can run the executable with this command:
`./target/debug/hello_cargo`
Running cargo build for the first time also causes Cargo to create a new file at the top level: Cargo.lock. This file keeps track of the exact versions of dependencies in your project. 

we can also use cargo run to compile the code and then run the resultant executable all in one command:
`cargo run`
Notice that this time we didn’t see output indicating that Cargo was compiling hello_cargo. Cargo figured out that the files hadn’t changed, so it didn’t rebuild but just ran the binary. 

If you had modified your source code, Cargo would have rebuilt the project before running it
`cargo check`

When your project is finally ready for release, you can use 
`cargo build --release `
to compile it with optimizations. This command will create an executable in `target/release` instead of `target/debug`. The optimizations make your Rust code run faster, but turning them on lengthens the time it takes for your program to compile. 

## 2. Programming a Guessing Game
we need to bring the io input/output library into scope. The io library comes from the standard library, known as std:
`use std::io;`
By default, Rust has a set of items defined in the standard library that it brings into the scope of every program. This set is called the prelude
If a type you want to use isn’t in the prelude, you have to bring that type into scope explicitly with a use statement. Using the std::io library provides you with a number of useful features, including the ability to accept user input.
the result of calling String::new, a function that returns a new instance of a String
the stdin function from the io module
The full job of read_line is to take whatever the user types into standard input and append that into a string (without overwriting its contents), so we therefore pass that string as an argument. The string argument needs to be mutable so the method can change the string’s content.

like variables, references are immutable by default. Hence, you need to write &mut guess rather than &guess to make it mutable. 

it also returns a Result value. Result is an enumeration, often called an enum, which is a type that can be in one of multiple possible states. We call each possible state a variant.

The purpose of these Result types is to encode error-handling information.
Result’s variants are Ok and Err. The Ok variant indicates the operation was successful, and inside Ok is the successfully generated value. The Err variant means the operation failed, and Err contains information about how or why the operation failed.

a crate is a collection of Rust source code files. The project we’ve been building is a binary crate, which is an executable. The rand crate is a library crate, which contains code that is intended to be used in other programs and can’t be executed on its own.

### Semantic versioning
Given a version number MAJOR.MINOR.PATCH, increment the:

MAJOR version when you make incompatible API changes
MINOR version when you add functionality in a backwards compatible manner
PATCH version when you make backwards compatible bug fixes
Additional labels for pre-release and build metadata are available as extensions to the MAJOR.MINOR.PATCH format.

The specifier 0.8.5 is actually shorthand for ^0.8.5, which means any version that is at least 0.8.5 but below 0.9.0.

Cargo considers these versions to have public APIs compatible with version 0.8.5, and this specification ensures you’ll get the latest patch release that will still compile with the code in this chapter. Any version 0.9.0 or greater is not guaranteed to have the same API as what the following examples use.

When we include an external dependency, Cargo fetches the latest versions of everything that dependency needs from the *registry*, which is a copy of data from Crates.io. Crates.io is where people in the Rust ecosystem post their open source Rust projects for others to use.

although we only listed rand as a dependency, Cargo also grabbed other crates that rand depends on to work. After downloading the crates, Rust compiles them and then compiles the project with the dependencies available.

If you immediately run cargo build again without making any changes, you won’t get any output aside from the Finished line. Cargo knows it has already downloaded and compiled the dependencies, and you haven’t changed anything about them in your Cargo.toml file. Cargo also knows that you haven’t changed anything about your code, so it doesn’t recompile that either. With nothing to do, it simply exits.

Cargo has a mechanism that ensures you can rebuild the same artifact every time you or anyone else builds your code: Cargo will use only the versions of the dependencies you specified until you indicate otherwise.

When you build a project for the first time, Cargo figures out all the versions of the dependencies that fit the criteria and then writes them to the Cargo.lock file. When you build your project in the future, Cargo will see that the Cargo.lock file exists and will use the versions specified there rather than doing all the work of figuring out versions again. This lets you have a reproducible build automatically. In other words, your project will remain at 0.8.5 until you explicitly upgrade, thanks to the Cargo.lock file. 

`cargo update`

First we add the line use rand::Rng;. The Rng trait defines methods that random number generators implement, and this trait must be in scope for us to use those methods. Chapter 10 will cover traits in detail.

`cargo doc --open`

A match expression is made up of arms. An arm consists of a pattern to match against, and the code that should be run if the value given to match fits that arm’s pattern. Rust takes the value given to match and looks through each arm’s pattern in turn. 

The secret_number, on the other hand, is a number type. A few of Rust’s number types can have a value between 1 and 100: i32, a 32-bit number; u32, an unsigned 32-bit number; i64, a 64-bit number; as well as others. Unless otherwise specified, Rust defaults to an i32, which is the type of secret_number unless you add type information elsewhere that would cause Rust to infer a different numerical type.

let guess: u32 = guess.trim().parse().expect("Please type a number!");

Shadowing


## 3. Common Programming Concepts
### 3.1 Variables and Mutability
Constants can be declared in any scope, including the global scope, which makes them useful for values that many parts of code need to know about.

The last difference is that constants may be set only to a constant expression, not the result of a value that could only be computed at runtime.

#![allow(unused)]
fn main() {
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
}

Rust’s naming convention for constants is to use all uppercase with underscores between words.

Constants are valid for the entire time a program runs, within the scope in which they were declared. This property makes constants useful for values in your application domain that multiple parts of the program might need to know about, such as the maximum number of points any player of a game is allowed to earn, or the speed of light.

Hard coding (also hard-coding or hardcoding) is the software development practice of embedding data directly into the source code of a program or other executable object, as opposed to obtaining the data from external sources or generating it at runtime.

Shadowing is different from marking a variable as mut because we’ll get a compile-time error if we accidentally try to reassign to this variable without using the let keyword. By using let, we can perform a few transformations on a value but have the variable be immutable after those transformations have been completed.

    let mut spaces = "   ";
    spaces = spaces.len();
The error says we’re not allowed to mutate a variable’s type

### 3.2 Data Types
#### scalar type
let guess: u32 = "42".parse().expect("Not a number!");
If we don’t add the : u32 type annotation shown in the preceding code, Rust will display the following error, which means the compiler needs more information from us to know which type we want to use:
A scalar type represents a single value. Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters.

Length	Signed	Unsigned
8-bit	i8	u8
16-bit	i16	u16
32-bit	i32	u32
64-bit	i64	u64
128-bit	i128	u128
arch	isize	usize

Additionally, the isize and usize types depend on the architecture of the computer your program is running on, which is denoted in the table as “arch”: 64 bits if you’re on a 64-bit architecture and 32 bits if you’re on a 32-bit architecture.

Note that number literals that can be multiple numeric types allow a type suffix, such as 57u8, to designate the type. Number literals can also use _ as a visual separator to make the number easier to read, such as 1_000, which will have the same value as if you had specified 1000.

Number literals	Example
Decimal	98_222
Hex	0xff
Octal	0o77
Binary	0b1111_0000
Byte (u8 only)	b'A'

Wrap in all modes with the wrapping_* methods, such as wrapping_add.
Return the None value if there is overflow with the checked_* methods.
Return the value and a boolean indicating whether there was overflow with the overflowing_* methods.
Saturate at the value’s minimum or maximum values with the saturating_* methods.

Rust also has two primitive types for floating-point numbers, which are numbers with decimal points. Rust’s floating-point types are f32 and f64, which are 32 bits and 64 bits in size, respectively. The default type is f64 because on modern CPUs, it’s roughly the same speed as f32 but is capable of more precision. 

#### compound type

 Rust has two primitive compound types: tuples and arrays.

The Tuple Type
A tuple is a general way of grouping together a number of values with a variety of types into one compound type. Tuples have a fixed length: once declared, they cannot grow or shrink in size.

### 3.3 functions
Rust code uses snake case as the conventional style for function and variable names