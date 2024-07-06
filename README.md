# Api by TMDB
<a href="https://www.themoviedb.org/about/logos-attribution">
   <img style="float: left; width: 25%;" src="https://github.com/zhangyemengren/movie_database/blob/main/brand.svg?row=true"/>
</a>

## db
```bash
docker run -d \
--name movie_db \
-e MYSQL_ROOT_PASSWORD=root \
-v usr/database:/var/lib/mysql \
-p 3306:3306 \
mysql:latest
```
