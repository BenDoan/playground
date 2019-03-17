<template>
  <div class="container">
    <div class="row">
      <h3>Fertilizer Calc</h3>
    </div>
    <div v-for="error in errors" :key="error" class="text-danger">{{error}}</div>
    <div v-for="message in messages" :key="message" class="text-success">{{message}}</div>
    <form class="form-inline" v-on:submit.prevent="">
      <div class="form-group mr-2">
        NPK Analysis: <input type="text" class="form-control ml-2 mb-2">
      </div>
    </form>
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
            <td>{{(component[1]*100).toFixed(2)}}%</td>
          </tr>
        </tbody>
      </table>
    </div>

    <div class="row" v-if="Object.keys(resultPercents).length > 0">
      <h3>Result</h3>
      <table class="table">
        <thead>
          <th scope="col">Ingredient</th>
          <th scope="col">CAS Number</th>
          <th scope="col">Percent</th>
        </thead>
        <tbody>
          <tr v-for="[ingredient, percent] in Object.entries(resultPercents)" :key="ingredient">
            <td scope="row">
                {{ingredient}}
            </td>
            <td>{{ingredientCasNumbers[ingredient]}}</td>
            <td>{{(percent * 100).toFixed(2)}}%</td>
          </tr>
          <tr>
            <td scope="row"><strong>Total</strong></td>
            <td scope="row"></td>
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
          <tr v-for="nutrient in nutrientOrder" :key="nutrient">
            <td scope="row" v-if="concentrations[nutrient]">{{nutrient}}</td>
            <td scope="row" v-if="concentrations[nutrient]">
              {{concentrations[nutrient].mul(100).toFixed(2)}}%
            </td>
          </tr>
        </tbody>
      </table>

      <div v-if="density !== 0">
        Density: {{ density.toFixed(2) }} lbs/gal
      </div>
    </div>
    <hr>
    <div v-if="allowUpload">
      Load data file: <input type="file" id="file" ref="file" v-on:change="handleDataUpload()"/>
    </div>
  </div>
</template>

<script>
import Big from 'big.js'
import fertilizerData from '../fertilizer-data.json'

const initRecipes = fertilizerData['product-recipes']
const initNutrientConcentrations = fertilizerData['nutrient-concentrations']
const initIngredientDensities = fertilizerData['ingredient-densities']
const initIngredientCasNumbers = fertilizerData['ingredient-CASNumbers']
const initNutrientOrder = fertilizerData['nutrient-order']

export default {
  name: 'app',
  data: function () {
    return {
      recipeComponents: [],
      recipes: this.convertRecipes(initRecipes),
      nutrientConcentrations: initNutrientConcentrations,
      ingredientDensities: initIngredientDensities,
      ingredientCasNumbers: initIngredientCasNumbers,
      nutrientOrder: initNutrientOrder,
      chosenProduct: null,
      chosenPercent: null,
      resultPercents: {},
      totalResultPercent: Big(0),
      concentrations: {},
      density: 0,
      errors: [],
      messages: [],
      uploadedDataString: null,
    }
  },
  computed: {
    allowUpload: function() {
      let urlParams = new URLSearchParams(window.location.search);
      return urlParams.has('upload');
    }
  },
  methods: {
    loadData: function(fertilizerData) {
      this.recipes = this.convertRecipes(fertilizerData['product-recipes'])
      this.nutrientConcentrations = fertilizerData['nutrient-concentrations']
      this.ingredientDensities = fertilizerData['ingredient-densities']
      this.ingredientCasNumbers = fertilizerData['ingredient-CASNumbers']
    },
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
    handleDataUpload: function() {
      const file = this.$refs.file.files[0];

      const reader = new FileReader()
      reader.onload = event => {
        try {
          this.loadData(JSON.parse(event.target.result));
          this.messages.push(`Successfuly loaded new data`);
        } catch (e) {
          this.errors.push(`Invalid JSON data file: ${e}`);
        }

      }
      reader.onerror = error => {
        this.errors.push(`Error while reading file: ${error}`);
      }
      reader.readAsText(file)
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
        this.totalResultPercent = Big(0)
      }

      const concentrations = {}
      for (const [ingredient, ingredientPercent] of Object.entries(resultPercents)) {
        if (!(ingredient in this.nutrientConcentrations)) {
          continue
        }
        for (const [nutrient, nutrientPercent] of Object.entries(this.nutrientConcentrations[ingredient])) {
          const curr = concentrations[nutrient] || Big(0)
          concentrations[nutrient] = curr.add(Big(nutrientPercent).mul(ingredientPercent))
        }
      }
      this.concentrations = concentrations

      let density = Big(0);
      for (const [recipe, percent] of this.recipeComponents) {
        if (this.ingredientDensities[recipe]) {
          density = density.add(Big(this.ingredientDensities[recipe]).mul(percent))
        } else {
          this.errors.push("Couldn't find density for " + recipe)
        }
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
      this.messages = []
    },
  }
}

</script>

<style>
</style>
