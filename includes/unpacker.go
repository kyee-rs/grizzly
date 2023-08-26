package main

import (
	"archive/tar"
	"bytes"
	"compress/gzip"
	_ "embed"
	"io"
	"log"
	"os"
)

// embed_replace
var tarballData []byte

func main() {
	buf := bytes.NewBuffer(tarballData)

	// Create a new gzip reader to decompress the data
	gzipReader, err := gzip.NewReader(buf)
	if err != nil {
		log.Fatal(err)
	}
	defer func(gzipReader *gzip.Reader) {
		err := gzipReader.Close()
		if err != nil {
			log.Fatalln("Failed to close GZIP_READER")
		}
	}(gzipReader)

	tarReader := tar.NewReader(gzipReader)

	for {
		header, err := tarReader.Next()
		if err == io.EOF {
			break
		}
		if err != nil {
			log.Fatal(err)
		}

		switch header.Typeflag {
		case tar.TypeDir:
			if err := os.Mkdir(header.Name, 0755); err != nil {
				log.Fatal(err)
			}
		case tar.TypeReg:
			data, err := io.ReadAll(tarReader)
			if err != nil {
				log.Fatal(err)
			}
			if err := os.WriteFile(header.Name, data, 0644); err != nil {
				log.Fatal(err)
			}
		default:
			log.Printf("Unknown type: %v in %s", header.Typeflag, header.Name)
		}
	}
}
