<script setup lang="ts">
import TheChoice from "@/components/Choice.vue";
import Documentation from "@/components/Documentation.vue";
</script>

<template>
  <!-- style="display: flex; flex-direction: column; justify-content: center; align-items: center" -->
  <div class="flex flex-col content-center items-center">
    <h1 class="text-2xl">Password Management</h1>
    <h3 class="text-lg">Securely split and restore your passwords</h3>
  </div>

  <div class="py-4"></div>

  <TheChoice/>

  <div class="py-8"></div>

  <Documentation/>

</template>
