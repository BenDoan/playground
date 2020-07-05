<template>
  <div>
    <div class="container">
      <div class="row">
        <h3>Product Identifier Search</h3>
      </div>
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

      <div class="row" v-if="resultingIdentifier">
        <h4>Identifier: {{resultingIdentifier}}</h4>
      </div>
      <div class="row" v-else>
        <h4>Identifier: Not found</h4>
      </div>

      <hr class="row">

      <div class="row">
        Load data: <input type="file" id="file" ref="file" v-on:change="handleDataUpload()"/>
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
    }
  },
  computed: {
    resultingIdentifier() {
      this.chosenProducts.sort();
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
      }
    },
    onDelete(index) {
      this.chosenProducts.splice(index, 1);
      this.chosenProduct = null;
    },
    loadData: function(data) {
      const lines = data.split("\n");
      for (const line of lines) {
        const split = line.split("|");
        if (split.length <= 1) continue

        const ingredients = split.slice(0, split.length - 1)
        ingredients.sort();
        const ident = split[split.length - 1];

        for (const ingredient of ingredients) {
          this.ingredients.push(ingredient);
        }
        this.ingredients = [...new Set(this.ingredients)]

        this.sortedPipedProductsToIdentifier[ingredients.join("|")] = ident;
      }
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
        }

      }
      reader.onerror = error => {
        this.errors.push(`Error while reading file: ${error}`);
      }
      reader.readAsText(file)
    },
  }
}

</script>
