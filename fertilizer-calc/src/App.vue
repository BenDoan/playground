<template>
  <div>
    <div class="container">
      <div class="row">
        <h3>Fertilizer Calc</h3>
      </div>
    </div>
    <div class="container" v-if="recipes">
      <form class="form-inline row" v-on:submit.prevent="">
        <div class="form-group mr-2">
          NPK Analysis: <input type="text" class="form-control npk-analysis ml-2 mb-2">
        </div>
      </form>
      <form class="form-inline row" v-on:submit.prevent="onAdd">
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
        <h3>Recipeasda</h3>
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
            <tr>
              <td scope="row"><strong>Total</strong></td>
              <td scope="row"><strong>{{totalRecipePercent.mul(100).toFixed(2)}}%</strong></td>
            </tr>
          </tbody>
        </table>
      </div>

      <div class="row" v-if="hasData">
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
      </div>

      <div class="row" v-if="hasData">
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
      </div>

      <div v-for="(error, i) in errors" :key="i" class="text-danger row">{{error}}</div>
      <div v-for="(message, i) in messages" :key="i" class="text-success row">{{message}}</div>

      <div v-if="hasData && densityLowerRange != 0" class="row">
        <div v-if="densityLowerRange.eq(densityUpperRange)">
          <strong>Density:</strong> {{densityLowerRange.toFixed(2)}} lbs/gal
        </div>
        <div v-else>
          <strong>Density:</strong> {{densityLowerRange.toFixed(2)}}-{{densityUpperRange.toFixed(2)}} lbs/gal
        </div>
      </div>

      <div v-if="showPh" class="row">
        <div v-if="phLowerRange.eq(phUpperRange)">
          <strong>pH:</strong> {{phLowerRange.toFixed(2)}}
        </div>
        <div v-else>
          <strong>pH:</strong> {{phLowerRange.toFixed(2)}}-{{phUpperRange.toFixed(2)}}
        </div>
      </div>

      <div class="row" v-if="showPh">
        <h4>Note: pH calculation assumes:</h4>
      </div>
      <div class="row" v-if="showPh">
        <p class="card card-body bg-light">
          <ol>
            <li>Fertilizer products used to make the blends are both weak acids</li>
            <li>Concentration of protons is equal to concentration of the acid</li>
            <li>Components are simple liquids which are not buffered</li>
            <li>Ionization constants are equal</li>
            <li>There is no neutralization reaction or formation of a salt</li>
          </ol>
        </p>
      </div>
      <hr>
    </div>
    <div class="container">
      <div class="row">
        Load data: <input type="file" id="file" ref="file" v-on:change="handleDataUpload()"/>
      </div>
    </div>
  </div>
</template>

<script>
import Big from 'big.js'

function parse(str) {
  try {
    return JSON.parse(str)
  } catch (e) {
    return JSON.parse(atob(str))
  }
}

export default {
  name: 'app',
  data: function () {
    return {
      recipeComponents: [],
      recipes: null,
      nutrientConcentrations: null,
      ingredientDensities: null,
      ingredientCasNumbers: null,
      nutrientOrder: null,
      ingredientPh: null,

      chosenProduct: null,
      chosenPercent: null,
      resultPercents: {},
      totalResultPercent: Big(0),
      totalRecipePercent: Big(0),
      concentrations: {},
      densityUpperRange: Big(0),
      densityLowerRange: Big(0),
      phUpperRange: Big(0),
      phLowerRange: Big(0),
      phMissing: false,
      errors: [],
      messages: [],
      uploadedDataString: null,
    }
  },
  created: function () {
    try {
      this.loadData(parse(localStorage.data_string));
      this.messages.push(`Loaded data from local storage`);
    } catch (e) {
      1
    }
  },
  computed: {
    hasData() {
      return Object.keys(this.resultPercents).length > 0
    },
    showPh() {
      const percentAwayFrom100 = Math.abs((parseFloat(this.totalResultPercent) * 100) - 100)
      const isRoughly100Percent = percentAwayFrom100 <= 5
      return this.hasData && this.phLowerRange != 0 && !this.phMissing && isRoughly100Percent
    },
  },
  methods: {
    loadData: function(fertilizerData) {
      this.recipes = this.convertRecipes(fertilizerData['product-recipes'])
      this.nutrientConcentrations = fertilizerData['nutrient-concentrations']
      this.ingredientDensities = fertilizerData['ingredient-densities']
      this.ingredientCasNumbers = fertilizerData['ingredient-CASNumbers']
      this.nutrientOrder = fertilizerData['nutrient-order']
      this.ingredientPh = fertilizerData['ingredient-ph']
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
          this.loadData(parse(event.target.result));
          localStorage.data_string = event.target.result
          this.messages.push("Loaded data");
          // eslint-disable-next-line
          console.log("Loaded new data");
        } catch (e) {
          this.errors.push(`Invalid JSON data file: ${e}`);
        }

      }
      reader.onerror = error => {
        this.errors.push(`Error while reading file: ${error}`);
      }
      reader.readAsText(file)
    },
    _calcResultPercents() {
      const resultPercents = {}
      for (const [recipe, percent] of this.recipeComponents) {
        for (const [baseIngredient, baseIngredientPercent] of Object.entries(this.recipes[recipe])) {
          const curr = resultPercents[baseIngredient] || Big(0)
          if (!this.isValidNum(percent)){
            this.errors.push(`Invalid recipe percent for ${recipe}: ${percent}`)
            continue
          }
          resultPercents[baseIngredient] = curr.add(Big(percent).mul(baseIngredientPercent))
        }
      }
      this.resultPercents = resultPercents;

      this.totalRecipePercent = this.recipeComponents.reduce((sum, x) => sum.add(x[1]), Big(0))

      if (this.recipeComponents.length > 0) {
        this.totalResultPercent = Object.values(this.resultPercents).reduce((sum, x) => sum.add(x))
      } else {
        this.totalResultPercent = Big(0)
      }
    },
    _calcNutrientConcentrations() {
      const concentrations = {}
      for (const [ingredient, ingredientPercent] of Object.entries(this.resultPercents)) {
        if (!(ingredient in this.nutrientConcentrations)) {
          continue
        }
        for (const [nutrient, nutrientPercent] of Object.entries(this.nutrientConcentrations[ingredient])) {
          const curr = concentrations[nutrient] || Big(0)
          if (!this.isValidNum(nutrientPercent)){
            this.errors.push(`Invalid nutrient percent for ${ingredient} ${nutrient}: ${nutrientPercent}`)
            continue
          }
          concentrations[nutrient] = curr.add(Big(nutrientPercent).mul(ingredientPercent))
        }
      }
      this.concentrations = concentrations
    },
    _calcDensities() {
      let densityLowerRange = Big(0);
      let densityUpperRange = Big(0);
      for (const [recipe, percent] of this.recipeComponents) {
        if (this.ingredientDensities[recipe]) {
          let thisDensityLower;
          let thisDensityUpper;
          if (this.ingredientDensities[recipe].includes("-")) {
            const [lower, upper] = this.ingredientDensities[recipe].split("-");

            if (!this.isValidNum(lower)){
              this.errors.push(`Invalid density number for ${recipe}: ${lower}`)
              continue
            }
            if (!this.isValidNum(upper)){
              this.errors.push(`Invalid density number for ${recipe}: ${upper}`)
              continue
            }

            thisDensityLower = Big(lower);
            thisDensityUpper = Big(upper);
          } else {
            const num = this.ingredientDensities[recipe]
            if (!this.isValidNum(num)) {
              this.errors.push(`Invalid density number for ${recipe}: ${num}`)
              continue
            }

            thisDensityLower = thisDensityUpper = Big(num)
          }
          densityLowerRange = densityLowerRange.add(thisDensityLower.mul(percent))
          densityUpperRange = densityUpperRange.add(thisDensityUpper.mul(percent))
        } else {
          this.errors.push("Couldn't find density for " + recipe)
        }
      }
      this.densityUpperRange = densityUpperRange
      this.densityLowerRange = densityLowerRange
    },
    _calcPh() {
        let phLowerRange = Big(0);
        let phUpperRange = Big(0);
        for (const [recipe, percent] of this.recipeComponents) {
          if (this.ingredientPh[recipe]) {
            let thisPhLower;
            let thisPhUpper;
            if (this.ingredientPh[recipe].includes("-")) {
              const [lower, upper] = this.ingredientPh[recipe].split("-");

              if (!this.isValidNum(lower)){
                this.errors.push(`Invalid pH value for ${recipe}: ${lower}, remove this product to see pH calculation`)
                this.phMissing = true
                return
              }
              if (!this.isValidNum(upper)){
                this.errors.push(`Invalid pH value for ${recipe}: ${upper}, remove this product to see pH calculation`)
                this.phMissing = true
                return
              }

              thisPhLower = Big(lower);
              thisPhUpper = Big(upper);
            } else {
              const num = this.ingredientPh[recipe]
              if (!this.isValidNum(num)) {
                this.errors.push(`Invalid pH number for ${recipe}: ${num}, remove this product to see pH calculation`)
                this.phMissing = true
                return
              }

              thisPhLower = thisPhUpper = Big(num)
            }
            phLowerRange = phLowerRange.add(Big(percent).mul(Math.pow(10, -thisPhLower)))
            phUpperRange = phUpperRange.add(Big(percent).mul(Math.pow(10, -thisPhUpper)))
          } else {
            this.phMissing = true
            this.errors.push("pH cannot be calculated")
            return
          }
        }
        if (!phUpperRange.eq(0) && !phLowerRange.eq(0)) {
          this.phUpperRange = Big(-Math.log10(parseFloat(phUpperRange)))
          this.phLowerRange = Big(-Math.log10(parseFloat(phLowerRange)))
        }
    },
    calc: function() {
      this._calcResultPercents()

      if (this.nutrientConcentrations) {
        this._calcNutrientConcentrations()
      }

      if (this.ingredientDensities) {
        this._calcDensities()
      }

      if (this.ingredientPh) {
        this._calcPh()
      }
    },
    onAdd: function() {
      if (this.chosenProduct && this.chosenPercent) {
        this.clearErrors();
        this.recipeComponents.push([this.chosenProduct, this.chosenPercent * .01])
        this.calc()
        this.chosenProduct = null;
        this.chosenPercent = null;
        this.phMissing = false;
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
    isValidNum: function(s) {
      try {
        Big(s)
        return true
      } catch (e) {
        return false
      }
    }
  }
}

</script>

<style scoped>
.npk-analysis.npk-analysis {
  width: 350px;
}
</style>
