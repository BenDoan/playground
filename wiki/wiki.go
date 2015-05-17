package main

import (
	"fmt"
	"github.com/microcosm-cc/bluemonday"
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
	}
}

func GetArticle(w http.ResponseWriter, r *http.Request) {
	title := r.Form.Get("title")
	format := r.Form.Get("format")

	body, err := ioutil.ReadFile("data/" + title)
	if err != nil {
		fmt.Printf("Couldn't find file '%s'", title)
		http.Error(w, err.Error(), 400)
		return
	}

	switch format {
	case "markdown":
		fmt.Fprintf(w, string(body))
	case "html":
		processedBody := processMarkdown(body)
		safe := renderMarkdown(processedBody)
		fmt.Fprintf(w, string(safe))
	default:
		log.Printf("Invalid format type: '%s'", format)
		http.Error(w, err.Error(), 400)
		return
	}
}

func renderMarkdown(body []byte) []byte {
	unsafe := blackfriday.MarkdownCommon(body)
	safe := bluemonday.UGCPolicy().SanitizeBytes(unsafe)

	return safe
}

func processMarkdown(text []byte) []byte {
	// create wiki links
	rp := regexp.MustCompile(`\[\[([a-z]+)\]\]`)
	body_s := rp.ReplaceAll(text, []byte(`<a href="/$1">$1</a>`))

	return body_s
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
