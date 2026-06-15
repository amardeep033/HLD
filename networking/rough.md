1. dns resolution
2. cdn(cloudflare) vs multi-region geo-distributed cloud servers
3. proxy(zscaler) vs cdn(is speacial proxy) vs vpn
4. api gateway vs load balancing
5. routing vs isp(proxy?)
6. proxy(app level) vs vpn(device level + encryption) vs firewall(can be at multiple layer)
7. vpn vs firewall
8. osi layer with devices and protocols
9. full flow of a web request (from typing the URL to receiving the response) -- one  with vpn and one without vpn

-> VPN:
    - we first connect to vpn 
    - subsequently, all our traffic[can be semi too] is routed through the vpn server
    - os sees connected with vpn -- so all traffic is routed through vpn [tunnel]
    - isp to vpn server[isp doesnt see the actual destination -- only sees vpn server as destination] -- vpn server to destination server
    - VPN creates a virtual network adapter -- so two adapters: physical wifi is connected to the internet, and virtual adapter is connected to the vpn server
    - vpn server acts as a proxy for our traffic, masking our IP address and encrypting our data
    - famous vpn providers: nordvpn, expressvpn, cyberghost, surfshark, privateinternetaccess
    - vpn use: privacy, security, bypassing geo-restrictions, accessing blocked content, secure remote access to corporate networks
    - It adds a wrapper around request and response, hence encrypting and changing the source and destination of the traffic, making it more secure and private, but also can introduce latency due to encryption and routing through the vpn server

-> Forward proxy:
    - not used by common users, but used by organizations to control and monitor employee internet usage
    - all the traffic from the employees goes through the forward proxy server, which can filter content, block certain websites, and log user activity
    - vs vpn : doesnt encrypt traffic by default, and only routes specific traffic through the proxy server, while vpn routes all traffic through the vpn server
    - vs vpn : forward proxy is app specific, while vpn is device-wide -- then how does all my traffic goes through corporate forward proxy? -- because they configure the proxy settings on the devices to route specific traffic through the forward proxy server
    - famous forward proxy software: squid, apache mod_proxy, nginx, tinyproxy
    - use: content filtering, access control, caching, and logging user activity in organizational settings
    - where does it lies -- in wifi? in gateway? -- it can be deployed in various locations within a network, such as on a dedicated server, within a gateway, or even on individual devices
    - laptop -> WIFI -> Firewall -> Forward Proxy -> Internet

-> Reverse proxy:
    - sits in front of web servers and forwards client requests to the appropriate backend server
    - used for load balancing, caching[CDN], and security purposes

-> Scenarios: 
    1. office wifi -> cannot connect to chess.com or company's dev app hosted on cloud 
    2. office wifi + vpn -> cannot connect to chess.com but can connect to company's dev app 
    3. home wifi -> can connect to chess.com but cant connect to company's dev app 
    4. home wifi + vpn -> can connect to chess.com and also connect to company's dev app
    -> If Wifi at home + office VPN: then how my packet flows : split tunnel or full tunnel vpn -- in split tunnel, only specific traffic goes through the vpn, while in full tunnel, all traffic goes through the vpn

-> Firewall:
    - can be hardware or software-based, also at any place in the network

-> gateway:
    - its just a layer between two networks 
    - for example : our office network and the internet
    - it comes between firewall and forward proxy in the office network

-> DNS resolution:
    - multiple layers of dns resolution: local dns cache, os level dns resolver, router dns resolver, isp dns resolver, public dns resolvers (google, cloudflare), authoritative dns servers for the domain
    - its same like api request flow -- goes through all layers
    - Wifi provides network config through DHCP which includes 
        | Thing       | Example                |
        | ----------- | ---------------------- |
        | Your IP     | 192.168.1.25           |
        | Subnet mask | 255.255.255.0          |
        | Gateway     | 192.168.1.1            |
        | DNS server  | 192.168.1.1 or 8.8.8.8 | <-- this is the dns server that your laptop will use to resolve domain names to IP addresses
    - DNS has its own protocol and port (UDP/TCP 53) and is used to resolve domain names to IP addresses
    - Common public DNS resolvers:
        | Thing       | Example                |
        | ----------- | ---------------------- |
        | Your IP     | 192.168.1.25           |
        | Subnet mask | 255.255.255.0          |
        | Gateway     | 192.168.1.1            |
        | DNS server  | 192.168.1.1 or 8.8.8.8 |
    - ISPs also run their own DNS infrastructure: Airtel DNS, Comcast DNS, Verizon DNS

-> ISP: 
    - Internet Service Provider, provides internet connectivity to homes and businesses
    - ISPs have their own infrastructure, including routers, switches, and DNS servers, to route traffic between their customers and the internet
    - Only routes till the vpn server if vpn is used, or else till the destination server if vpn is not used[but doesnt make use of own infrastructure for routing -- just routes to the destination server using the internet backbone]

-> CDN:
    - CDN is used for caching and delivering content closer to the users, improving performance and reducing latency
    - CDN vs having multiple geo-distributed servers: CDN is a network of servers distributed across various locations that cache and deliver content, while having multiple geo-distributed servers means hosting your content on multiple servers located in different regions without necessarily using a CDN for caching and delivery
    - fmaous CDN providers: Cloudflare, Akamai, Amazon CloudFront, Fastly
    - CDN can also act as a reverse proxy, sitting in front of the origin server and caching content to improve performance and reduce load on the origin server
    - technically a CDN provider can read data that passes through it if HTTPS/TLS terminates at the CDN and this is one of the biggest trust/security tradeoffs in modern web architecture -- decrypt for caching
    - 

    

-------------------------------------------------------------------------

Laptop IP: 192.168.1.10
Gateway: 192.168.1.1
Proxy: 10.20.30.40

Browser
 ↓
send packet to proxy (10.20.30.40)
 ↓
OS says:
"not local network"
 ↓
send packet to default gateway
 ↓
gateway routes toward proxy

-------------------------------------------------------------------------

Browser/App
 ↓
Laptop OS
 ↓
Wi-Fi NIC
 ↓
Office Access Point
 ↓
LAN Switch
 ↓
Gateway Router
 ↓
Firewall
 ↓
Forward Proxy / Secure Web Gateway
 ↓
Internet

-------------------------------------------------------------------------

| Component                          | Hardware?                    | Software?                  | Famous Examples                            | Physical Place                                |
| ---------------------------------- | ---------------------------- | -------------------------- | ------------------------------------------ | --------------------------------------------- |
| Browser/App                        | ❌                            | ✅                          | Google Chrome, Mozilla Firefox             | **Your laptop**                               |
| Laptop OS / Networking Stack       | ❌                            | ✅                          | Linux, Windows                             | **Your laptop**                               |
| Wi-Fi NIC                          | ✅                            | ✅ drivers/firmware         | Intel Wi-Fi card, Realtek NIC              | **Inside your laptop**                        |
| Office Access Point                | ✅                            | ✅ firmware                 | Cisco APs, Ubiquiti UniFi                  | **Office ceiling/walls**                      |
| LAN Switch                         | ✅                            | Sometimes managed software | Cisco switches, Juniper Networks           | **Office server/network room**                |
| Gateway Router                     | ✅                            | ✅ routing OS/software      | Cisco routers, MikroTik                    | **Office edge/network room**                  |
| Firewall                           | Often dedicated appliance    | ✅ filtering/security logic | Palo Alto Networks, Fortinet FortiGate     | **Company network edge / datacenter**         |
| VPN Client                         | ❌                            | ✅                          | Fortinet FortiClient, Cisco AnyConnect     | **Your laptop**                               |
| VPN Gateway / VPN Server           | Sometimes appliance          | ✅ VPN server software      | FortiGate VPN, OpenVPN server              | **Company datacenter / cloud**                |
| Forward Proxy / Secure Web Gateway | Sometimes appliance          | ✅                          | Zscaler, Squid                             | **Company cloud / security provider network** |
| Reverse Proxy                      | Usually software             | ✅                          | Nginx, Envoy                               | **Near backend servers/cloud infra**          |
| Company Dev App                    | Usually cloud VMs/containers | ✅                          | Internal Rust backend, Kubernetes services | **Company AWS/Azure/GCP VPC**                 |
| Internet Backbone                  | Massive hardware             | Massive software           | ISPs, undersea cables, BGP routers         | **Outside everywhere globally**               |
