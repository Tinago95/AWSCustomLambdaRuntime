## Why Rust

Whether Rust will eventually kill C++ remains to be seen. With companies like Tesla and Microsoft still heavily reliant on C++ it would appear there is a strong case for C++'s continued existence. However, I do believe C++ will eventually lose its position as the pre-eminent language for high performance computing and low level programming to Rust or Carbon, a Google love child designed to have reverse compatibility with C++. There are several arguments that can be made in support of this statement but that is an entire blog of its own.

Here is a none exhaustive list of the most notable features that come with Rust.

- No run time garbage collection (grants control of memory management)
- Can target embedded systems
- Rich type system and ownership model that helps you weed out memory related bugs thereby ensuring memory and thread safety.
- Tiny footprint
- A package manager called Cargo

## Set up Rust
#### installing rust 
``` https://www.rust-lang.org/tools/install ```

#### install zigbuild
``` cargo install cargo-zigbuild ```

### Write Simple Rust Program


### Build binary executable using zigbuild
From the project root run ``` cargo zigbuild --target aarch64-unknown-linux-gnu --release ```
Running

#### Create Lambda function 
##### In AWS 
1. Provide function name
2. Under the Custom Runtime section of the Runtime selection drop-down menu, select `Provide your own bootsrap on Amazon Linux 2`
3. In the architecture section, select `arm64`
4. Create
5. Select upload from .zip and select the lambda.zip file we created earlier on our local machine. 


### Deploy Lambda with CDK

### Test








## cross compiling
- cross  https://github.com/cross-rs/cross#installation (easy)
- https://artofserverless.com/rust-lambdas-macos/ (hard)