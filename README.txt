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
       Accept             output       application/octet-stream,  text/plain,  text/html,
                          type         image/svg+xml, image/png, image/jpeg
       X-QR-Width         default
                          width
       X-QR-Height        default
                          height
       X-QR-Min-Width     minimum
                          width
       X-QR-Min-Height    minimum
                          height
       X-QR-Max-Width     maximum
                          width
       X-QR-Max-Height    maximum
                          height
       X-QR-Dark-Color    dark color   rrggbb
                          (hex)
       X-QR-Light-Color   light        rrggbb
                          color
                          (hex)
       X-QR-Ver‐          QR version   normal, micro
       sion-Type          type
       X-QR-Ver‐          QR version   1 -> 40 for normal, 1 -> 4 for micro.
       sion-Number        number
       X-QR-EC-Level      error        L, M, Q, H
                          checking
                          level
       X-QR-Quiet-Zone    add  quiet   true/ false
                          zone

   PARAMETER EXAMPLES
              curl qrcode.show/INPUT -H "Accept: image/svg+xml"

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
                  curl -d "${input}" https://qrcode.show -H "Accept: image/svg+xml"
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

       • Supports Accept header to control the output format.

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
