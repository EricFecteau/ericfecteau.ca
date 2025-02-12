#!/bin/bash -e

function get_name {
  cat ./mem_log/prog_type.txt
}

echo "      date     time  type   $(free | grep total | sed -E 's/^    (.*)/\1/g')" | xargs > ./mem_log/out.log
while true; do
    echo "$(date '+%Y-%m-%d %H:%M:%S.%3N')  $(get_name)  $(free | grep Mem: | sed 's/Mem://g')" | xargs >> ./mem_log/out.log
    sleep 0.01
done
