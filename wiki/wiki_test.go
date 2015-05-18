package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	"net/http"
	"net/http/httptest"
	"testing"
)

var (
	testArticle = &Article{"testarticle", "This is the body"}
)

func TestAddArticle(t *testing.T) {
	articleJson, _ := json.Marshal(testArticle)
	reader := bytes.NewReader(articleJson)

	req, _ := http.NewRequest("PUT", "/article", reader)
	resp := httptest.NewRecorder()

	HandleArticle(resp, req)

	if resp.Code != http.StatusOK {
		t.Errorf("Article not added")
	}
}

func TestGetArticleMarkdown(t *testing.T) {
	TestAddArticle(t)

	url := fmt.Sprintf("/article?title=%s&format=markdown", testArticle.Title)
	req, _ := http.NewRequest("GET", url, nil)
	resp := httptest.NewRecorder()

	HandleArticle(resp, req)

	if resp.Code != http.StatusOK {
		t.Errorf("Article not found")
	}

	b := fmt.Sprintf("%s", resp.Body)

	if b != testArticle.Body {
		t.Errorf("Wrong output")
	}
}

func TestGetArticleHtml(t *testing.T) {
	TestAddArticle(t)

	url := fmt.Sprintf("/article?title=%s&format=html", testArticle.Title)
	req, _ := http.NewRequest("GET", url, nil)
	resp := httptest.NewRecorder()

	HandleArticle(resp, req)

	if resp.Code != http.StatusOK {
		t.Errorf("Article not found")
	}

	b := fmt.Sprintf("%s", resp.Body)
	expectedOutput := fmt.Sprintf("<p>%s</p>\n", testArticle.Body)
	if b != expectedOutput {
		t.Errorf("Wrong output")
	}
}

func TestGetArticleMissing(t *testing.T) {
	TestAddArticle(t)

	req, _ := http.NewRequest("GET", "/article?title=ello&format=markdown", nil)
	resp := httptest.NewRecorder()

	HandleArticle(resp, req)

	if resp.Code != http.StatusNotFound {
		t.Errorf("Failed to return error code")
	}
}
