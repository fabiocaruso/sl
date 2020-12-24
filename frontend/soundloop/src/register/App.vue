<template>
	<div id="app">
		<div id="content">
			<RegisterForm :apiUrl="apiUrl" class="register"></RegisterForm>
		</div>
	</div>
</template>

<script>
import RegisterForm from './components/RegisterForm.vue'

let url = process.env.VUE_APP_API_URL;
if (url == undefined) {
	url = ""
}

export default {
  name: 'App',
  components: {
		RegisterForm,
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

#content {
	width: 50vw;
	height: 50vh;
	padding: 15px;
	border-radius: 15px;
	background: #d3d3d378;
}

.register {
	width: 100%;
	height: 100%;
	overflow-y: auto;
}
</style>
