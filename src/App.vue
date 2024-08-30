<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import CategoryCard from './components/CategoryCard.vue';

const categoryData = ref([]);

async function getAllCategories() {
  categoryData.value = await invoke("get_all_categories");
  console.log(categoryData.value);
}

document.addEventListener("DOMContentLoaded", async (e) => {
  await getAllCategories();
});
</script>

<template>
  <div class="container">
    <h1>Welcome to Recipie!</h1>
    <p>Here you will find information on every type of recipe!</p>

    <h2>Categories</h2>
    <div class="categories">
      <CategoryCard v-for="category in categoryData.categories" :key="category.idCategory" :category="category" />
    </div>
  </div>
</template>

<style scoped>

h1 {
  font-size: 4rem;
  margin-bottom: 0.5em;
}

h1 + p {
  font-size: 1.2rem;
  color: rgb(192, 192, 192);
}

h2 {
  text-align: left;
  margin: 2.5rem;
  margin-top: 5rem;
  margin-bottom: 0;
  font-size: 3rem;
}

.categories {
  margin: 25px;
  padding: 25px;
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  grid-gap: 2rem;
}

@media (max-width: 1500px) {
 .categories {
   grid-template-columns: repeat(2, 1fr);
 }
}

@media (max-width: 900px) {
 .categories {
   grid-template-columns: repeat(1, 1fr);
 }
}
</style>
