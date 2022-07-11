# Nginx Simple Config Creation Tool.
Basically a CLI tool to create proxy servers. Eventually implement a TUI.

# Unfinished...
I am slow at this. I am learning rust at the same time.

## Project Plan
I want a simple cli that can quickly create a proxy server for any 
sort of http app I spin up.

Rough checklist of functionality.
- [ ] Create simple config files that enable nginx to act as a proxy.
  - [x] Serialise text file. ( Broken root currently.)
  - [ ] Symbolic link creation from available to enabled.
  - [ ] Option to run certbot to get letsencrypt cert.
- [ ] Implement a TUI
  - [ ] Enable/Disable Create/Delete current configs. 


