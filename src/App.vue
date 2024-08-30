<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import RecipeCard from './components/RecipeCard.vue';

const recipeData = ref([]);

async function getAllRecipies() {
  recipeData.value = await invoke("get_recipies");
}

document.addEventListener("DOMContentLoaded", async (e) => {
  await getAllRecipies();
});
</script>

<template>
  <div class="container">
    <h1>Welcome to Recipie!</h1>
    <p>Here you will find information on every type of recipe!</p>

    <div class="fruits">
      <RecipeCard v-for="recipe in recipeData" :key="recipe.id" :recipe="recipe" />
    </div>
  </div>
</template>

<style scoped>

h1 {
  margin-bottom: 1rem;
}

h1 + p {
  color: rgb(192, 192, 192);
}

.fruits {
  margin: 25px;
  padding: 25px;
  display: grid;
  grid-template-columns: repeat(5, 1fr);
  grid-gap: 2.5rem;
}

@media (max-width: 1500px) {
 .fruits {
   grid-template-columns: repeat(3, 1fr);
 }
}

@media (max-width: 900px) {
 .fruits {
   grid-template-columns: repeat(2, 1fr);
 }
}

@media (max-width: 700px) {
 .fruits {
   grid-template-columns: repeat(1, 1fr);
 }
}
</style>
