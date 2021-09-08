package main

import "net/http"

func main() {
	s := []byte("Hello World!")
	http.HandleFunc("/", func(rw http.ResponseWriter, r *http.Request) {
		rw.Write(s)
	})

	if err := http.ListenAndServe("127.0.0.1:8080", nil); err != nil {
		panic(err)
	}
}
