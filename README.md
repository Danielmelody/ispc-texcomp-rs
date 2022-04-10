# ISPC Texture Compression Rust bindings


Yet an another Rust binding for ispc texture compression

Supported texture formats:

* BC7
* BC6H
* BC3
* BC4
* ETC1

### Integration

To use this crate, one could simply add this to it's Cargo.toml

```toml
[dependencies]
ispc-texcomp = "0.1"
```

But this would only work on platforms that comes with our [prebuilt textcomp kernels](https://github.com/Danielmelody/ispc-texcomp-rs/tree/master/src/ispc), for platform outside those list, one must has `ispc` installed in PATH, and the **ispc** feature must be toggled.

```toml
[dependencies]
ispc-texcomp = {version="0.1", features=["ispc"]}
```

Thus `ispc` would been called on build time, compiling texcomp kernels into rs bindings and platform-specific static libraries.

Acknowledgement: this repo was imported from https://github.com/gwihlidal/intel-tex-rs, which seems to be dead. 
