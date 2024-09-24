# FAQ

**1. Where are secrets stored?**

Encrypted in app memory (RAM).

**2. How are secrets protected??**

Your data is encrypted on client side (browser) with AES 256, 32-key. Then you receive special URL to secret. 
Server side knows nothing about original message it just stores encrypted data (in RAM).

When you open a secret URL, client (browser) loads encrypted data from server side. Then client side will decrypt
data with strong key encoded into URL.

Secret URLs have limited lifespan (one hour, two hours or one day max).

**3. What algorithm is used for encryption?**

AES 256, key length 32.

**4. How many resources are needed for the application?**

Docker containers consume ~12 MB of RAM.

```shell
$ docker stats --no-stream

CONTAINER ID   NAME              CPU %     MEM USAGE / LIMIT    MEM %     NET I/O           BLOCK I/O      PIDS
94d9d31ddf83   pw-cache          1.27%     7.977MiB / 1.69GiB   0.46%     16.2kB / 0B       0B / 0B        6
0d3c9c52165a   pw                0.00%     4.082MiB / 1.69GiB   0.24%     63.2kB / 224kB    0B / 0B        2
```

Image size is ~30 MB:

```shell
$ docker images

REPOSITORY                                                TAG                IMAGE ID       CREATED          SIZE
tinyops/pw                                                1.6.0              ee1c473b8920   30 minutes ago   29.7MB
```

**5. I'd like to build my own image. What should I do?**

Visit [tutorial](../BUILD.md).