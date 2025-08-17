# `@ylcc/lithtech-parser`

![https://github.com/jerry4718/js-lithtech-parser/actions](https://github.com/jerry4718/js-lithtech-parser/workflows/CI/badge.svg)

> Parse lithtech file (dtx/ltb/dat) with wasm/napi for Node.js/Deno/Browser.

## Install this test package

```
pnpm add @ylcc/lithtech-parser
```

## Usage

### Build

After `pnpm build` command, you can see `lithtech-parser.[darwin|win32|linux].node` file in project root. This is the native addon built from [lib.rs](./src/lib.rs).

### Test

With [ava](https://github.com/avajs/ava), run `pnpm test` to testing native addon. You can also switch to another testing framework if you want.

## Develop requirements

- Install the latest `Rust`
- Install `Node.js@16+` which fully supported `Node-API`
- Run `corepack enable`

## Test in local

- pnpm
- pnpm build
- pnpm test

And you will see:

```bash
$ ava --verbose

  ✔ sync function from native code
  ✔ sleep function from native code (201ms)
  ─

  2 tests passed
✨  Done in 1.12s.
```
