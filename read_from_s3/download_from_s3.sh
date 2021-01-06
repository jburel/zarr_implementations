# cd to this directory
# https://stackoverflow.com/a/6393573/2700168
cd "${0%/*}"

go get github.com/aws/aws-sdk-go
go run download.go 6001247.zarr
