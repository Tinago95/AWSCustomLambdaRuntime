## Why Rust

Whether Rust is the C++ killer we've all been waiting for remains to be seen. With companies like Tesla and Microsoft still heavily reliant on C++ it would appear there is a strong case for C++'s continued existence. Nonetheless, Rust's growing popularity among developers is definitely something to take notice of. [1] In 2019 AWS officially became a sponsor of the Rust project before partnering with Microsoft, Google, Mozilla and Huawei in 2020 to create the Rust Foundation, an organistion whose sole purpose is to support Rust and fuel its adoption as a general purpose language. 

The rapid uptake of Rust by the three largest cloud tech companies has seen the Rust community grow considerably since Rust was first released in 2010. In saying so, it is becoming increasingly obvious that Rust skills will incrementally be in high demand well into the future. 

As a programming language Rust has all the hallmarks of a mature programming language that has the capacity to handle projects of any level of complexity. My personal favourite Rust Feature is the provision of an inbuilt packgage manager called `cargo`, (something I wish C++ came with). Rust also has other notable features, I have provided a list of some of them below: 

- No run time garbage collection (grants control of memory management)
- Can target embedded systems
- Rich type system and ownership model that helps you weed out memory related bugs thereby ensuring memory and thread safety.
- Tiny footprint
- A package manager called Cargo (Mentioned it twice because it is that good)

### Set up toolchain
#### installing rust 
``` https://www.rust-lang.org/tools/install ```

#### Install zigbuild using cargo
``` cargo install cargo-zigbuild ```

### Create Rust project  using the cargo
- Create new Rust Project
``` cargo new hello-world ```

-  Open the project folder and install dependencies 

``` 
        cargo add lambda_runtime tokio serde_json serde 
```

-  Replace  src/main.rs with the following code 


```rust 
use lambda_runtime::{Context, Error, service_fn, run, LambdaEvent};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[tokio::main]
async fn main () ->Result<(), Error>{
    let handler = service_fn(handler);
   run(handler).await?;
    Ok(())
}

#[derive(Deserialize)]
struct Event {
    first_name: String,
    last_name: String

}

#[derive(Serialize)]
struct Output {
    message: String,
    request_id:String
}

async fn handler (event: LambdaEvent<Event>) -> Result<Output, Error> {
    let message:String = format!("Hi {} {} , welcome to rust in the cloud!", event.payload.first_name, event.payload.last_name);
    Ok(Output { message, request_id: event.context.request_id })
}

```


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


### References
[1]  Sustainability with Rust, https://aws.amazon.com/blogs/opensource/sustainability-with-rust/#:~:text=We%20use%20Rust%20to%20deliver,%2C%20Amazon%20CloudFront%2C%20and%20more.
