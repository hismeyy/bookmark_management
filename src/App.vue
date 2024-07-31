<template>
  <div class="container">
    <Sidebar />
    <div class="box">
      <Nav />
      <main class="content">
        <router-view></router-view>
      </main>
    </div>
  </div>
</template>

<script setup>
import Nav from './components/Nav.vue';
import Sidebar from './components/Sidebar.vue';
</script>

<style scoped>
.container {
  display: flex;
  height: 100vh;
  overflow: hidden;
}

.box {
  flex: 1;
  background-color: #fff;
}

.content {
  width: 100%;
  height: 100vh;
}
</style>
