<script setup lang="ts">
import TheChoice from "@/components/Choice.vue";
</script>

<template>
  <div style="display: flex; flex-direction: column; justify-content: center; align-items: center">
    <h1>Password Management</h1>
    <h3>Securely split and restore your passwords</h3>
  </div>

  <div style="margin: 50px auto"></div>

  <TheChoice/>

  <div style="margin: 100px auto"></div>

  <div class="container" style="display: flex; flex-direction: column; align-items: center; justify-content: center">
    <h2><strong>Documentation</strong></h2>
  </div>

  <div class="container" style="display: flex; flex-direction: column; align-items: center; justify-content: center; max-width: 800px">
    <div style="display: flex; flex-direction: column; max-width: 800px; min-width: 800px">
      <h2>Split Password:</h2>
      <ol>
        <li>Go to "Split" page</li>
        <li>Type your password and notes</li>
        <li>Click on "split" button, which generates qr codes (password shares)</li>
        <li>Save qr codes on you computer: right click on each qr code and then click "Save Image As..."</li>
        <li>Keep Pieces of the secret on different devices</li>
      </ol>
    </div>

    <div style="margin: 15px auto"></div>

    <div class="container" style="display: flex; flex-direction: column; max-width: 800px; min-width: 800px">
      <h2>Restore Password:</h2>
      <ol>
        <li>Gather minimum number of qr images (2 in our case) on your computer</li>
        <li>Go to "Recover" page</li>
        <li>Click on "Choose files"</li>
        <li>Click on "Recover" button, it will recover the password from qr codes and shows in a "password" box</li>
      </ol>
    </div>

    <div style="margin: 30px auto"></div>

    <div class="container" style="display: flex; flex-direction: column; max-width: 800px; min-width: 800px">
      <h2>Security Aspect:</h2>
      <ol>
        <li>
          <p>This site is a simple HTML page that run only on your local computer, so it's completely secure to use it</p>
          <p>There is no any other 3-rd parties involved</p>
        </li>
        <li>
          <a href="https://github.com/meta-secret/meta-secret-node">Open Source</a>
        </li>
      </ol>
    </div>


    <div style="margin: 30px auto"></div>
  </div>

</template>
