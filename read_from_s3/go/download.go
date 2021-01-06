package main

import (
    "github.com/aws/aws-sdk-go/aws"
    "github.com/aws/aws-sdk-go/aws/credentials"
    "github.com/aws/aws-sdk-go/aws/endpoints"
    "github.com/aws/aws-sdk-go/aws/session"
    "github.com/aws/aws-sdk-go/service/s3"
    "github.com/aws/aws-sdk-go/service/s3/s3manager"

    "fmt"
    "os"
    "path/filepath"
    "strings"
    "time"
)

func main() {
    start := time.Now()
    argsWithoutProg := os.Args[1:]
    if len(argsWithoutProg) < 1 {
        fmt.Println("Please specified the Zarr file to download")
    } else {
        download(argsWithoutProg[0])
    }
    elapsed := time.Since(start)
    fmt.Println("time", elapsed)
}

func download(name string) {
    endpoint := "https://s3.embassy.ebi.ac.uk/"
    bucket := "idr"
    prefix := "zarr/v0.1/" + name
    path := "/tmp/" + name


    sess, _ := session.NewSession(&aws.Config{
        Region: aws.String(endpoints.UsWest1RegionID), Endpoint: aws.String(endpoint),
        Credentials: credentials.AnonymousCredentials,
        S3ForcePathStyle: aws.Bool(true),
        DisableRestProtocolURICleaning: aws.Bool(true)},  
    )
    downloader := s3manager.NewDownloader(sess, func(d *s3manager.Downloader) {
        d.PartSize = 64 * 1024 * 1024 // 64MB per part
        d.Concurrency = 6
    })
    svc := s3.New(sess)
    params := &s3.ListObjectsInput{
        Bucket: aws.String(bucket),
        Prefix: aws.String(prefix),
    }

    resp, _ := svc.ListObjects(params)
    os.MkdirAll(path, os.ModePerm)
    for _, key := range resp.Contents { 
        currS3FilePath := *key.Key
        fmt.Println(currS3FilePath)
        file := createFile(currS3FilePath, path, prefix)
        fmt.Println("Created", file.Name())
        numBytes, err := downloader.Download(file, &s3.GetObjectInput{
            Bucket: aws.String(bucket),
            Key: aws.String(currS3FilePath),
        })
        if err != nil {
          exitErrorf("Unable to download key %q, %v", currS3FilePath, err)
        }
        fmt.Println("Downloaded", file.Name(), numBytes, "bytes")
    }
}

func createFile(filePath string, path string, prefix string) *os.File {
    res := strings.Split(strings.Replace(filePath, prefix+"/", "", 1), "/")
    original := path
    for i := 0; i < len(res)-1; i++ {
        p := filepath.Join(original, res[i])
        err := os.MkdirAll(p, os.ModePerm)
        if err != nil {
            exitErrorf("Unable to create directory %q, %v", p, err)
        }
        original = p
    }
    p := filepath.Join(original, res[len(res)-1])
    file, err := os.Create(p)
    if err != nil {
        exitErrorf("Unable to create file %q, %v", p, err)
    }
    return file
}

func exitErrorf(msg string, args ...interface{}) {
    fmt.Fprintf(os.Stderr, msg+"\n", args...)
    os.Exit(1)
}
