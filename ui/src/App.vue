<template>
  <Discuss msg="Discuss Demo"/>
</template>

<script>
import Discuss from './components/discuss.vue'

export default {
  name: 'App',
  components: {
    Discuss,
  }
}
</script>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
</style>
