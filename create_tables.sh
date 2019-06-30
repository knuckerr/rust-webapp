#!/bin/bash
docker container exec -i postgres  psql common < init.sql
