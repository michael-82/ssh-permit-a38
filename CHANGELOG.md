# SSH Permit A38 - Changelog

## v0.2.0 - 2018-08-18

- Support for SSH config files [#5](https://github.com/ierror/ssh-permit-a38/issues/5)

    If a ssh-permit-a38 hostname or alias matches the ssh configs Host (or Hostname), User, Port and Host information are used for authorized_keys sync connection

- sync command switch -y, --yes-authorized-keys-prompt:  Automatic yes to authorized_keys location prompts
    

## v0.1.0 - 2018-04-01

- Support for SSH agent authentication [#4](https://github.com/ierror/ssh-permit-a38/issues/4) - Thank you [@kdar:](https://github.com/kdar:)

- Support for host aliases [#2](https://github.com/ierror/ssh-permit-a38/issues/2): 

    - Set an alias "um" for hostname "urlsmash.403.io" 
    ```
    ssh-permit-a38 host urlsmash.403.io alias um
    ```
    
    After this point you can use the alias or the hostname for all host related commands.

    - Remove an alias for hostname "urlsmash.403.io" 
    ```
    ssh-permit-a38 host urlsmash.403.io alias
    ```
    
- Vagrant files and Makefile targets to build linux releases

- Fixed Typos [#1](https://github.com/ierror/ssh-permit-a38/issues/1) [#3](https://github.com/ierror/ssh-permit-a38/issues/3) - Thank you [@0xflotus](https://github.com/0xflotus) and [@robwetz](https://github.com/robwetz)


## v0.0.1 - 2018-03-18

- initial release