<script setup>
const props = defineProps(["recipe"]);
</script>

<template>
  <div class="recipe-card">
    <h2>{{ recipe.name }}</h2>
    <p>Family: {{ recipe.family }}</p>
    <p>Order: {{ recipe.order }}</p>
    <p>Genus: {{ recipe.genus }}</p>
    <br>
    <p>Nutritional Facts (100g):</p>
    <ul>
      <li>Calories: {{ recipe.nutritions.calories }}</li>
      <li>Fat: {{ recipe.nutritions.fat }}</li>
      <li>Sugar: {{ recipe.nutritions.sugar }}</li>
      <li>Carbohydrates: {{ recipe.nutritions.carbohydrates }}</li>
      <li>Protein: {{ recipe.nutritions.protein }}</li>
    </ul>
  </div>
</template>

<style scoped>
.recipe-card {
  display: flex;
  flex-direction: column;
  align-items: start;

  background: rgba(128, 128, 128, 0.4);
  padding: 20px;
  border-radius: 12px;

  h2 {
    font-size: 2rem;
    padding-bottom: 1rem;
  }

  p {
    font-weight: 600;
  }

  ul {
    margin-left: 25px;

    li {
      text-align: start;
    }
  }
}

@media (max-width: 700px) {
 .recipe-card {
   align-items: center;
 }
}
</style>
