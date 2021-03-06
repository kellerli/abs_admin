# abs_admin  语言后台管理模板项目。 此版本接口火热进行中，前端模块开发进行中
*  rust  common admin server
*  基于Rust语言，急速开发，高性能，无内存泄漏，协程竞争，坚如磐石
*  采用DDD领域驱动设计，基于后台开发最常见领域驱动设计
*  采用ActixWeb，常年屠榜web框架压测网站的框架，Tokio,常用的serde json
*  采用Mysql,Redis，最通用的中间件和数据库
*  采用VueJs，最简单通用低成本的前端解决方案
*  基于[Rbatis](https://github.com/rbatis/rbatis)，和Mybatis-Plus一样的好用，简洁，易扩展的ORM框架
*  提供现成的权限-角色-资源 关系映射，权限支持父子级关系，角色支持父子级关系。支持权限，角色 层级 勾选，多选，单选
*  提供现成的Jwt Token后台用户登录模式
*  提供现成的菜单展示，可随着权限赋予和移除 展示隐藏菜单、按钮

# 此项目存在的意义
* 快速开发迭代无忧，后台系统几乎是每个应用必须的。rust的优势即 编写完毕，无bug，无内存溢出，泄漏
* 复杂高并发无忧，例如某公司开发某ERP项目，如果是用Java之类的语言，水平参差不齐的公司越到后期代码量越大，复杂业务也导致内存暴涨，业务越复杂，性能越差。而使用rust起码保证性能不掉，内存没问题
* 易上手，为初次使用Rbatis和actix_web的用户提供学习案例
* 符合传统Java后台设计模式，习惯（Mybatis+Spring+SrpingMVC,前者支持，后2者不需要）。同时还保持安全+高性能

# 快速安装教程
* 1.docker（需安装docker）命令快速启动redis和mysql(用户名root密码123456)。生产docker可以建议部署http服务，原则上生产环境不建议用docker部署数据库
```cmd
docker run -it -d --name redis -p 6379:6379 redis
docker run -d -p 3306:3306 -e MYSQL_ROOT_PASSWORD=123456 --name mysql -e TZ=Asia/Shanghai mysql:5.7
```
* 2.使用MysqlWorkBench或Navcat等工具 导入database.sql脚本到Mysql数据库（mysql用户名密码root  123456）（redis无密码）中

* 3.安装打开PostMan ，导入postman.json到postman中即可使用写好的请求
```cmd
打开postman,导入 postman.json
```
* 4.使用Clion克隆导入abs_admin项目，命令行执行或者点开main.rs点击按钮运行
```cmd
cargo run
```


# dep crate(依赖库)
* rbatis
* actix-web
* tokio
* serde
* redis-rs

# database（数据库）
* mysql 5.7
* redis 
# web view manager(web后台页面js框架)
* vuejs(基于Vuejs)
* element ui(饿了么UI)

# module(模块)
* JWT token Auth(基于JWT token的权限鉴权)
* Role,User,Reource（角色，用户，权限）


