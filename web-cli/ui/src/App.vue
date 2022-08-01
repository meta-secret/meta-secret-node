<script setup lang="ts">
import NavBar from "@/components/NavBar.vue";
</script>

<template>
  <header>
    <NavBar/>
  </header>

  <div class="py-4"/>

  <div>
    <RouterView />
  </div>

</template>

<style>
@import "@/assets/base.css";

.container {
  display: flex;
  justify-content: flex-start;
  max-width: 1376px;
  margin: 0 auto;
}

</style>
