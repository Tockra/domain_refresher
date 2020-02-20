# Basic Idea
This tool is for people who want to host their own server at home. The most ISPs don't give you a static IPv4 address. 
If your router restarts/reconnect you can get a new IPv4 internet address. This is bad when you have a A-Record which let your domain example.com
point at your ip address at home. After you get a new IP nobody can reach your server with your domain.

You can run this tool on your server to check your ip address regulary. If your domain have a A-Record to a IP address which isn't the 
current address of your server, this tool will set your current A-Record to your current ip address.

This tools just works with domains managed by https://onyxhosting.de .

# Build
First of all you need to install Rust and cargo (https://www.rust-lang.org/tools/install). 

Now enter domain_refresher/ in your terminal and run 
```bash
cargo build --release
```
You'll find the binary in target/release/

# Docker
You can use the Dockerfile, too. If you have installed docker, you don't need to install rust. You only need to run: 
```bash
docker build --tag <your_image_name> .
```
Now you need to create a directory where you want to safe the configuration. Move the docker-compose.yml file there and edit it. 
You need to change the name of the image to your chosen name. Create a config.json with following syntax and start the application with
```bash
docker-compose up -d 
```

# Config
In the configuration file you need to add your login information for onyxhost, your domain name and the interval in which the tool
shall check the current ip address. Please don't add sub-domains and don't add http:// or https:// !
The following is a example for the config.json :
```json
{
  "user": "your_user_name",
  "password": "password",
  "domain": "example.com, second.com",
  "refresh_time": "10"
}
```
This config file would update the domain example.com and second.com every 10 minutes. If the login information is wrong or you don't own the given domains this tool will crash.
