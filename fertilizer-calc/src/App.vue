<template>
  <div class="container">
    <div class="row">
      <h3>Fertilizer Calc</h3>
    </div>
    <form class="form-inline" v-on:submit.prevent="onAdd">
      <div class="form-group mr-2" required>
        <label class="sr-only" for="product-input">Product</label>
        <select v-model="chosenProduct" class="form-control" id="product-input">
          <option :value="null" disabled>Product</option>
          <option v-for="name in Object.keys(recipes)" :key="name">{{name}}</option>
        </select>
      </div>
      <div class="input-group mr-2">
        <label class="sr-only" for="percent-input">Percent</label>
        <input v-model="chosenPercent" class="form-control" type="number" min="1" max="100" step="0.01" placeholder="Percent" id="percent-input"/>
        <div class="input-group-append">
          <span class="input-group-text">%</span>
        </div>
      </div>
      <a class="btn btn-primary text-white" v-on:click="onAdd">Add</a>
    </form>

    <hr>

    <div class="row" v-if="recipeComponents.length > 0">
      <h3>Recipe</h3>
      <table class="table">
        <thead>
          <th scope="col">Product</th>
          <th scope="col">Percent</th>
        </thead>
        <tbody>
          <tr v-for="(component, index) in recipeComponents" :key="index">
            <td scope="row">
              <span>
                <a href="#">
                  <span aria-hidden="true" v-on:click="onDelete(index)">&times;</span>
                </a>
                {{component[0]}}
              </span>
            </td>
            <td>{{component[1]*100}}%</td>
          </tr>
        </tbody>
      </table>
    </div>

    <div class="row" v-if="resultPercents">
      <h3>Result</h3>
      <table class="table">
        <thead>
          <th scope="col">Ingredient</th>
          <th scope="col">Percent</th>
        </thead>
        <tbody>
          <tr v-for="([ingredient, percent], index) in Object.entries(resultPercents)" :key="index">
            <td scope="row">
                {{ingredient}}
            </td>
            <td>{{(percent * 100).toFixed(2)}}%</td>
          </tr>
          <tr>
            <td scope="row"><strong>Total</strong></td>
            <td scope="row"><strong>{{totalResultPercent.mul(100).toFixed(2)}}%</strong></td>
          </tr>
        </tbody>
      </table>
    </div>

  </div>

</template>

<script>
import Big from 'big.js'
import recipes from '../recipes.json'

export default {
  name: 'app',
  data: function () {
    return {
      recipes: this.convertRecipes(recipes),
      recipeComponents: [],
      chosenProduct: null,
      chosenPercent: null,
      resultPercents: null,
      totalResultPercent: 0,
    }
  },
  methods: {
    convertRecipes(recipes) {
      const newRecipes = {}
      for (const [recipeName, recipe] of Object.entries(recipes)) {
        for (const [ingredientName, percentStr] of Object.entries(recipe)) {
          if (!(recipeName in newRecipes)) {
            newRecipes[recipeName] = {}
          }
          newRecipes[recipeName][ingredientName] = Big(percentStr)
        }
      }
      return newRecipes
    },
    calc() {
      const resultPercents = {}
      for (const [recipe, percent] of this.recipeComponents) {
        for (const [baseIngredient, baseIngredientPercent] of Object.entries(this.recipes[recipe])) {
          const curr = resultPercents[baseIngredient] || Big(0)
          resultPercents[baseIngredient] = curr.add(Big(percent).mul(baseIngredientPercent))
        }
      }
      this.resultPercents = resultPercents;

      this.totalResultPercent = Object.values(resultPercents).reduce((sum, x) => sum.add(x))
    },
    onAdd: function() {
      if (this.chosenProduct && this.chosenPercent) {
        this.recipeComponents.push([this.chosenProduct, this.chosenPercent * .01])
        this.calc()
        this.chosenProduct = null;
        this.chosenPercent = null;
      }
    },
    onDelete: function(index) {
      this.recipeComponents.splice(index, 1)
      this.calc()
    },
  }
}

</script>

<style>
</style>
