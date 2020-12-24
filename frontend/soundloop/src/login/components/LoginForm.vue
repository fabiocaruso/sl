<template>
  <div class="form">
		<div ref="inputs" class="inputs">
			<div class="field">
				<label>E-Mail</label>
				<input class="email" v-model="email" name="email" @keyup.enter="validate" placeholder="E-Mail" autocomplete="email" />
			</div>
			<div class="field">
				<label>Password</label>
				<input class="password" v-model="password" name="password" @keyup.enter="validate" placeholder="Password" type="password" autocomplete="current-password" />
			</div>
		</div>
		<div class="error" v-if="error">
			<p>{{ e }}</p>
		</div>
		<div ref="controls" class="controls">
			<button class="btn_register" @click="redirect('/auth/register')">Register</button>
			<button class="btn_login" @click="validate" type="submit">Login</button>
		</div>
  </div>
</template>

<script>
export default {
  name: 'LoginForm',
  props: {
    apiUrl: String
	},
	data() {
		return {
			email: '',
			password: '',
			error: false,
			e: '',
		}
	},
	methods: {
		validateEMail() {
			if (this.email === '') {
				this.e = "Please enter a E-Mail adress!";
			} else if (!this.email.match(`.*@.*`)) {
				this.e = "E-Mail not valid!"
			} else {
				return true;
			}
			return false;
		},
		validatePassword() {
			if (this.password === '') {
				this.e = "Please enter a password!";
			} else {
				return true;
			}
			return false;
		},
		validate() {
			if (this.validateEMail() && this.validatePassword()) {
				this.error = false;
				this.login();
			}
			this.error = true;
		},
		login() {
			let formdata = {
				email: this.email,
				password: this.password,
			}
			fetch('/auth/login', {
				method: 'POST',
				credentials: 'same-origin',
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify(formdata)
			})
				.then(resp => resp.json())
				.then(data => {
					console.log(JSON.stringify(data));
					if (data.result == 'failed') {
						this.e = data.error;
						this.error = true;
					} else {
						this.redirect('/')
					}
				});
		},
		redirect(path, token) {
			if (token === undefined) {
				let url = window.location.origin;
				url += path;
				window.location.href = url;
			}
		},
	},
	mounted() {
	},
}
</script>

<style scoped>
.form {
	display: flex;
	flex-direction: column;
	justify-content: space-between;
}

.inputs {
	display: inline-flex;
	flex-direction: column;
	gap: 10px;
	margin-bottom: 10px;
	overflow: auto;
}

.error > p {
	color: red;
}

.field {
	display: flex;
	flex-direction: column;
	flex-wrap: wrap;
	gap: 5px;
}

.field > label {
	text-align: left;
}

.field > input {
	padding: 8px;
}

.controls {
	display: flex;
	flex-wrap: wrap;
	gap: 5px;
	justify-content: center;
}

.controls > button {
	padding: 5px;
}
</style>
