# weChat mini app backend server

* 技术选型
1. Rust
2. Docker


* 框架
1. Actix-web

* workflows
1. 本地开发完成后推送到 master 分支
2. Github Actions 监听 master 分支的 push 和 pull_request hooks，然后开启 Docker build 并在结束后把镜像推送到个人 Docker Hub
3. ssh 登陆云服务器拉取镜像并跑起来【还有工作没完善】

* 本地热加载启动
> LOG_LEVEL=DEBUG cargo watch -x run
