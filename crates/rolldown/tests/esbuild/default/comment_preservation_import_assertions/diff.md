# Diff
## /out/entry.js
### esbuild
```js
// entry.jsx
import "foo" assert { type: "json" };
import "foo" assert { type: "json" };
import "foo" assert {
  /* before */
  type: "json"
};
import "foo" assert {
  type:
    /* before */
    "json"
};
import "foo" assert {
  type: "json"
  /* before */
};
```
### rolldown
```js
import "foo";


```
### diff
```diff
===================================================================
--- esbuild	/out/entry.js
+++ rolldown	entry.js
@@ -1,16 +1,2 @@
-// entry.jsx
-import "foo" assert { type: "json" };
-import "foo" assert { type: "json" };
-import "foo" assert {
-  /* before */
-  type: "json"
-};
-import "foo" assert {
-  type:
-    /* before */
-    "json"
-};
-import "foo" assert {
-  type: "json"
-  /* before */
-};
\ No newline at end of file
+import "foo";
+

```