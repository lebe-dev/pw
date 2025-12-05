# FAQ

**1. Où sont stockés les secrets?**

Chiffrés dans la mémoire de l'application (RAM).

**2. Comment les secrets sont-ils protégés?**

Vos données sont chiffrées côté client (navigateur) avec AES 256, clé de 32 octets. Ensuite, vous recevez une URL spéciale vers le secret.
Le serveur ne connaît rien du message original, il stocke uniquement les données chiffrées (en RAM).

Lorsque vous ouvrez une URL de secret, le client (navigateur) charge les données chiffrées depuis le serveur. Ensuite, le client déchiffre
les données avec la clé forte encodée dans l'URL.

Les URL de secrets ont une durée de vie limitée (une heure, deux heures ou un jour maximum).

**3. Quel algorithme est utilisé pour le chiffrement?**

AES 256, longueur de clé 32.

**4. Est-il possible d'envoyer des liens à usage unique via des messageries?**

Oui, le lien n'expirera pas.

**5. Combien de ressources sont nécessaires pour l'application?**

Les conteneurs Docker consomment ~12 Mo de RAM.

```shell
$ docker stats --no-stream

CONTAINER ID   NAME              CPU %     MEM USAGE / LIMIT    MEM %     NET I/O           BLOCK I/O      PIDS
94d9d31ddf83   pw-cache          1.27%     7.977MiB / 1.69GiB   0.46%     16.2kB / 0B       0B / 0B        6
0d3c9c52165a   pw                0.00%     4.082MiB / 1.69GiB   0.24%     63.2kB / 224kB    0B / 0B        2
```

La taille de l'image est d'environ 30 Mo:

```shell
$ docker images

REPOSITORY                                                TAG                IMAGE ID       CREATED          SIZE
tinyops/pw                                                1.6.0              ee1c473b8920   30 minutes ago   29.7MB
```

**5. Je voudrais construire ma propre image. Que dois-je faire?**

Consultez le [tutoriel](../BUILD.md).
