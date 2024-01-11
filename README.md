# Demonstration of hyper issue [#3269](https://github.com/hyperium/hyper/issues/3269)

Run:
```shell
./run.sh
```

Example output:
```
Server running as pid 177499
Listening on http://127.0.0.1:3000 routes /stream and /regular

Reqwest (hyper<1)
Using regular route
took: 2.208865495s
Using stream route
took: 8.265027194s

Hyper (hyper>=1)
Using regular route
took: 2.175678127s
Using stream route
took: 2.246615493s
```
