# Demonstration of hyper issue [#3269](https://github.com/hyperium/hyper/issues/3269)

Run:
```shell
./run.sh
```

Example output:
```
Server running as pid 190511
Listening on http://127.0.0.1:3000 routes /stream and /regular

Hyper<1.0
Using regular route
took: 2.5972577279999998s
Using stream route
took: 8.238051985s

Hyper>=1.0
Using regular route
took: 2.281762271s
Using stream route
took: 2.177986499s
```
