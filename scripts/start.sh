#!/bin/bash

ADDRESS=127.0.0.1:7880 ./target/release/webserver \
& ADDRESS=127.0.0.1:7879 IFRAME_ADDRESS=http://127.0.0.1:7880/page?title=baz ./target/release/webserver \
& ADDRESS=127.0.0.1:7878 IFRAME_ADDRESS=http://127.0.0.1:7879/page_with_iframe?title=bar ./target/release/webserver
