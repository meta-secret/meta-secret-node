<script lang="js">

import init from "meta-secret-web-cli";

export default {
  methods: {
    recoverPassword() {
      init().then(() => {
        alert("Patience ;)")
      });
    }
  }
}
</script>

<template>
  <h1 align="center">Recover Password</h1>
  <div class="container">
    <div style="display: flex; flex-direction: column; align-items: stretch">
      <input class="submit-button" type="button" id="splitButton" value="Recover" @click="recoverPassword">
    </div>
  </div>
</template>