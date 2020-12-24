<template>
	<div id="app">
		<div id="content">
			<LoginForm :apiUrl="apiUrl" class="login"></LoginForm>
		</div>
	</div>
</template>

<script>
import LoginForm from './components/LoginForm.vue'

let url = process.env.VUE_APP_API_URL;
if (url == undefined) {
	url = ""
}

export default {
  name: 'App',
  components: {
		LoginForm,
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
	width: max-content;
	height: max-content;
	min-width: 250px;
	padding: 15px;
	border-radius: 15px;
	background: #d3d3d378;
}

.login {
	width: 100%;
	height: 100%;
	overflow-y: auto;
}
</style>
