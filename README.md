# hsum
hsum is a simple cli tool that compute the hash digest of a file.

I have a feeling that a better cli tool than md5sum or sha256sum will be completed.

Currently, only stdin/stdout are supported for input/output.

In the future, we may support full file manipulation.

## Supported algorithms
- MD5
  - [ ] MD5 (Deprecated. However,it may be supported for a short version to maintain compatibility. Under consideration.)
- SHA1
  - [ ] SHA1 (Deprecated. However,it may be supported for a short version to maintain compatibility. Under consideration.)
- SHA2
  - [ ] SHA224 (Under consideration.) 
  - [x] SHA256
  - [x] SHA384
  - [x] SHA512
  - [ ] SHA512-224 (Under consideration.) 
  - [x] SHA512-256
- SHA3
  - [ ] SHA3-224 (Under consideration.) 
  - [ ] SHA3-256
  - [ ] SHA3-384
  - [ ] SHA3-512
  - [ ] SHAKE128 (Under consideration.)
  - [ ] SHAKE256 (Under consideration.)
- KangarooTwelve
  - [ ] KangarooTwelve (Under consideration.)
- RIPEMD
  - [ ] RIPEMD-128 (Under consideration.)
  - [ ] RIPEMD-160 (Under consideration.)
  - [ ] RIPEMD-256 (Under consideration.)
  - [ ] RIPEMD-320 (Under consideration.)
- BLAKE
  - [ ] BLAKE3 (For now, only: Only Regular-hash are supported. I won't say anything about KDF-mode, etc. at this date.)

Support for KDF, KDF-mode, Password hash, etc. is also being considered. but will likely not be implemented.

## ToDo
- [ ] Rewrite the program code neatly.
- [ ] Work on high speed reading.
- [ ] Rewrite README.md neatly.
- [ ] Clarify policies, etc.

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
