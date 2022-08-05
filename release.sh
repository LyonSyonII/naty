cd naty_common
cargo release $1 -x

cd ../naty_nativefy
cargo release $1 -x &

cd ../naty_app
cargo release $1 -x &

cd ../
cargo release $1 -x