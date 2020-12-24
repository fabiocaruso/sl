<template>
  <div class="form">
		<div ref="inputs" class="inputs">
			<div class="field">
				<label>Name</label>
				<input class="name" v-model="name" name="name" @keyup.enter="validate" placeholder="Name" autocomplete="family-name" />
			</div>
			<div class="field">
				<label>First Name</label>
				<input class="first_name" v-model="firstName" name="firstName" @keyup.enter="validate" placeholder="Fist Name" autocomplete="given-name" />
			</div>
			<div class="field">
				<label>E-Mail</label>
				<input class="email" v-model="email" name="email" @keyup.enter="validate" placeholder="E-Mail" autocomplete="email" />
			</div>
			<div class="field">
				<label>Password</label>
				<input class="password" v-model="password" name="password" @keyup.enter="validate" placeholder="Password" type="password" autocomplete="new-password" />
			</div>
			<div class="field">
				<label>Reenter Password</label>
				<input class="rpassword" v-model="rpassword" name="rpassword" @keyup.enter="validate" placeholder="Reenter Password" type="password" autocomplete="new-password" />
			</div>
		</div>
		<div class="error" v-if="error">
			<p>{{ e }}</p>
		</div>
		<div ref="controls" class="controls">
			<button class="btn_register" @click="validate" type="submit">Register</button>
			<button class="btn_login" @click="redirect('/auth/login')">Login</button>
		</div>
  </div>
</template>

<script>
export default {
  name: 'RegisterForm',
  props: {
    apiUrl: String
	},
	data() {
		return {
			name: '',
			firstName: '',
			email: '',
			password: '',
			rpassword: '',
			error: true,
			e: ''
		}
	},
	methods: {
		validateName() {
			if (this.name === '') {
				this.e = "Empty name is not allowed!";
			} else {
				return true;
			}
			return false;
		},
		validateFirstName() {
			if (this.firstName === '') {
				this.e = "Empty first name is not allowed!";
			} else {
				return true;
			}
			return false;
		},
		validateEMail() {
			if (this.email === '') {
				this.e = "Empty E-Mail is not allowed!";
			} else if (!this.email.match(`.*@.*`)) {
				this.e = 'E-Mail not valid!';
			} else {
				return true;
			}
			return false;
		},
		validatePassword() {
			if (this.password === '' && this.rpassword === '') {
				this.e = "Empty password is not allowed!";
			} else if (this.password !== this.rpassword) {
				this.e = 'The passwords do not match!';
			} else if (!this.password.match('^.{8}.*$$')) {
				this.e = 'The password needs to be at least 8 chars long!';
			} else {
				return true;
			}
			return false;
		},
		validate() {
			if (this.validateName() && this.validateFirstName() && this.validateEMail() && this.validatePassword()) {
				this.error = false;
				this.register();
			}
			this.error = true;
		},
		register() {
			let formdata = {
				name: this.name,
				first_name: this.firstName,
				email: this.email,
				password: this.password,
			}
			fetch('/auth/register', {
				method: 'PUT',
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
						this.redirect('/auth/login')
					}
				});
		},
		redirect(path) {
			window.location = path;
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
