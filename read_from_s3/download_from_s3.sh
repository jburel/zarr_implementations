# cd to this directory
# https://stackoverflow.com/a/6393573/2700168
cd "${0%/*}"

go run download.go 6001247.zarr
