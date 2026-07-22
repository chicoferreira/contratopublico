#!/bin/sh
/entrypoint.sh &

rotate() {
  echo "Rotating exit IP..."
  warp-cli disconnect
  warp-cli registration delete
  warp-cli registration new
  warp-cli connect
}

elapsed=0
while true; do
  sleep 60
  elapsed=$((elapsed + 60))
  code=$(curl -s -o /dev/null -w '%{http_code}' --max-time 30 --socks5-hostname localhost:1080 https://www.base.gov.pt/Base4/pt/resultados/)
  if [ "$code" = "999" ] || [ "$elapsed" -ge 3600 ]; then
    rotate
    elapsed=0
  fi
done
