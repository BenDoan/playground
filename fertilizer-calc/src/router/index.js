import Vue from "vue";
import Router from "vue-router";
import Calc from "@/Calc";
import Search from "@/Search";

Vue.use(Router);

export default new Router({
  mode: "history",
  routes: [
    {
      path: "/",
      name: "Calc",
      component: Calc
    },
    {
      path: "/search",
      name: "Search",
      component: Search
    }
  ]
});
