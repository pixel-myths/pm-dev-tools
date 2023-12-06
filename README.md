# pm-dev-tools

## Introduction

Set of tools for ease of development. Currently adds screen based diagnostics and a basic console which can be triggered by `"`.

Current out-of-box commands supported:
 - `diagnostics fps`: toggle fps counter.


## Adding the Plugin 

```rust
use bevy::prelude::*;
use pm_dev_tools::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PMDevToolsPlugin)
        .run();
}
```

## Extending Commands With Game Specific Features

 Although we aim to provide set of commands for general use cases such as diagnostics. It is also obvious that you will need to add new commands specific to your game. `pm-dev-tools` re-exports necessary crates. It is also easy to do so:

```rust
/// Example command
#[derive(Parser, ConsoleCommand)]
#[command(name = "example")]
struct ExampleCommand {
    /// Some message
    msg: String,
}

fn example_command(mut log: ConsoleCommand<ExampleCommand>) {
    if let Some(Ok(ExampleCommand { msg })) = log.take() {
        // handle command
    }
}
```

and it can be registered to the app as follows:

```rust
use bevy::prelude::*;
use pm_dev_tools::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PMDevToolsPlugin)
        .add_console_command::<ExampleCommand, _>(example_command)
        .run();
}
```
 
