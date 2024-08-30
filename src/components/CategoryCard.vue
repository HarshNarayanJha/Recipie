<script setup>
const props = defineProps(["category"]);

const maxDesc = 75;
if (props.category.strCategoryDescription.length > maxDesc) {
  props.category.strCategoryDescription = props.category.strCategoryDescription.substr(0, maxDesc) + "...";
}
</script>

<template>
  <div class="category-card">
    <img :src="category.strCategoryThumb" alt="" />
    <h2>{{ category.strCategory }}</h2>
    <p>{{ category.strCategoryDescription }}</p>
  </div>
</template>

<style scoped>
.category-card {
  display: flex;
  flex-direction: column;
  align-items: start;

  background: rgba(128, 128, 128, 0.4);
  background-image: linear-gradient(black, transparent);
  padding: 2em;
  border-radius: 24px;

  max-height: 400px;

  img {
    max-width: 100%;
    align-self: center;
    padding: 1em 3em;
    color: linear-gradient(black, transparent);
  }

  h2 {
    font-size: 2.5rem;
    margin-top: 0.7em;
    margin-bottom: 1em;
  }

  p {
    font-weight: 600;
    text-align: end;
    /* overflow: hidden; */
  }
}

@media (max-width: 700px) {
  .category-card {
    align-items: center;
  }
}
</style>
