<template>
  <div class="container">
    <div class="row">
      <h3>Fertilizer Calc</h3>
    </div>
    <div v-for="error in errors" :key="error" class="text-danger">{{error}}</div>
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
      <div class="input-group mr-2">
        <button class="btn btn-primary text-white" v-on:click="onAdd">Add</button>
      </div>
      <div v-if="density !== 0">
        density: {{ density.toFixed(2) }} lbs/gal
      </div>
    </form>

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

    <div class="row" v-if="Object.keys(resultPercents).length > 0">
      <h3>Result</h3>
      <table class="table">
        <thead>
          <th scope="col">Ingredient</th>
          <th scope="col">Percent</th>
        </thead>
        <tbody>
          <tr v-for="[ingredient, percent] in Object.entries(resultPercents)" :key="ingredient">
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

      <h3>Nutrient Concentrations</h3>
      <table class="table">
        <thead>
          <th scope="col">Nutrient</th>
          <th scope="col">Percent</th>
        </thead>
        <tbody>
          <tr v-for="[nutrient, percent] in Object.entries(concentrations)" :key="nutrient">
            <td scope="row">
                {{nutrient}}
            </td>
            <td>{{percent.mul(100).toFixed(2)}}%</td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>

<script>
import Big from 'big.js'
import recipes from '../recipes.json'
import nutrientConcentrations from '../nutrient-concentrations.json'
import ingredientDensities from '../ingredient-densities.json'

export default {
  name: 'app',
  data: function () {
    return {
      recipes: {},
      recipeComponents: [],
      chosenProduct: null,
      chosenPercent: null,
      resultPercents: {},
      totalResultPercent: Big(0),
      concentrations: {},
      density: 0,
      errors: [],
    }
  },
  mounted: function () {
    this.recipes = this.convertRecipes(recipes);
  },
  methods: {
    convertRecipes: function(recipes) {
      const newRecipes = {}
      for (const [recipeName, recipe] of Object.entries(recipes)) {
        for (const [ingredientName, percentStr] of Object.entries(recipe)) {
          if (!(recipeName in newRecipes)) {
            newRecipes[recipeName] = {}
          }
          try {
            newRecipes[recipeName][ingredientName] = Big(percentStr)
          } catch (error) {
            this.errors.push(`Error while parsing "${percentStr}" for ${ingredientName} in ${recipeName}`);
          }
        }
      }
      return newRecipes
    },
    calc: function() {
      const resultPercents = {}
      for (const [recipe, percent] of this.recipeComponents) {
        for (const [baseIngredient, baseIngredientPercent] of Object.entries(this.recipes[recipe])) {
          const curr = resultPercents[baseIngredient] || Big(0)
          resultPercents[baseIngredient] = curr.add(Big(percent).mul(baseIngredientPercent))
        }
      }
      this.resultPercents = resultPercents;
      if (this.recipeComponents.length > 0) {
        this.totalResultPercent = Object.values(resultPercents).reduce((sum, x) => sum.add(x))
      } else {
        this.totalResultPercent = Big(0);
      }

      const concentrations = {}
      for (const [ingredient, ingredientPercent] of Object.entries(resultPercents)) {
        if (!(ingredient in nutrientConcentrations)) {
          if (ingredient !== "Water") {
            this.errors.push("Couldn't find nutrient concentrations for ingredient " + ingredient)
          }
          continue
        }
        for (const [nutrient, nutrientPercent] of Object.entries(nutrientConcentrations[ingredient])) {
          const curr = concentrations[nutrient] || Big(0)
          concentrations[nutrient] = curr.add(Big(nutrientPercent).mul(ingredientPercent))
        }
      }
      this.concentrations = concentrations

      let density = Big(0);
      for (const [recipe, percent] of this.recipeComponents) {
        density = density.add(Big(ingredientDensities[recipe]).mul(percent))
      }
      this.density = density
    },
    onAdd: function() {
      if (this.chosenProduct && this.chosenPercent) {
        this.clearErrors();
        this.recipeComponents.push([this.chosenProduct, this.chosenPercent * .01])
        this.calc()
        this.chosenProduct = null;
        this.chosenPercent = null;
      }
    },
    onDelete: function(index) {
      this.clearErrors();
      this.recipeComponents.splice(index, 1)
      this.calc()
    },
    clearErrors: function() {
      this.errors = []
    },
  }
}

</script>

<style>
</style>
