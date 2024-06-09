# weChat mini app backend server

* 技术选型
1. Rust
2. Docker


* 框架
1. Actix-web

* workflows
1. 本地开发完成后推送到 master 分支
2. Github Actions 监听 master 分支的 push 和 pull_request hooks，然后开启 Docker build 并在结束后把镜像推送到个人 Docker Hub【github 登陆】
3. ssh 登陆云服务器拉取镜像并跑起来【还有工作没完善】

* 本地热加载启动
> LOG_LEVEL=DEBUG cargo watch -x run


* 云服务器部署操作

1. 更新镜像
```javascript
docker images // 显示所有镜像
docker rmi imageID // 删除镜像

docker pull tobeurself111/actix-for-min
```
2. 暂停服务容器
```javascript
docker ps // 显示所有运行的容器
docker ps -a // 显示所有容器
docker rm containerID // 删除容器

docker stop containerID
```
3. 用最新镜像起服务
```javascript
docker run -d -v /tmp/my-data/mini-images:/tmp/my-data/mini-images -p 3000:3000 --name actix-test --net my_testnet imageID
```
4. 测试服务状态
```javascript
curl "localhost:3000/rust/shop-list?page=1&size=10"
```
