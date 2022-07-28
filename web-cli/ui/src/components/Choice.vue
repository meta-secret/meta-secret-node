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
  <div style="margin: 50px auto"></div>
  <div class="center">
    <RouterLink class="btn-lg btn-trans" to="/split">Split</RouterLink>
    <div style="margin-left: 30px; margin-right: 30px"></div>
    <RouterLink class="btn-lg btn-trans" to="/split">Recover</RouterLink>
  </div>
</template>
