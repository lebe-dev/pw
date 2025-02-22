# FAQ

**1. ¿Dónde se almacenan los secretos?**

Cifrados en la memoria de la aplicación (RAM).

**2. ¿Cómo se protegen los secretos?**

Tus datos se cifran en el lado del cliente (navegador) con AES 256 y una clave de 32 bytes. Luego, recibes una URL especial para acceder al secreto.
El servidor no conoce el mensaje original, solo almacena los datos cifrados (en RAM).

Cuando abres una URL de secreto, el cliente (navegador) carga los datos cifrados desde el servidor. Luego, el lado del cliente descifra los datos utilizando una clave fuerte codificada en la URL.

Las URLs de los secretos tienen una vida útil limitada (una hora, dos horas o un máximo de un día).

**3. ¿Qué algoritmo se utiliza para el cifrado?**

AES 256, con una clave de 32 bytes.

**4. ¿Es posible enviar enlaces de un solo uso a través de mensajeros?**

Sí, el enlace no expirará.

**5. ¿Cuántos recursos necesita la aplicación?**

Los contenedores de Docker consumen aproximadamente 12 MB de RAM.

```shell
$ docker stats --no-stream

CONTAINER ID   NAME              CPU %     MEM USAGE / LIMIT    MEM %     NET I/O           BLOCK I/O      PIDS
94d9d31ddf83   pw-cache          1.27%     7.977MiB / 1.69GiB   0.46%     16.2kB / 0B       0B / 0B        6
0d3c9c52165a   pw                0.00%     4.082MiB / 1.69GiB   0.24%     63.2kB / 224kB    0B / 0B        2
```

El tamaño de la imagen es de aproximadamente 30 MB:

```shell
$ docker images

REPOSITORY                                                TAG                IMAGE ID       CREATED          SIZE
tinyops/pw                                                1.6.0              ee1c473b8920   30 minutes ago   29.7MB
```

**6. Quiero crear mi propia imagen. ¿Qué debo hacer?**

Visita el [tutorial](../BUILD.md).
