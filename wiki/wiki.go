package main

import (
	"encoding/json"
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

func angularHandler(w http.ResponseWriter, r *http.Request) {
	err := templates.ExecuteTemplate(w, "base.html", nil)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
	}
}

func HandleArticle(w http.ResponseWriter, r *http.Request) {
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
		http.Error(w, err.Error(), http.StatusNotFound)
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
	rp := regexp.MustCompile(`\[\[([a-zA-z0-9_]+)\]\]`)
	body_s := rp.ReplaceAll(text, []byte(`<a href="/$1">$1</a>`))

	return body_s
}

type Article struct {
	Title string
	Body  string
}

func CreateArticle(w http.ResponseWriter, r *http.Request) {
	decoder := json.NewDecoder(r.Body)
	var article Article
	err := decoder.Decode(&article)

	if err != nil {
		http.Error(w, err.Error(), 400)
		return
	}

	err = ioutil.WriteFile("data/"+article.Title, []byte(article.Body), 0644)

	if err != nil {
		log.Printf("Error saving file: %s", err)
		http.Error(w, err.Error(), 500)
		return
	}
}

func main() {
	http.HandleFunc("/", angularHandler)
	http.HandleFunc("/article", HandleArticle)

	http.Handle("/static/", http.StripPrefix("/static/", http.FileServer(http.Dir("./static/"))))
	http.Handle("/partials/", http.StripPrefix("/partials/", http.FileServer(http.Dir("./partials/"))))

	log.Println("Listening on", listen)
	http.ListenAndServe(listen, nil)
}
