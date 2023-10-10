# HSum
A simple cli tool get hash digest from stdin.

## Supported algorithms
* SHA2
  * SHA256
  * SHA384
  * SHA512
  * SHA512-256

# Build
```bash
zsh ./build.sh release
```

**extract**
```bash
tar -zxvf *.tar.gz && rm *.tar.gz
```

# Usage
```bash
# Display
./hsum --hash sha512-256 -u true < README.md

# Redirect
./hsum --hash sha512-256 -u true < README.md > output.txt

# Pipe
./hsum --hash sha512-256 -u true < README.md | cat
```
