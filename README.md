# Memo
target file size: 4.19GB.

**macOS MD5:**

8.23s user 0.64s system 99% cpu 8.896 total

**macOS HSum --hash md5:**

6.18s user 0.48s system 99% cpu 6.672 total

# hsum
hsum is a simple cli tool that compute the hash digest of a file.

I have a feeling that a better cli tool than md5sum or sha256sum will be completed.

Currently, only stdin/stdout are supported for input/output.

In the future, we may support full file manipulation.

## Supported algorithms  
- MD5
  - [x] MD5
- SHA1
  - [x] SHA1
- SHA2
  - [x] SHA256
  - [x] SHA384
  - [x] SHA512
  - [x] SHA512-256
- SHA3
  - [x] SHA3-256
  - [x] SHA3-384
  - [x] SHA3-512
  
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

echo hello | ./hsum --hash sha512-256 -u true
```