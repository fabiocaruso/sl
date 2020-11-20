<template>
	<div id="app">
		<list :apiUrl="apiUrl" class="list"></list>
	</div>
</template>

<script>
import list from './components/List.vue'

let url = process.env.VUE_APP_API_URL;
if (url == undefined) {
	url = ""
}

export default {
  name: 'App',
  components: {
		list,
	},
	data() {
		return {
			apiUrl: url,
		}
	},
}
</script>

<style>
*{
	padding:0;
	margin:0;
}

html, body, #app {
	width: 100%;
	height: 100%;
}

#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
	display: flex;
	align-items: center;
	justify-content: center;
}
</style>
