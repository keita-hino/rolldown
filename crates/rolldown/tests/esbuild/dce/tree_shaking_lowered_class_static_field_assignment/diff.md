# Diff
## /out/entry.js
### esbuild
```js
// entry.ts
var KeepMe1 = class {
};
KeepMe1.x = "x";
KeepMe1.y = "y";
KeepMe1.z = "z";
var KeepMe2 = class {
};
KeepMe2.x = "x";
KeepMe2.y = sideEffects();
KeepMe2.z = "z";
var KeepMe3 = class {
};
KeepMe3.x = "x";
KeepMe3.y = "y";
KeepMe3.z = "z";
new KeepMe3();
```
### rolldown
```js

//#region entry.ts
class KeepMe2 {
	static x = "x";
	static y = sideEffects();
	static z = "z";
}
class KeepMe3 {
	static x = "x";
	static y = "y";
	static z = "z";
}
new KeepMe3();

//#endregion

```
### diff
```diff
===================================================================
--- esbuild	/out/entry.js
+++ rolldown	entry.js
@@ -1,13 +1,11 @@
-var KeepMe1 = class {};
-KeepMe1.x = "x";
-KeepMe1.y = "y";
-KeepMe1.z = "z";
-var KeepMe2 = class {};
-KeepMe2.x = "x";
-KeepMe2.y = sideEffects();
-KeepMe2.z = "z";
-var KeepMe3 = class {};
-KeepMe3.x = "x";
-KeepMe3.y = "y";
-KeepMe3.z = "z";
+class KeepMe2 {
+    static x = "x";
+    static y = sideEffects();
+    static z = "z";
+}
+class KeepMe3 {
+    static x = "x";
+    static y = "y";
+    static z = "z";
+}
 new KeepMe3();

```