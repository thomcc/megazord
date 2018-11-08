# Rust-lang 50007 repro case

This repo has three crates in it, one root crate and two dependent crates. All three have some `#[no_mangle] pub extern "C" fn`s in them, and the root would like to contain all of these when compiled as a cdylib.

Unfortunately, some targets (not all) seem to break this. This repo demonstrates the issue (and also a somewhat tedious workaround, which is enabled when building with `--features='export_fns'`).

One target that reliably produces the issue is i686-linux-android, so you will probably have to set it up.

### Setting up i686-linux-android:

1. Download the NDK from https://developer.android.com/ndk/downloads, it doesn't matter what version, and unzip it.
2. Run `/path/to/downloaded/ndk/build/tools/make_standalone_toolchain.py --arch=x86 --api=28 --install-dir="/tmp/ndk-x86-28"`. The api version doesn't matter, nor does where you install it, although you'll need to refer to where you install it later.
3. Run `rustup target add i686-linux-android`.

### Reproducing the issue

Once that's done, it's just a matter of running the following (assuming you installed the ndk toolchain to /tmp/ndk-x86-28 as described above):

```
RUSTFLAGS="-C linker=/tmp/ndk-x86-28/bin/i686-linux-android-clang" cargo build --target i686-linux-android
```

This will produce `target/i686-linux-android/debug/libmegazord.so`. If you inspect it with `nm -g` (if you don't have it installed, `/tmp/ndk-x86-28/bin/i686-linux-android-nm` is available as part of the NDK you installed), you can see that `foo_fn` and `bar_fn` are not present.

This can be worked around, and if you build with `--features='export_fns'`, it will enable our fairly tedious and somewhat hacky workaround, and you can see that the symbols in question are present.
