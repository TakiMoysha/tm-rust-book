# !/bin/bash

case $1 in
    enable_linking_feature)
        #* not work on windows
        cargo run --features bevy/dynamic
        ;;
    *)
        ;;
esac