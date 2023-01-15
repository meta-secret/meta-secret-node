<script lang="ts">
import {defineComponent} from 'vue'
import RegistrationComponent from "@/components/vault/Registration.vue";

export default defineComponent({
  components: {
    RegistrationComponent
  },

  methods: {
    isNewUser() {
      if (localStorage.userId) {
        if (localStorage.userId !== '') {
          return false;
        }
      }
      return true;
    }
  }
});
</script>

<template>

  <div class="flex justify-center py-6">
    <p class="text-2xl">Distributed Vault</p>
  </div>

  <div v-if="isNewUser()">
    <RegistrationComponent/>
  </div>

  <div v-else>
    <div class="container flex max-w-md py-2 items-stretch">
      <RouterLink
          class="w-1/2 text-center rounded-l-lg px-6 py-3 text-white bg-orange-600 active:bg-orange-800"
          to="/vault/secrets">Secrets
      </RouterLink>

      <RouterLink
          class="w-1/2 text-center rounded-r-lg px-6 py-3 text-dark bg-gray-100 active:bg-gray-300"
          to="/vault/devices">Devices
      </RouterLink>
    </div>

    <div>
      <RouterView/>
    </div>
  </div>

</template>
