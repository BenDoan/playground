<template>
  <div>
    <div class="container">
      <div class="row">
        <h3>Product Identifier Search</h3>
      </div>
    </div>
    <div class="container" v-if="ingredients.length != 0">
      <form class="form-inline row" v-on:submit.prevent="">
        <div class="form-group mr-2" required>
          <label class="sr-only" for="product-input">Product</label>
          <select v-model="chosenProduct" class="form-control" id="product-input">
            <option :value="null" disabled>Product</option>
            <option v-for="name in ingredients" :key="name">{{name}}</option>
          </select>
        </div>
        <div class="input-group mr-2">
          <button class="btn btn-primary text-white" v-on:click="onAdd">Add</button>
        </div>
      </form>

      <div class="row" v-if="chosenProducts.length > 0">
        <table class="table">
          <thead>
            <th scope="col">Product</th>
          </thead>
          <tbody>
            <tr v-for="(product, index) in chosenProducts" :key="product">
              <td scope="row">
                <span>
                  <a href="#">
                    <span aria-hidden="true" v-on:click="onDelete(index)">&times;</span>
                  </a>
                  {{product}}
                </span>
              </td>
            </tr>
          </tbody>
        </table>
      </div>

      <div v-if="this.chosenProducts.length > 0">
        <div class="row" v-if="resultingIdentifier">
          <h4>Identifier: <span class="green">{{resultingIdentifier}}</span></h4>
        </div>
        <div class="row" v-else>
          <h4>Identifier: <span class="red">Not found</span></h4>
        </div>
      </div>


      <div class="card card-body bg-light row mt5 mb5" v-if="errors.length !== 0 || messages.length !== 0">
        <div v-for="(error, i) in errors" :key="i" class="text-danger">{{error}}</div>
        <div v-for="(message, i) in messages" :key="i" class="text-success">{{message}}</div>
      </div>

      <hr class="row">

    </div>
    <div class="container">
      <div class="row">
        Load data: <input type="file" id="file" ref="file" @change="handleDataUpload()"/>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  name: 'app',
  data: function () {
    return {
      errors: [],
      ingredients: [],
      chosenProduct: null,
      chosenProducts: [],
      sortedPipedProductsToIdentifier: {},
      messages: [],
    }
  },
  computed: {
    resultingIdentifier() {
      console.log(this.sortedPipedProductsToIdentifier, this.chosenProducts)
      return this.sortedPipedProductsToIdentifier[this.chosenProducts.join("|")]
    }
  },
  created: function () {
    try {
      this.loadData(localStorage.data_string_search);
      this.messages.push(`Loaded data from local storage`);
    } catch (e) {
      1
    }
  },
  methods: {
    onAdd() {
      if (!this.chosenProducts.includes(this.chosenProduct)) {
        this.chosenProducts.push(this.chosenProduct);
        this.chosenProducts.sort();
      }
    },
    onDelete(index) {
      this.chosenProducts.splice(index, 1);
      this.chosenProduct = null;
    },
    loadData: function(data) {
      const totalIngredients = []

      const lines = data.split("\n");
      for (const line of lines) {
        const split = line.split("|");
        if (split.length <= 1) continue

        const ingredients = split.slice(0, split.length - 1)
        ingredients.sort()

        const ident = split[split.length - 1];

        for (const ingredient of ingredients) {
          totalIngredients.push(ingredient);
        }

        this.sortedPipedProductsToIdentifier[ingredients.join("|")] = ident;
      }
      const uniqueIngredients = [...new Set(totalIngredients)]
      uniqueIngredients.sort()
      this.ingredients = uniqueIngredients
    },
    handleDataUpload: function() {
      const file = this.$refs.file.files[0];

      const reader = new FileReader()
      reader.onload = event => {
        try {
          this.loadData(event.target.result);
          localStorage.data_string_search = event.target.result
          this.messages.push("Loaded data");
        } catch (e) {
          this.errors.push(`Invalid JSON data file: ${e}`);
          // eslint-disable-next-line
          console.error(e)
        }

      }
      reader.onerror = error => {
        this.errors.push(`Error while reading file: ${error}`);
        // eslint-disable-next-line
        console.log(error)
      }
      reader.readAsText(file)
    },
  }
}

</script>

<style scoped>
.red {
  color: red;
}

.green {
  color: green;
}

.mt1 {
  margin-top: 1px;
}
.mt2 {
  margin-top: 2px;
}
.mt3 {
  margin-top: 3px;
}
.mt4 {
  margin-top: 4px;
}
.mt5 {
  margin-top: 5px;
}

.mb1 {
  margin-bottom: 1px;
}
.mb2 {
  margin-bottom: 2px;
}
.mb3 {
  margin-bottom: 3px;
}
.mb4 {
  margin-bottom: 4px;
}
.mb5 {
  margin-bottom: 5px;
}
</style>
