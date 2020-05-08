package main

import (
	"fmt"
	"net/http"
)

func hello(w http.ResponseWriter, req *http.Request) {
	// defer req.Body.Close()
	// data, err := ioutil.ReadAll(req.Body)
	// if err != nil {
	// 	fmt.Printf("err: %v", err)
	// 	return
	// }
	// fmt.Printf("data : %v", string(data))
	w.Write([]byte("hello"))
}

func headers(w http.ResponseWriter, req *http.Request) {
	for name, headers := range req.Header {
		for _, h := range headers {
			fmt.Fprintf(w, "%v: %v\n", name, h)
		}
	}
}

func main() {
	http.HandleFunc("/store", hello)
	http.ListenAndServe(":8080", nil)
}
