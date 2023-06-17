# CosmWasm Smart Contract Workspace

This workspace contains multiple smart contracts written in Rust and CosmWasm.

## Table of Contents

- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
- [Description](#description)
  - [first-contract](#first-contract)
  - [common](#common)
- [Usage](#usage)
  - [Test](#test)
  - [Lint](#lint)
- [Development](#development)
- [Contributing](#contributing)
- [License](#license)

## Getting Started

These instructions will help you get a copy of the smart contract up and running on your local machine for development and testing purposes.

### Prerequisites

- [CosmWasm](https://github.com/CosmWasm/cosmwasm)
- Rust: [Installation Guide](https://www.rust-lang.org/tools/install)
- Command runner: [just](https://github.com/casey/just)

### Installation

1. Clone the repository:

    ```shell
    git clone <repository_url>
    ```

2. Change into the project directory:

    ```shell
    cd <project_directory>
    ```

3. Build the smart contract:

    ```shell
    just optimize
    ```

## Description

### first-contract

Provide a brief description or overview of the first smart contract in the workspace.

### common

Provide a brief description or overview of the common package in the workspace.

## Usage

Describe how to deploy and interact with the smart contract. Provide examples and explanations of the available functionality.

### Test

```shell
just test
```

### Lint

```shell
just clippy && just fmt 
```

### JSON Schema

```shell
just schema
```

## Development

Explain any additional setup or configuration for development purposes. Include information on the project structure and any relevant details.

## Contributing

Explain how other developers can contribute to the smart contract. Include guidelines for pull requests, code style, and development practices.

```shell
just default-flow
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.
