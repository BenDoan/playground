package main

import (
	"fmt"
	"html/template"
	"log"
	"net/http"
)

var listen = ":8080"

var templates = template.Must(template.ParseFiles("templates/base.html"))

func handler(w http.ResponseWriter, r *http.Request) {
	err := templates.ExecuteTemplate(w, "base.html", nil)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
	}
}

func getAllArticles(w http.ResponseWriter, r *http.Request) {
	fmt.Fprintf(w, `["article1", "article2", "article3"]`)
}

func main() {
	http.HandleFunc("/", handler)
	http.HandleFunc("/articles", getAllArticles)
	http.Handle("/static/", http.StripPrefix("/static/", http.FileServer(http.Dir("./static/"))))
	http.Handle("/partials/", http.StripPrefix("/partials/", http.FileServer(http.Dir("./partials/"))))

	fmt.Printf("")

	log.Println("Listening on", listen)
	http.ListenAndServe(listen, nil)
}
