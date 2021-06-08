## What is it

High-performance visualization of live market data feed supplied by Pyth Network and decentralized Solana blockchain.

### Goal

Assemble basic data-delivery portal by merging real-time market data subscription (Pyth + Solana) with static research (CoinGecko) and augmented with randomly generated super high-frequenecy feed to simulate a Depth of Book and Time and Sales windows.

### Technology

1. UI: Typescript + React + Hooks
2. Image Generation: WebAssembly on Rust
3. Data Rendering: canvas-based grids

## What is it not

1. Not a production-ready app for mass consumption
2. Not an endorsement for Canvas + WebAssembly + Rust as a preferred solution for fast data rendering
3. Not meant to save the world from hunger or climate change

https://user-images.githubusercontent.com/82620796/121120663-a1251e00-c7e3-11eb-8f6c-beeda3e9996c.mov

https://user-images.githubusercontent.com/82620796/121123527-00396180-c7e9-11eb-8d88-979aac984dee.mov

## How to initialize demo (mac version):

1. Install Rust:

   > curl https://sh.rustup.rs -sSf | sh

2. install WebAssembly:

   > curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

3. build wasm/rust module:

   > yarn build:wasm

4. install dependencies:

   > yarn

## Available Scripts

In the project directory, you can run:

### `yarn start`

Runs the app in the development mode.\
Open [http://localhost:3000](http://localhost:3000) to view it in the browser.

The page will reload if you make edits.\
You will also see any lint errors in the console.

### `yarn test`

Launches the test runner in the interactive watch mode.\
See the section about [running tests](https://facebook.github.io/create-react-app/docs/running-tests) for more information.

### `yarn build`

Builds the app for production to the `build` folder.\
It correctly bundles React in production mode and optimizes the build for the best performance.

The build is minified and the filenames include the hashes.\
Your app is ready to be deployed!

See the section about [deployment](https://facebook.github.io/create-react-app/docs/deployment) for more information.

## Learn More

You can learn more in the [Create React App documentation](https://facebook.github.io/create-react-app/docs/getting-started).

To learn React, check out the [React documentation](https://reactjs.org/).
