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

**4. I want to build my own image. What should I do?**

Visit [tutorial](../BUILD.md).