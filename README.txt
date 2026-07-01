QRcode.show(1)                        General Commands Manual                       QRcode.show(1)

   NAME
       QRcode.show - Generate QR code easily for free - QR Code Generation as a Service

   USAGE
              curl qrcode.show/INPUT

              curl qrcode.show -d INPUT

              curl qrcode.show -d @/PATH/TO/INPUT

              echo INPUT | curl qrcode.show -d @-

   USAGE EXAMPLES
              curl qrcode.show/https://example.com

              curl qrcode.show -d https://example.com

              curl qrcode.show -d @/path/to/input.txt

              echo https://example.com | curl qrcode.show -d @-

   PARAMETERS
       Header             Descrip‐     Format / Options
                          tion
       ───────────────────────────────────────────────────────────────────────────────────
       accept             output       application/octet-stream,  text/plain,  text/html,
                          type         image/svg+xml, image/png, image/jpeg
       x-qr-width         default
                          width
       x-qr-height        default
                          height
       x-qr-min-width     minimum
                          width
       x-qr-min-height    minimum
                          height
       x-qr-max-width     maximum
                          width
       x-qr-max-height    maximum
                          height
       x-qr-dark-color    dark color   rrggbb
                          (hex)
       x-qr-light-color   light        rrggbb
                          color
                          (hex)
       x-qr-ver‐          QR version   normal, micro
       sion-type          type
       x-qr-ver‐          QR version   1 -> 40 for normal, 1 -> 4 for micro.
       sion-number        number
       x-qr-ec-level      error        L, M, Q, H
                          checking
                          level
       x-qr-quiet-zone    add  quiet   true/ false
                          zone

   PARAMETER EXAMPLES
              curl qrcode.show/INPUT -H accept:image/svg+xml

   SHELL FUNCTIONS
       Shell  functions  that  can  be  added to .bashrc or .bash_profle for quickly generating QR
       codes from the command line.  The command takes the argument as input or reads  from  stdin
       if none was supplied and outputs the QR code to stdout.

              qrcode () {
                  local input="$*"
                  [ -z "$input" ] && local input="@/dev/stdin"
                  curl -d "$input" https://qrcode.show
              }
              qrsvg () {
                  local input="$*"
                  [ -z "$input" ] && local input="@/dev/stdin"
                  curl -d "${input}" https://qrcode.show -H accept:image/svg+xml
              }
              qrserve () {
                  local port=${1:-8080}
                  local dir=${2:-.}
                  local ip="$(ifconfig | grep -Eo '[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}' | fzf --prompt IP:)" \
                  && echo http://$ip:$port | qrcode \
                  && python -m http.server $port -b $ip -d $dir
              }

   FEATURES
       • No data collection or retention.

       • Fast and simple API that works on both web and terminal.

       • Supports GET and POST requests.

       • Supports accept header to control the output format.

   CREDITS
       • Main Library: ⟨https://github.com/kennytm/qrcode-rust⟩

       • Cloudflare Worker : ⟨https://github.com/cloudflare/workers-rs⟩

       • Alternate Web Server: ⟨https://github.com/tokio-rs/axum⟩

       NOTE:  Only  the  direct dependencies for the core logic are listed here Please contact the
       project maintainer if you are missing from the list.

   RELATED LINKS
       • Project Repository: ⟨https://github.com/sayanarijit/qrcode.show⟩

       • Project Maintainer: ⟨https://arijitbasu.in⟩

   COPYRIGHT
       © Arijit Basu 2026

                                                                                    QRcode.show(1)
