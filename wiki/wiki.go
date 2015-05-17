package main

import (
	"fmt"
	"github.com/russross/blackfriday"
	"html/template"
	"io/ioutil"
	"log"
	"net/http"
	"regexp"
)

var (
	listen = ":8080"
)

var templates = template.Must(template.ParseFiles("templates/base.html"))

func handler(w http.ResponseWriter, r *http.Request) {
	err := templates.ExecuteTemplate(w, "base.html", nil)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
	}
}

func Article(w http.ResponseWriter, r *http.Request) {
	r.ParseForm()
	switch r.Method {
	case "GET":
		GetArticle(w, r)
	case "PUT":
		CreateArticle(w, r)
	case "POST":
		CreateArticle(w, r)
	}
}

func GetArticle(w http.ResponseWriter, r *http.Request) {
	title := r.Form.Get("title")
	format := r.Form.Get("format")

	body, err := ioutil.ReadFile("data/" + title)
	if err != nil {
		fmt.Printf("Couldn't find file '%s'", title)
		w.WriteHeader(500)
		return
		// handle error
	}

	switch format {
	case "markdown":
		fmt.Fprintf(w, string(body))
	case "html":
		rp := regexp.MustCompile(`\[\[([a-z]+)\]\]`)
		body_s := rp.ReplaceAllString(string(body), `<a href="/$1">$1</a>`)

		fmt.Fprintf(w, string(blackfriday.MarkdownCommon([]byte(body_s))))
	default:
		log.Printf("Invalid format type: '%s'", format)
	}
}

func CreateArticle(w http.ResponseWriter, r *http.Request) {
	title := r.Form.Get("title")
	body := r.Form.Get("body")

	ioutil.WriteFile("data/"+title, []byte(body), 0644)
}

func main() {
	http.HandleFunc("/", handler)
	http.HandleFunc("/article", Article)

	http.Handle("/static/", http.StripPrefix("/static/", http.FileServer(http.Dir("./static/"))))
	http.Handle("/partials/", http.StripPrefix("/partials/", http.FileServer(http.Dir("./partials/"))))

	log.Println("Listening on", listen)
	http.ListenAndServe(listen, nil)
}
