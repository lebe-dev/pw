# 常见问题

**1. 密钥存储在哪里？**

加密存储在应用程序内存（RAM）中。

**2. 密钥如何受到保护？**

您的数据在客户端（浏览器）使用 AES 256、32 字节密钥进行加密。然后您会收到一个指向密钥的特殊 URL。
服务器端对原始消息一无所知，它只存储加密数据（在 RAM 中）。

当您打开密钥 URL 时，客户端（浏览器）从服务器端加载加密数据。然后客户端使用
编码在 URL 中的强密钥解密数据。

密钥 URL 具有有限的生命周期（最多一小时、两小时或一天）。

**3. 加密使用什么算法？**

AES 256，密钥长度 32。

**4. 可以通过即时通讯工具发送一次性链接吗？**

可以，链接不会过期。

**5. 应用程序需要多少资源？**

Docker 容器消耗约 12 MB 的 RAM。

```shell
$ docker stats --no-stream

CONTAINER ID   NAME              CPU %     MEM USAGE / LIMIT    MEM %     NET I/O           BLOCK I/O      PIDS
94d9d31ddf83   pw-cache          1.27%     7.977MiB / 1.69GiB   0.46%     16.2kB / 0B       0B / 0B        6
0d3c9c52165a   pw                0.00%     4.082MiB / 1.69GiB   0.24%     63.2kB / 224kB    0B / 0B        2
```

镜像大小约为 30 MB：

```shell
$ docker images

REPOSITORY                                                TAG                IMAGE ID       CREATED          SIZE
tinyops/pw                                                1.6.0              ee1c473b8920   30 minutes ago   29.7MB
```

**5. 我想构建自己的镜像。应该怎么做？**

请访问[教程](../BUILD.md)。
