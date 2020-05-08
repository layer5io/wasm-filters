package main

import (
	"fmt"
	"net/http"
)

func store(w http.ResponseWriter, req *http.Request) {
	// defer req.Body.Close()
	// data, err := ioutil.ReadAll(req.Body)
	// if err != nil {
	// 	fmt.Printf("err: %v", err)
	// 	return
	// }
	// fmt.Printf("data : %v", string(data))
	w.Write([]byte("store"))
}

func auth(w http.ResponseWriter, req *http.Request) {
	token := req.Header.Get("token")
	if token != "hello" {
		w.Write([]byte("Unauthorized, token recieved: " + token))
	} else {
		w.Write([]byte("Authorized"))
	}
}
func headers(w http.ResponseWriter, req *http.Request) {
	for name, headers := range req.Header {
		for _, h := range headers {
			fmt.Fprintf(w, "%v: %v\n", name, h)
		}
	}
}

func main() {
	http.HandleFunc("/store", store)
	http.HandleFunc("/auth", auth)
	http.ListenAndServe(":8080", nil)
}
