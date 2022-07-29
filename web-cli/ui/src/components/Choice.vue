<script setup lang="ts">

</script>

<style>
.btn-trans {
  background: transparent;
  color: #636c6c;
  font-weight: 700;
  border: 3px solid #ccc;
  border-radius: 30px;
  font-size: 16px;
}

.btn-group-lg > .btn, .btn-lg {
  padding: 10px 16px;
  font-size: 18px;
  line-height: 1.3333333;
  border-radius: 6px;
}

.center {
  display: flex;
  justify-content: center;
  margin: 0 auto;
}
</style>

<template>
  <div class="center" style="justify-content: space-evenly;max-width: 600px;margin: 0 auto;">
    <RouterLink class="btn-lg btn-trans" to="/split">Split</RouterLink>
    <RouterLink class="btn-lg btn-trans" to="/recover">Recover</RouterLink>
  </div>
</template>
