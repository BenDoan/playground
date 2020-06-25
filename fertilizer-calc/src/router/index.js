import Vue from "vue";
import Router from "vue-router";
import App from "@/App";
import Search from "@/Search";

Vue.use(Router);

export default new Router({
  mode: "history",
  routes: [
    {
      path: "/",
      name: "App",
      component: App
    },
    {
      path: "/search",
      name: "Search",
      component: Search
    }
  ]
});
