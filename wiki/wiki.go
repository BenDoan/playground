package main

import (
	"encoding/json"
	"fmt"
	"github.com/russross/blackfriday"
	"html/template"
	"io/ioutil"
	"log"
	"net/http"
	//"net/url"
)

var (
	listen   = ":8080"
	articles = make(map[string]map[string]string)
)

var templates = template.Must(template.ParseFiles("templates/base.html"))

func handler(w http.ResponseWriter, r *http.Request) {
	err := templates.ExecuteTemplate(w, "base.html", nil)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
	}
}

func getAllArticles(w http.ResponseWriter, r *http.Request) {
	j, _ := json.Marshal(articles)
	fmt.Fprintf(w, string(j))
}

func Article(w http.ResponseWriter, r *http.Request) {
	r.ParseForm()
	b, _ := ioutil.ReadAll(r.Body)
	fmt.Println(string(b))
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
	body, err := ioutil.ReadFile("data/" + title)
	var _ = body
	if err != nil {
		fmt.Printf("Couldn't find file '%s'", title)
		w.WriteHeader(400)
		return
		// handle error
	}

	fmt.Fprintf(w, string(blackfriday.MarkdownCommon(body)))
}

func CreateArticle(w http.ResponseWriter, r *http.Request) {
	title := r.Form.Get("title")
	body := r.Form.Get("body")

	fmt.Printf("writing %s:%s", title, body)
	ioutil.WriteFile("data/"+title, []byte(body), 0644)
}

func setup() {
	articles["hello"] = map[string]string{
		"title":    "This is an article",
		"contents": "Hello world!",
	}
}

func main() {
	setup()

	http.HandleFunc("/", handler)
	http.HandleFunc("/articles", getAllArticles)
	http.HandleFunc("/article", Article)

	http.Handle("/static/", http.StripPrefix("/static/", http.FileServer(http.Dir("./static/"))))
	http.Handle("/partials/", http.StripPrefix("/partials/", http.FileServer(http.Dir("./partials/"))))

	fmt.Printf("")

	log.Println("Listening on", listen)
	http.ListenAndServe(listen, nil)
}
