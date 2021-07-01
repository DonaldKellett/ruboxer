# ruboxer

Rudimentary container tool for Linux

## About

`ruboxer` stands for both "**ru**dimentary **boxer**" and "**Ru**st **boxer**". It was conceived as a simple yet educational systems programming project to explore the following areas:

- Rust as a systems programming language, a type and memory-safe alternative to C
- System calls and features exported by the Linux kernel used in modern container runtimes such as Docker and Kubernetes

## Linux features employed by `ruboxer`

- chroot
- Process namespaces
- Memory cgroups

## Linux features yet to be employed

- Capabilities
- Network namespaces
- Seccomp profiles
- Mandatory access control (MAC)
- ...

## Credits

Inspired by Eric Chiang's [Containers from Scratch](https://ericchiang.github.io/post/containers-from-scratch/)

## License

[MIT](./LICENSE)
