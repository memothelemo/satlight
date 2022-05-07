# Salite

## About

An intimation of Luau programming language but has a bit
different syntax and somewhat different typing approach.

**This programming language is experimental and unstable especially the typechecking system. Use it at your own risk.**

## Prerequisities when compiling

Make sure you set your toolchain to `nightly`, because it requires the feature of [`ptr_const_cast`](./lang/checker/src/analyzer/expressions/library.rs)

Search it for yourself how to do that.

## Code

```
local function factorial(n: number) -> number
	if n == 0 then
		return 1
	else
		return 0
	end
end
factorial(nil) -- Oh no! type error
```

## Credits / References

https://github.com/Kampfkarren/full-moon
https://github.com/kdy1/swc/tree/e88e5e4c824926e5f3a5240986c33bb30f7508b5/typescript/checker
