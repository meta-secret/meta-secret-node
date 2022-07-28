<template>
  <!-- Shared Secret Card -->
  <div style="display: flex; justify-content: flex-start">
    <div class="box red">
      <h2>My Shared Secret</h2>
      <h3>Site: google.com</h3>
      <p>Description: shared password for google.com</p>

      <p>Custodian:</p>

      <div class="secret-info">
      <ul class="custodian-items">
        <div class="custodian">
          <li class="custodian-item">device: 6b86b273ff34fce19d6b804eff5a3f5747ada4eaa22f1d49c01e52ddb7875b4b</li>
          <button>Request</button>
        </div>
        <div  class="custodian">
          <li class="custodian-item">device: d4735e3a265e16eee03f59718b9b5d03019c07d8b6c51f90da3a666eec13ab35</li>
          <button>Request</button>
        </div>
        <div  class="custodian">
          <li class="custodian-item">device: 4e07408562bedb8b60ce05c1decfe3ad16b72230967de01f640b7e4729b49fce</li>
          <button>Request</button>
        </div>
        <div  class="custodian">
          <li class="custodian-item">device: 4b227777d4dd1fc61c6f884f48641d02b4d121d3fd328cb08b5531fcacdabf8a</li>
          <button>Request</button>
        </div>
      </ul>
      </div>

      <br>

      <div class="secret-info">
        <ul class="custodian-items">
          <li class="custodian-item"><h3>Shards: 5</h3></li>
          <li class="custodian-item"><h3>Threshold: 3</h3></li>
        </ul>


      </div>


    </div>
  </div>
</template>

<style>
.box {
  border-radius: 3px;
  box-shadow: 0 30px 40px -20px hsl(229, 6%, 66%);
  padding: 30px;
  margin: 20px;
  background-color: #3c3c3c2c
}

.red {
  border-top: 1px solid hsl(0, 78%, 62%);
}

.custodian {
  display: flex;
  justify-content: flex-start;
  padding: 5px;
}

.custodian-items {
  align-items: center;
  padding-left: 0;
  background-color: inherit;
}

.custodian-item {
  display: inline-block;
  padding-right: 25px;
}

</style>