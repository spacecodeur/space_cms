#!/bin/bash

## build containers

./cli.sh commands/services/web/container-management/build.sh &
./cli.sh commands/services/db/container-management/build.sh &
wait

## run some scripts like migrations, wasm building, ...

# ./cli.sh commands/services/web/in-container/migrate/up-all.sh

## finish !

echo
echo "All set!"
echo "Next step : ./cli.sh commands/services/web/in-container/start-service.sh"
echo "Happy coding! :D"