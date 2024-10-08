package main

import (
	"fs"
	"os"
)

func main() {
	// 1 . gather env vars.

	url := os.Getenv("DIR")
	filesDir := os.Getenv("DIR")

	if len(url) == 0 || len(filesDir) == 0 {
		panic("Please specify SERVICE_URL and DIR as env var!")
	}

	f, err := listFiles(filesDir)

	if err != nil {
		panic(err)
	}

	for _, ff := range f {
		go processFile(ff, url)
	}

	// 2. list files in dir
	// 3. call the word splitter goroutine
}

func listFiles(d string) ([]os.DirEntry, error) {
	return fs.ReadDirFS(d)
}

func processFile(f os.DirEntry, url string) {
	println(f)
}
