Blocks and files are the same thing
```c
let a = {
	export let foo = 5;
	let bar = 5;
}

a.foo = 0 // works
a.bar = 3 // errors because `bar` is not exported
```
