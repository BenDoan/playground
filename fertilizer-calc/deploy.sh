#!/bin/bash

./build.sh && docker save fertilizer-calc | gzip | pv | ssh flainted docker load && ssh flainted "cd flainted_compose && docker-compose up -d"
