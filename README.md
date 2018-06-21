# :warning: IMPORTANT :warning:

This is a fork of the **docker-compose-wait** repository.

This fork satisfies two issues:

1. We rely on the shell based implementation, tagged as 1.0.0 and we want to make sure that this is available
2. We regard the dynamic inclusion and execution of an external component a potential security risk and therefor we want to use our own copy

This mean that you can use the component as specified in the original documentation (see below), you do however have to point to our own fork.

```
ADD https://raw.githubusercontent.com/ufoscout/docker-compose-wait/1.0.0/wait.sh /wait.sh
```

Should be written as:

```
ADD https://raw.githubusercontent.com/DK-Hostmaster/docker-compose-wait/1.0.0/wait.sh /wait.sh
```

If you have any questions or comments to this practice please contact: @jonasbn

Thank you

_The documentation below is from the parent repository of this fork_

---------------------------------------

# docker-compose-wait
A small command line utility to wait for other docker images to be started while using docker-compose.
It permits to wait for a fixed amount of seconds and/or to wait until a TCP port is open on a target image.

# Usage
This utility should be used in docker build process and launched before your application starts.

For example, your application "MySuperApp" uses MongoDB, Postgres and MySql (wow!) and you want to be sure that when it starts all other systems are available, then simply customize your dockerfile this way:

```
FROM alpine

## Add your application to the docker image
ADD MySuperApp.sh /MySuperApp.sh

## Add the wait script to the image
ADD https://github.com/ufoscout/docker-compose-wait/releases/download/2.0.0/wait /wait
RUN chmod +x /wait

## Launch the wait tool and then your application
CMD /wait && /MySuperApp.sh
```

Done! the image is ready.

Now let's modify the docker-compose.yml file:

```
version: "3"

services:

  mongo:
    image: mongo:3.4
    hostname: mongo
    ports:
      - "27017:27017"

  postgres:
    image: "postgres:9.4"
    hostname: postgres
    ports:
      - "5432:5432"

  mysql:
    image: "mysql:5.7"
    hostname: mysql
    ports:
      - "3306:3306"

  mySuperApp:
    image: "mySuperApp:latest"
    hostname: mySuperApp
    environment:
      WAIT_HOSTS: postgres:5432, mysql:3306, mongo:27017
```

When docker-compose is started (or docker stack), your application will be started only when all the pairs host:port in the WAIT_HOSTS variable are available.
The WAIT_HOSTS environment variable is not mandatory, if not declared, the script executes without waiting.

# Additional configuration options
The behaviour of the wait utility can be configured with the following environment variables:
- *WAIT_HOSTS*: comma separated list of pairs host:port for which you want to wait.
- *WAIT_HOSTS_TIMEOUT*: max number of seconds to wait for the hosts to be available before failure. The default is 30 seconds.
- *WAIT_BEFORE_HOSTS*: number of seconds to wait (sleep) before start checking for the hosts availability
- *WAIT_AFTER_HOSTS*: number of seconds to wait (sleep) once all the hosts are available

# Notes
This utility was explicitly written to be used with docker-compose; however, it can be used with whatever tool as it has no dependencies on docker.

From version 2.0.0 this utility was rewritten from scratch in [rust](https://www.rust-lang.org), one of the consequences is that it does not require netcat or any other tool to be installed on the OS as in version 1.0.0.
