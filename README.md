# HostRN
HostRN is a fun little side project where instead of having to manually start and stop services on your terminal, you can now do it all from a web interface.

## FAQ
### Why choose this?
HostRN doesn't use docker. Because... I don't know. Some of my projects that I have in my self-hosted server has some cool stuff I don't want to keep opening my terminal up. So, that's why I made this!

### Isn't there a better alternatives?
Probably. I don't really care that much because this project is mainly for me to learn Rust as the backend and learn SolidJS for the frontend.

### What makes this better?
It's not. Probably. We give you less features because, well, fuck off with a bunch of features. I just want a simple save, start, stop, delete, and list services.

## Installation
- Ensure you have both `cargo` and `pnpm` installed on your system. You can install cargo from [here](https://www.rust-lang.org/tools/install) and pnpm from [here](https://pnpm.io/installation).
- Clone the repository, and run `pnpm install` to install all the dependencies.
- Once done, run `pnpm run start` to run both servers at once! *The power of workspaces! And also, Rust and SolidJS.*
- Then, just visit `http://localhost:3000` to see the web interface. You can also view it from another device in the same network or zero-trust VPN!
    - PLEASE AND PLEASE create a `.env` file in [@frontend](frontend/) with `API_URL` set to your server's IP address.

## LICENSE
HostRN is licensed under the MIT License. See the [LICENSE](LICENSE) file for more information.
