<script lang="ts">
import { defineComponent } from 'vue'

export default defineComponent({
  methods: {
    testMethod() {
      alert("ts works!")
    }
  }
})

</script>

<template>
  <div>
    <h1>This is an about page</h1>
    <button type="button" @click="testMethod">
      Options
    </button>
  </div>
</template>

<style>

</style>
